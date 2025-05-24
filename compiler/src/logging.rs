use std::env;
use std::io;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::*,
    EnvFilter,
};
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_appender::non_blocking::WorkerGuard;

/// Configuración del formato de logs
#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    /// Formato legible para humanos con colores (para desarrollo)
    Human,
    /// Formato JSON para entornos de producción
    Json,
}

/// Inicializa el sistema de logging
///
/// # Arguments
///
/// * `app_name` - Nombre de la aplicación para incluir en los logs
/// * `format` - Formato de los logs (Human o Json)
/// * `file_path` - Ruta opcional para guardar logs en un archivo
///
/// # Returns
///
/// Devuelve un `WorkerGuard` que debe mantenerse vivo mientras se desea
/// que los logs se escriban a un archivo
pub fn init(app_name: &str, format: LogFormat, file_path: Option<&str>) -> Option<WorkerGuard> {
    // Configura la variable de entorno si no está ya configurada
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    // Integración con crate log
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::from_default_env();
    let mut file_guard = None;

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(ErrorLayer::default());

    match format {
        LogFormat::Human => {
            // Formato humano con colores para terminal
            let fmt_layer = fmt::Layer::default()
                .with_ansi(true)
                .with_target(true)
                .with_span_events(FmtSpan::CLOSE)
                .with_writer(io::stdout);
            
            // Si se solicita salida a archivo, configurarla
            if let Some(path) = file_path {
                let file_appender = tracing_appender::rolling::daily(
                    std::path::Path::new(path).parent().unwrap_or_else(|| std::path::Path::new(".")),
                    std::path::Path::new(path).file_name().unwrap_or_default(),
                );
                let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
                file_guard = Some(guard);
                
                let file_layer = fmt::Layer::default()
                    .with_ansi(false)
                    .with_writer(non_blocking);

                registry
                    .with(fmt_layer)
                    .with(file_layer)
                    .init();
            } else {
                registry
                    .with(fmt_layer)
                    .init();
            }
        },
        LogFormat::Json => {
            // Formato JSON para entornos de producción
            let json_layer = fmt::Layer::default()
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_writer(io::stdout);

            // Si se solicita salida a archivo, configurarla
            if let Some(path) = file_path {
                let file_appender = tracing_appender::rolling::daily(
                    std::path::Path::new(path).parent().unwrap_or_else(|| std::path::Path::new(".")),
                    std::path::Path::new(path).file_name().unwrap_or_default(),
                );
                let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
                file_guard = Some(guard);
                
                let file_layer = fmt::Layer::default()
                    .json()
                    .with_writer(non_blocking);

                registry
                    .with(json_layer)
                    .with(file_layer)
                    .init();
            } else {
                registry
                    .with(json_layer)
                    .init();
            }
        }
    }

    tracing::info!(
        app_name = app_name,
        version = env!("CARGO_PKG_VERSION"),
        "Initialized logging"
    );

    file_guard
}
