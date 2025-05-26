//! Punto de entrada principal para el compilador de Kumeo.

use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use kumeo_compiler::{
    ast::{self, Agent, Argument, Program},
    codegen,
    error::KumeoError,
    logging::{self, LogFormat},
    parser,
    semantic::SemanticAnalyzer,
};
use tracing::metadata::LevelFilter;

/// Formatos de salida soportados
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    /// Formato legible para humanos
    Human,
    /// Formato JSON
    Json,
    /// Formato YAML
    Yaml,
}

/// Comandos disponibles
#[derive(Debug, Subcommand)]
enum Commands {
    /// Valida la sintaxis y semántica de un archivo Kumeo
    Check {
        /// Archivo de entrada a validar
        #[arg(short, long)]
        input: PathBuf,
        
        /// Formato de salida
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Human)]
        format: OutputFormat,
    },
    
    /// Formatea un archivo Kumeo
    Format {
        /// Archivo de entrada a formatear
        #[arg(short, long)]
        input: PathBuf,
        
        /// Archivo de salida (opcional, si no se especifica se sobrescribe el archivo de entrada)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Verificar formato sin modificar el archivo
        #[arg(long)]
        check: bool,
    },
    
    /// Genera código a partir de un archivo Kumeo
    Generate {
        /// Archivo de entrada
        #[arg(short, long)]
        input: PathBuf,
        
        /// Directorio de salida
        #[arg(short, long, default_value = "./output")]
        output: PathBuf,
        
        /// Validar el archivo antes de generar el código
        #[arg(long, default_value_t = true)]
        validate: bool,
    },
}

/// Opciones de línea de comandos
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Nivel de verbosidad
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    
    /// Formato de los logs
    #[arg(long, default_value = "auto")]
    log_format: String,
    
    /// Comando a ejecutar
    #[command(subcommand)]
    command: Commands,
}

/// Función principal
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Configurar el nivel de log basado en la verbosidad
    if cli.verbose > 0 {
        std::env::set_var("RUST_LOG", "debug");
        if cli.verbose > 1 {
            std::env::set_var("RUST_LOG", "trace");
        }
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    
    // Inicializar el sistema de logging
    let log_format = match cli.log_format.as_str() {
        "json" => LogFormat::Json,
        _ => LogFormat::Human,
    };
    logging::init("kumeo-compiler", log_format, None);
    
    // Ejecutar el comando correspondiente
    match cli.command {
        Commands::Check { input, format } => check_command(&input, format).await,
        Commands::Format { input, output, check } => format_command(&input, output, check).await,
        Commands::Generate { input, output, validate } => generate_command(&input, &output, validate).await,
    }
}

