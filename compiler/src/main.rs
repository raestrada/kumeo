use std::fs;
use std::path::PathBuf;
use std::env;

// Importar del crate principal
use kumeo_compiler::{
    LogFormat, init, info, debug, error,
    resource::{ResourceManager, ResourceConfig, ResourceError}
};
use serde::Deserialize;

// CLI struct simplified for testing
struct Cli {
    input: PathBuf,
    #[allow(dead_code)]
    output: Option<PathBuf>,
    debug: bool,
}

#[derive(Debug, Deserialize)]
struct WorkflowConfig {
    name: String,
    source: String,
    target: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determinar el formato de logging basado en el entorno
    let log_format = if env::var("KUMEO_JSON_LOGS").is_ok() {
        LogFormat::Json
    } else {
        LogFormat::Human
    };
    
    // Inicializar el sistema de logging
    let _guard = init("kumeo-compiler", log_format, None);
    
    // Simplified for testing - only using a hardcoded cli
    let cli = Cli {
        input: PathBuf::from("../examples/simple_workflow.kumeo"),
        output: None,
        debug: true,
    };
    
    // Read the input file
    info!("Reading input file: {}", cli.input.display());
    let input = match fs::read_to_string(&cli.input) {
        Ok(content) => content,
        Err(e) => {
            error!(error = %e, path = %cli.input.display(), "Failed to read input file");
            return Err(format!("Failed to read input file: {}", e).into());
        }
    };
    
    // For testing, just print the input and note that lexer testing is done via the test_lexer binary
    if cli.debug {
        debug!("File to parse: {}", cli.input.display());
        debug!("Content length: {} bytes", input.len());
        debug!("\nTo test the lexer, run: cargo run --bin test_lexer");
    }
    
    // Example of using the ResourceManager
    example_resource_usage().await?;
    
    // Parser temporarily disabled during testing
    info!("Parser implementation is in progress");
    
    Ok(())
}

/// Example function demonstrating the usage of ResourceManager
async fn example_resource_usage() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting resource usage example");
    
    // Create a ResourceManager with default configuration
    let resource_manager = ResourceManager::new();
    
    // Example 1: Load a local file
    let local_path = "../examples/simple_workflow.kumeo";
    match resource_manager.load(local_path).await {
        Ok(resource) => {
            info!("Successfully loaded local file: {}", local_path);
            if let Ok(content) = resource.as_str() {
                info!("File content (first 100 chars): {}...", &content[..content.len().min(100)]);
            }
        }
        Err(e) => error!("Failed to load local file: {}", e),
    }
    
    // Example 2: Load a file using file:// scheme
    let file_uri = format!("file://{}", std::env::current_dir()?.join(local_path).display());
    match resource_manager.load(&file_uri).await {
        Ok(_) => info!("Successfully loaded file using file:// scheme: {}", file_uri),
        Err(e) => error!("Failed to load file using file:// scheme: {}", e),
    }
    
    // Note: Uncomment and configure these examples as needed
    
    /*
    // Example 3: Load from HTTP (requires network)
    let http_uri = "https://raw.githubusercontent.com/raestrada/kumeo/main/README.md";
    match resource_manager.load(http_uri).await {
        Ok(resource) => {
            info!("Successfully loaded HTTP resource: {}", http_uri);
            if let Ok(content) = resource.as_str() {
                info!("Content (first 100 chars): {}...", &content[..content.len().min(100)]);
            }
        }
        Err(e) => error!("Failed to load HTTP resource: {}", e),
    }
    
    // Example 4: Load from Git (requires git feature and network)
    #[cfg(feature = "git")]
    {
        let git_uri = "git://github.com/raestrada/kumeo/main/README.md";
        match resource_manager.load(git_uri).await {
            Ok(resource) => {
                info!("Successfully loaded Git resource: {}", git_uri);
                if let Ok(content) = resource.as_str() {
                    info!("Content (first 100 chars): {}...", &content[..content.len().min(100)]);
                }
            }
            Err(e) => error!("Failed to load Git resource: {}", e),
        }
    }
    
    // Example 5: Load from S3 (requires s3 feature and credentials)
    #[cfg(feature = "s3")]
    {
        let s3_uri = "s3://my-bucket/path/to/resource.txt";
        match resource_manager.load(s3_uri).await {
            Ok(resource) => {
                info!("Successfully loaded S3 resource: {}", s3_uri);
                if let Ok(content) = resource.as_str() {
                    info!("Content (first 100 chars): {}...", &content[..content.len().min(100)]);
                }
            }
            Err(e) => error!("Failed to load S3 resource: {}", e),
        }
    }
    */
    
    info!("Resource usage example completed");
    Ok(())
}
