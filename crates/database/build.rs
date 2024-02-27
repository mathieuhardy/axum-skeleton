use std::fs::File;
use std::io::{Read, Write};
use walkdir::WalkDir;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=scripts/sql");

    let mut output = "//! This file contains all SQL requests as variables.\n".to_string();

    for entry in WalkDir::new("scripts/sql").into_iter() {
        // Get SQL files only
        let fs_entry = entry?.clone();
        let extension = fs_entry.path().extension().unwrap_or_default();

        if extension != "sql" {
            continue;
        }

        // Read content
        let mut file = File::open(fs_entry.path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let content = content.trim();

        // Generate unique variable name from path
        let var = fs_entry
            .path()
            .display()
            .to_string()
            .to_uppercase()
            .replace("/", "_")
            .replace("-", "_")
            .replace("SCRIPTS_", "")
            .replace(".SQL", "");

        // Append to output string
        if !output.is_empty() {
            output += "\n";
        }

        output += "/// Undocumented.\n";
        output += &format!("pub const {var}: &str = \"{content}\";\n");
    }

    File::create("src/requests.rs")?.write_all(output.as_bytes())?;

    Ok(())
}
