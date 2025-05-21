use std::env;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use urlencoding::decode;

// Per il download
fn download_file(url: &str, dest_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    let mut out = File::create(dest_path)?;
    std::io::copy(&mut resp, &mut out)?;
    Ok(())
}

// Log su c:\eml_opener\emlopen_log.txt
fn init_logger() -> File {
    let log_path = "c:\\eml_opener\\emlopen_log.txt";
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("⚠️ Cannot open log file")
}

fn main() {
    let mut log_file = init_logger();

    let args: Vec<String> = env::args().collect();
    let _ = writeln!(log_file, "\n---");

    if args.len() <= 1 {
        let _ = writeln!(log_file, "❌ No file passed.");
        println!("❌ No file passed.");
        return;
    }

    let raw = args[1].trim_start_matches("emlopen://");
    let decoded = decode(raw).unwrap_or_else(|_| raw.into());
    let mut decoded_str = decoded.to_string();

    // Aggiungi automaticamente https://
    if !decoded_str.starts_with("http:/") && !decoded_str.starts_with("https:/") {
        decoded_str = format!("https://{}", decoded_str);
    }

    let _ = writeln!(log_file, "📥 Opening: {}", decoded_str);
    println!("📥 Opening: {}", decoded_str);

    let path_to_open = if decoded_str.starts_with("http:/") || decoded_str.starts_with("https:/") {
        let tmp_path = PathBuf::from("C:\\eml_opener\\downloaded_file.msg");

        match download_file(&decoded_str, &tmp_path) {
            Ok(_) => {
                let _ = writeln!(log_file, "✅ File downloaded: {}", tmp_path.display());
                tmp_path
            }
            Err(e) => {
                let _ = writeln!(log_file, "❌ Download error: {}", e);
                return;
            }
        }
    } else {
        PathBuf::from(decoded_str)
    };

    let result = Command::new("cmd")
        .args(&["/C", "start", "", path_to_open.to_str().unwrap()])
        .spawn();

    match result {
        Ok(_) => {
            let _ = writeln!(log_file, "✅ File launched.");
        }
        Err(e) => {
            let _ = writeln!(log_file, "❌ Launch error: {}", e);
        }
    }
}
