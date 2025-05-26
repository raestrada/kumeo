use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use kumeo_compiler::{
    ast::Workflow, 
    error::KumeoError, 
    lexer::Lexer, 
    parser,
    LogFormat, 
    debug, error, info, init, trace,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::EnvFilter;

/// Kumeo Compiler - Compila y valida archivos de definición de flujos de trabajo Kumeo
#[derive(Debug, Parser)]
#[command(name = "kumeoc", version, about, long_about = None)]
struct Cli {
    /// Nivel de verbosidad (puede usarse varias veces para más detalle, ej: -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Formato de salida
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Subcomandos
    #[command(subcommand)]
    command: Commands,
}

/// Comandos disponibles
#[derive(Debug, Subcommand)]
enum Commands {
    /// Valida un archivo Kumeo sin compilar
    Check {
        /// Archivo de entrada (.kumeo)
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Compila un archivo Kumeo a JSON
    Build {
        /// Archivo de entrada (.kumeo)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Archivo de salida (opcional, por defecto se usa el nombre del archivo de entrada con extensión .json)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Validar el workflow después de compilar
        #[arg(long, default_value_t = true)]
        validate: bool,
    },
    
    /// Formatea un archivo Kumeo
    Fmt {
        /// Archivo de entrada (.kumeo)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Archivo de salida (opcional, por defecto sobrescribe el archivo de entrada)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Verificar el formato sin realizar cambios
        #[arg(long)]
        check: bool,
    },
}

/// Formatos de salida soportados
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
enum OutputFormat {
    /// Salida legible para humanos (por defecto)
    #[default]
    Human,
    
    /// Salida en formato JSON
    Json,
    
    /// Salida en formato YAML
    Yaml,
}

/// Resultado de la validación
#[derive(Debug, Serialize, Deserialize)]
struct ValidationResult {
    valid: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parsear argumentos de línea de comandos
    let cli = Cli::parse();
    
    // Configurar logging
    let log_level = match cli.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    
    let log_format = if std::env::var("KUMEO_JSON_LOGS").is_ok() {
        LogFormat::Json
    } else {
        LogFormat::Human
    };
    
    let _guard = init("kumeo-compiler", log_format, Some(log_level));
    
    // Ejecutar el comando correspondiente
    let result = match cli.command {
        Commands::Check { input } => check_command(&input, cli.format).await,
        Commands::Build { input, output, validate } => {
            build_command(&input, output, validate, cli.format).await
        },
        Commands::Fmt { input, output, check } => {
            fmt_command(&input, output, check, cli.format).await
        },
    };
    
    // Manejar errores
    if let Err(e) = result {
        error!(error = %e, "Command failed");
        
        // Mostrar el error en el formato solicitado
        match cli.format {
            OutputFormat::Human => {
                eprintln!("Error: {}", e);
            },
            OutputFormat::Json => {
                let error_json = json!({ "error": e.to_string() });
                println!("{}", serde_json::to_string_pretty(&error_json)?);
            },
            OutputFormat::Yaml => {
                let error_yaml = serde_yaml::to_string(&json!({ "error": e.to_string() }))?;
                println!("{}", error_yaml);
            },
        }
        
        process::exit(1);
    }
    
    Ok(())
}

/// Comando: Validar un archivo Kumeo
async fn check_command(input: &Path, format: OutputFormat) -> Result<()> {
    info!("Validating Kumeo file: {}", input.display());
    
    // Leer el archivo de entrada
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input.display()))?;
    
    // Parsear el contenido
    let workflow = parser::parse(&content)?;
    
    // Validar el workflow
    let validation_result = validate_workflow(&workflow);
    
    // Mostrar resultados
    match format {
        OutputFormat::Human => {
            if validation_result.valid {
                println!("✅ Workflow '{}' is valid", workflow.name);
            } else {
                println!("❌ Workflow '{}' has validation errors:", workflow.name);
                for error in &validation_result.errors {
                    println!("  - {}", error);
                }
            }
            
            if !validation_result.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation_result.warnings {
                    println!("  - {}", warning);
                }
            }
        },
        OutputFormat::Json => {
            let output = json!(validation_result);
            println!("{}", serde_json::to_string_pretty(&output)?);
        },
        OutputFormat::Yaml => {
            let output = serde_yaml::to_string(&validation_result)?;
            println!("{}", output);
        },
    }
    
    if !validation_result.valid {
        return Err(KumeoError::ValidationError("Workflow validation failed".to_string()).into());
    }
    
    Ok(())
}

