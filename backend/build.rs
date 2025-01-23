use std::io::Result;

fn main() -> Result<()> {
    tauri_build::build();
    prost_build::compile_protos(&["src/igdb/igdbapi.proto"], &["src/igdb"])?;

    Ok(())
}
