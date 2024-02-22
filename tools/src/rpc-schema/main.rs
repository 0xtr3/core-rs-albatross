mod openrpc;
mod parser;

use std::{error::Error, fs::File, io::Read, path::Path};

use crate::openrpc::OpenRpcBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("/Users/stefan/code/nimiq/core-rs-albatross/rpc-interface/src/types.rs");
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let ast = syn::parse_file(&content)?;
    let structs = parser::extract_structs_from_ast(&ast);

    let builder = OpenRpcBuilder::builder()
        .with_components()
        .with_schema(structs.get(1).unwrap())
        .with_schema(structs.get(2).unwrap())
        .with_schema(structs.get(3).unwrap())
        .with_schema(structs.get(4).unwrap())
        .with_schema(structs.get(17).unwrap());
    let definition = builder.build();
    let json_spec =
        serde_json::to_string_pretty(&definition).expect("Failed to serialize OpenRPC spec");
    println!("{}", json_spec);

    Ok(())
}