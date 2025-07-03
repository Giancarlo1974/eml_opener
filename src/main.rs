use std::env;
use std::fs::{self, OpenOptions, File, read_dir};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use urlencoding::decode;

// Constants
const DOWNLOAD_DIR: &str = "C:\\eml_opener";
const LOG_FILE_PATH: &str = "C:\\eml_opener\\emlopen_log.txt";
const MAX_FILE_AGE_HOURS: u64 = 1;

// Clean up old files (older than MAX_FILE_AGE_HOURS)
fn clean_old_files(log_file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let _ = writeln!(log_file, "🧹 Cleaning files older than {} hour(s)", MAX_FILE_AGE_HOURS);
    
    let dir = Path::new(DOWNLOAD_DIR);
    if !dir.exists() {
        fs::create_dir_all(dir)?;
        return Ok(());
    }
    
    let now = SystemTime::now();
    let entries = read_dir(dir)?;
    
    let mut cleaned_count = 0;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Skip log file and directories
        if path.is_dir() || path.to_string_lossy().ends_with("emlopen_log.txt") {
            continue;
        }
        
        // Check file age
        if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = now.duration_since(modified) {
                    if duration > Duration::from_secs(MAX_FILE_AGE_HOURS * 3600) {
                        if let Err(e) = fs::remove_file(&path) {
                            let _ = writeln!(log_file, "⚠️ Failed to remove old file {}: {}", path.display(), e);
                        } else {
                            let _ = writeln!(log_file, "🗑️ Removed old file: {}", path.display());
                            cleaned_count += 1;
                        }
                    }
                }
            }
        }
    }
    
    let _ = writeln!(log_file, "✅ Cleanup complete: removed {} file(s)", cleaned_count);
    Ok(())
}

// Generate a unique filename with timestamp
fn generate_unique_filename(url: &str) -> String {
    // Extract file extension from URL (.msg or .eml)
    let extension = if url.to_lowercase().ends_with(".eml") {
        "eml"
    } else {
        // Default to .msg if no recognized extension
        "msg"
    };
    
    // Generate timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();
    
    format!("download_{}_{}.{}", timestamp, extension, extension)
}

// Per il download
fn download_file(url: &str, log_file: &mut File) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    let status = resp.status();
    let _ = writeln!(log_file, "🌐 HTTP status: {}", status);

    if !status.is_success() {
        return Err(format!("HTTP error: {}", status).into());
    }
    
    // Create unique filename based on timestamp and extension
    let filename = generate_unique_filename(url);
    let dest_path = PathBuf::from(format!("{}\\", DOWNLOAD_DIR)).join(filename);
    
    // Ensure download directory exists
    if let Some(parent) = dest_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut out = File::create(&dest_path)?;
    std::io::copy(&mut resp, &mut out)?;
    
    Ok(dest_path)
}

// Log su c:\eml_opener\emlopen_log.txt
fn init_logger() -> File {
    // Ensure directory exists
    if let Some(parent) = Path::new(LOG_FILE_PATH).parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE_PATH)
        .expect("⚠️ Cannot open log file")
}

fn main() {
    let mut log_file = init_logger();
    let _ = writeln!(log_file, "\n--- {} ---", 
                   chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    // Clean up old files first
    if let Err(e) = clean_old_files(&mut log_file) {
        let _ = writeln!(log_file, "⚠️ Cleanup error: {}", e);
    }

    let args: Vec<String> = env::args().collect();

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
    if decoded_str.ends_with(".msg/") || decoded_str.ends_with(".eml/") {
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
        match download_file(&decoded_str, &mut log_file) {
            Ok(path) => {
                let _ = writeln!(log_file, "✅ File downloaded: {}", path.display());
                path
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
