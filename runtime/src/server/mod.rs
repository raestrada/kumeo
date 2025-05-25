//! gRPC server for the runtime

use crate::error::{Result, RuntimeError};
use crate::messaging::Manager as MessagingManager;
use crate::resources::Manager as ResourceManager;
use std::path::PathBuf;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tracing::{info, error};

/// Server that handles incoming connections
pub struct Server {
    socket_path: PathBuf,
    resource_manager: ResourceManager,
    messaging: Option<MessagingManager>,
}

impl Server {
    /// Creates a new server instance
    pub fn new(
        socket_path: PathBuf,
        resource_manager: ResourceManager,
        messaging: Option<MessagingManager>,
    ) -> Self {
        Self {
            socket_path,
            resource_manager,
            messaging,
        }
    }

    /// Starts the server
    pub async fn run(self) -> Result<()> {
        // Remove socket if it already exists
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.socket_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create the Unix socket listener
        let listener = UnixListener::bind(&self.socket_path)
            .map_err(|e| RuntimeError::Io(e))?;

        info!("Server listening on {:?}", self.socket_path);

        // Convertir el listener en un stream
        let incoming = {
            let stream = UnixListenerStream::new(listener);
            stream.map_ok(|unix| {
                let io = tokio_util::codec::Framed::new(
                    unix,
                    tokio_util::codec::LengthDelimitedCodec::new(),
                );
                
                // Aquí podrías implementar la lógica para manejar la conexión
                // y deserializar los mensajes gRPC
                
                // Por ahora, solo registramos la conexión
                info!("New connection");
                
                // Devolver un stream/sink para la conexión
                io
            })
        };

        // Crear el servicio gRPC
        let service = RuntimeServiceServer::new(RuntimeServiceImpl {
            resource_manager: self.resource_manager,
            messaging: self.messaging,
        });

        // Iniciar el servidor
        Server::builder()
            .add_service(service)
            .serve_with_incoming(incoming)
            .await
            .map_err(|e| RuntimeError::Other(format!("Server error: {}", e)))?;

        Ok(())
    }
}

// gRPC service implementation
struct RuntimeServiceImpl {
    resource_manager: ResourceManager,
    messaging: Option<MessagingManager>,
}

#[tonic::async_trait]
impl RuntimeService for RuntimeServiceImpl {
    async fn get_resource(
        &self,
        request: tonic::Request<ResourceRequest>,
    ) -> std::result::Result<tonic::Response<ResourceResponse>, tonic::Status> {
        let req = request.into_inner();
        match self.resource_manager.get(&req.uri).await {
            Ok(data) => Ok(tonic::Response::new(ResourceResponse {
                result: Some(resource_response::Result::Data(data)),
                metadata: Default::default(),
            })),
            Err(e) => Err(tonic::Status::internal(e.to_string())),
        }
    }

    async fn put_resource(
        &self,
        request: tonic::Request<PutResourceRequest>,
    ) -> std::result::Result<tonic::Response<ResourceResponse>, tonic::Status> {
        let req = request.into_inner();
        match self.resource_manager.put(&req.uri, &req.data).await {
            Ok(_) => Ok(tonic::Response::new(ResourceResponse {
                result: Some(resource_response::Result::Data(Vec::new()))),
                metadata: Default::default(),
            })),
            Err(e) => Err(tonic::Status::internal(e.to_string())),
        }
    }

    // Implementar otros métodos del servicio...
}

// Incluir el código generado por tonic-build
include!(concat!(env!("OUT_DIR"), "/kumeo.runtime.rs"));