/// Comando: Compilar un archivo Kumeo
async fn build_command(
    input: &Path,
    output: Option<PathBuf>,
    validate: bool,
    format: OutputFormat,
) -> Result<()> {
    info!("Compiling Kumeo file: {}", input.display());
    
    // Leer el archivo de entrada
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input.display()))?;
    
    // Parsear el contenido
    let workflow = parser::parse(&content)?;
    
    // Validar si es necesario
    if validate {
        let validation_result = validate_workflow(&workflow);
        if !validation_result.valid {
            return Err(KumeoError::ValidationError(
                "Workflow validation failed".to_string(),
            ).into());
        }
    }
    
    // Determinar el archivo de salida
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.to_path_buf();
        path.set_extension("json");
        path
    });
    
    // Serializar a JSON
    let json = match format {
        OutputFormat::Human | OutputFormat::Json => {
            serde_json::to_string_pretty(&workflow)?
        },
        OutputFormat::Yaml => {
            serde_yaml::to_string(&workflow)?
        },
    };
    
    // Escribir el archivo de salida
    fs::write(&output_path, json)
        .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;
    
    info!("Successfully compiled to: {}", output_path.display());
    
    Ok(())
}

/// Comando: Formatear un archivo Kumeo
async fn fmt_command(
    input: &Path,
    output: Option<PathBuf>,
    check: bool,
    format: OutputFormat,
) -> Result<()> {
    if check {
        info!("Checking format of: {}", input.display());
    } else {
        info!("Formatting: {}", input.display());
    }
    
    // Leer el archivo de entrada
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input.display()))?;
    
    // Parsear y volver a formatear
    let workflow = parser::parse(&content)?;
    let formatted = format_workflow(&workflow);
    
    // Verificar si hay cambios
    if content.trim() == formatted.trim() {
        info!("File is already properly formatted");
        return Ok(());
    }
    
    // Si solo estamos verificando, devolver error si hay cambios
    if check {
        return Err(KumeoError::FormatError("File is not properly formatted".to_string()).into());
    }
    
    // Escribir los cambios
    let output_path = output.as_deref().unwrap_or(input);
    fs::write(output_path, formatted)
        .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;
    
    info!("Successfully formatted: {}", output_path.display());
    
    Ok(())
}

/// Validar un workflow
fn validate_workflow(workflow: &Workflow) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    // Validar nombre del workflow
    if workflow.name.trim().is_empty() {
        errors.push("Workflow name cannot be empty".to_string());
    }
    
    // Validar fuente
    if workflow.source.is_none() {
        warnings.push("No source specified".to_string());
    }
    
    // Validar destino
    if workflow.target.is_none() {
        warnings.push("No target specified".to_string());
    }
    
    // Validar agentes
    if workflow.agents.is_empty() {
        warnings.push("No agents defined".to_string());
    }
    
    ValidationResult {
        valid: errors.is_empty(),
        errors,
        warnings,
    }
}

/// Formatear un workflow como string
fn format_workflow(workflow: &Workflow) -> String {
    // TODO: Implementar un formateador bonito para el workflow
    // Por ahora, simplemente usamos serde_json con indentación
    serde_json::to_string_pretty(workflow).unwrap_or_else(|_| "Error formatting workflow".to_string())
}
