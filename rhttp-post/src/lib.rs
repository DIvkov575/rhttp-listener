use std::error::Error;
use std::process::exit;
use serde::{Deserialize, Serialize};

// const IP: &str = "127.0.0.1";
// const PORT: &str = "3000";

// #[tokio::main]
pub fn send(url: String, file_path: String) -> Result<(), Box<dyn Error>> {
    // let url = IP.to_owned() + ":" + PORT;
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() <= 1 { return Err("Incorrect number of inputs".into()); }
    // let file_path = std::path::Path::new(&args[1]);
    let file_path = std::path::Path::new(&file_path);
    let file_name = file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let file_content = std::fs::read_to_string(file_path)?;
    let metafile_json = format!("{{\"name\": {}, \"contents\": {}}}", file_name, file_content);

    let client = reqwest::Client::new();
    let res = client.post(url)
        .body(metafile_json)
        .send();
        // .await?;

    Ok(())
}


#[derive(Serialize, Deserialize)]
struct MetaFile {
    name: String,
    contents: String,
}
