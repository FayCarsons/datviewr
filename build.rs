use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=.env");
    let dest_path = "./src/evil_env.rs";
    let mut file = File::create(&dest_path)?;

    dotenv().ok();

    for (key, value) in env::vars() {
        if key.starts_with("EVIL_") {
            let line = format!(
                "pub const {}: &str = \"{}\";\n",
                key,
                value.replace("\"", "\\\"")
            );
            file.write_all(line.as_bytes())?
        }
    }

    Ok(())
}