/// Comando para validar un archivo Kumeo
async fn check_command(input: &PathBuf, format: OutputFormat) -> Result<()> {
    // Leer el archivo de entrada
    let content = std::fs::read_to_string(input)
        .with_context(|| format!("No se pudo leer el archivo: {}", input.display()))?;
    
    // Parsear el contenido
    let program = parser::parse(&content)
        .map_err(|e| KumeoError::ParserError {
            line: 0,
            column: 0,
            message: e.to_string(),
        })?;
    
    // Validar el programa
    let mut analyzer = SemanticAnalyzer::new();
    let validation_result = analyzer.analyze_program(&program);
    
    // Mostrar resultados
    match format {
        OutputFormat::Human => {
            match validation_result {
                Ok(_) => {
                    println!("✅ El archivo es válido");
                    Ok(())
                }
                Err(e) => {
                    println!("❌ Se encontraron errores de validación:");
                    for error in e.to_string().lines() {
                        println!("  - {}", error);
                    }
                    Err(anyhow!("Validación fallida"))
                }
            }
        }
        OutputFormat::Json => {
            let errors = if let Err(e) = &validation_result {
                e.to_string().lines().map(|s| s.to_string()).collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let result = serde_json::json!({
                "valid": validation_result.is_ok(),
                "errors": errors
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
            validation_result.map_err(|e| anyhow!(e))
        }
        OutputFormat::Yaml => {
            let errors = if let Err(e) = &validation_result {
                e.to_string().lines().map(|s| s.to_string()).collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let result = serde_yaml::to_string(&serde_json::json!({
                "valid": validation_result.is_ok(),
                "errors": errors
            }))?;
            println!("{}", result);
            validation_result.map_err(|e| anyhow!(e))
        }
    }
}

/// Comando para formatear un archivo Kumeo
async fn format_command(input: &PathBuf, output: Option<PathBuf>, check: bool) -> Result<()> {
    // Leer el archivo de entrada
    let content = std::fs::read_to_string(input)
        .with_context(|| format!("No se pudo leer el archivo: {}", input.display()))?;
    
    // Parsear el contenido
    let program = parser::parse(&content)
        .map_err(|e| KumeoError::ParserError {
            line: 0,
            column: 0,
            message: e.to_string(),
        })?;
    
    // Formatear el programa
    let formatted = format_program(&program);
    
    // Verificar si hay cambios
    if content.trim() == formatted.trim() {
        println!("✅ El archivo ya está correctamente formateado");
        return Ok(());
    }
    
    if check {
        println!("❌ El archivo necesita ser formateado");
        return Err(anyhow!("El archivo necesita ser formateado"));
    }
    
    // Escribir los cambios
    let output_path = output.as_ref().unwrap_or(input);
    std::fs::write(output_path, formatted)
        .with_context(|| format!("No se pudo escribir en el archivo: {}", output_path.display()))?;
    
    println!("✅ Archivo formateado correctamente: {}", output_path.display());
    Ok(())
}

/// Comando para generar código a partir de un archivo Kumeo
async fn generate_command(input: &PathBuf, output: &PathBuf, validate: bool) -> Result<()> {
    // Leer el archivo de entrada
    let content = std::fs::read_to_string(input)
        .with_context(|| format!("No se pudo leer el archivo: {}", input.display()))?;
    
    // Parsear el contenido
    let program = parser::parse(&content)
        .map_err(|e| KumeoError::ParserError {
            line: 0,
            column: 0,
            message: e.to_string(),
        })?;
    
    // Validar el programa si es necesario
    if validate {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze_program(&program)?;
    }
    
    // Crear el directorio de salida si no existe
    if !output.exists() {
        std::fs::create_dir_all(output)
            .with_context(|| format!("No se pudo crear el directorio: {}", output.display()))?;
    }
    
    // Generar el código
    // TODO: Handle multiple workflows or select the first one
    if let Some(workflow) = program.workflows.first() {
        codegen::generate_workflow(workflow, output)?;
    } else {
        return Err(anyhow!("No workflows found in the program"));
    }
    
    println!("✅ Código generado correctamente en: {}", output.display());
    Ok(())
}

/// Formatea un programa en una cadena de texto
fn format_program(program: &Program) -> String {
    let mut result = String::new();
    
    // Formatear workflows
    for workflow in &program.workflows {
        result.push_str(&format!("workflow {} {{\n", workflow.name));
        
        // Agregar fuente si existe
        if let Some(source) = &workflow.source {
            result.push_str(&format!("  source: {}\n", format_source(source)));
        }
        
        // Agregar destino si existe
        if let Some(target) = &workflow.target {
            result.push_str(&format!("  target: {}\n", format_target(target)));
        }
        
        // Agregar agentes
        if !workflow.agents.is_empty() {
            result.push_str("  agents: [\n");
            for agent in &workflow.agents {
                result.push_str(&format!("    {}(\n", format_agent(agent)));
            }
            result.push_str("  ]\n");
        }
        
        result.push_str("}\n\n");
    }
    
    result
}

/// Formatea una fuente de datos
fn format_source(source: &crate::ast::Source) -> String {
    match source {
        crate::ast::Source::NATS(topic, options) => {
            if let Some(opts) = options {
                format!("NATS(\"{}\", {:?})", topic, opts)
            } else {
                format!("NATS(\"{}\")", topic)
            }
        }
    }
}

/// Formatea un destino de datos
fn format_target(target: &crate::ast::Target) -> String {
    match target {
        crate::ast::Target::NATS(topic, options) => {
            if let Some(opts) = options {
                format!("NATS(\"{}\", {:?})", topic, opts)
            } else {
                format!("NATS(\"{}\")", topic)
            }
        }
    }
}

/// Formatea un agente
fn format_agent(agent: &Agent) -> String {
    let mut result = String::new();
    
    // Agregar ID si existe
    if let Some(id) = &agent.id {
        result.push_str(&format!("id: \"{}\",\n", id));
    }
    
    // Agregar configuración específica del agente
    for arg in &agent.config {
        match arg {
            Argument::Named(name, value) => {
                result.push_str(&format!("    {}: {},\n", name, value));
            }
            Argument::Positional(value) => {
                result.push_str(&format!("    {},\n", value));
            }
        }
    }
    
    result.push(')');
    result
}
