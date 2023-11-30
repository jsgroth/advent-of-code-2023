use std::error::Error;
use std::{env, fs};

// Read input filename from arg $1 and read file contents into a String
pub fn read_input() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    let filename = args.next().ok_or("Missing required filename arg")?;

    let contents = fs::read_to_string(&filename)
        .map_err(|err| format!("Error reading file from '{filename}': {err}"))?;
    Ok(contents)
}
