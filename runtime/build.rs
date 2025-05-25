use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let proto_file = "proto/runtime.proto";
    
    // Configurar la compilación de protobuf
    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir.join("generated"))
        .compile(&[proto_file], &["proto/"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
    
    // Volver a compilar si cambian los archivos proto
    println!("cargo:rerun-if-changed={}", proto_file);
    
    Ok(())
}
