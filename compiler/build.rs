use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Re-run if the grammar changes
    println!("cargo:rerun-if-changed=src/parser/grammar.pest");
    
    // Generate the parser code
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let generated_file = Path::new(&out_dir).join("parser.rs");
    
    // Ensure the parser module can find the grammar
    fs::copy("src/parser/grammar.pest", Path::new(&out_dir).join("grammar.pest"))
        .expect("Failed to copy grammar file");
    
    // Generate the parser code
    let pest_code = format!(
        r#"
        #[allow(dead_code, non_camel_case_types)]
        #[derive(Parser)]
        #[grammar = "{}/grammar.pest"]
        pub struct KumeoParser;
        "#,
        out_dir.to_str().unwrap().replace('\\', "/"),
    );
    
    fs::write(&generated_file, pest_code).expect("Failed to write generated parser");
}
