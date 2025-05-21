use std::env;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use urlencoding::decode;

// Per il download
fn download_file(url: &str, dest_path: &PathBuf, log_file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    let status = resp.status();
    let _ = writeln!(log_file, "🌐 HTTP status: {}", status);

    if !status.is_success() {
        return Err(format!("HTTP error: {}", status).into());
    }

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
    let _ = writeln!(log_file, "🔎 Raw input: {}", args[1]);
    let decoded = decode(raw).unwrap_or_else(|_| raw.into());
    let mut decoded_str = decoded.to_string();

    // Rimuovi trailing slash se rovina il file
    if decoded_str.ends_with(".msg/") {
        decoded_str = decoded_str.trim_end_matches('/').to_string();
        let _ = writeln!(log_file, "✂️ Removed trailing slash: {}", decoded_str);
    }

    // Aggiungi automaticamente https:// se manca
    if !decoded_str.starts_with("http:/") && !decoded_str.starts_with("https:/") {
        decoded_str = format!("https://{}", decoded_str);
    }

    let _ = writeln!(log_file, "📥 Opening: {}", decoded_str);
    println!("📥 Opening: {}", decoded_str);

    let path_to_open = if decoded_str.starts_with("http:/") || decoded_str.starts_with("https:/") {
        let tmp_path = PathBuf::from("C:\\eml_opener\\downloaded_file.msg");

        match download_file(&decoded_str, &tmp_path, &mut log_file) {
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
