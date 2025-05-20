use std::env;
use std::process::Command;
use urlencoding::decode;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Il protocollo custom passa tutto come un unico argomento, tipo:
    // emlopen://C:/Users/User/Downloads/email.eml
    if args.len() > 1 {
        // Rimuove il protocollo "emlopen://"
        let raw = args[1].trim_start_matches("emlopen://");
        
        // Decodifica eventuali spazi (%20) o caratteri speciali
        let decoded = decode(raw).unwrap_or_else(|_| raw.into());

        println!("Apro file: {}", decoded);

        // Apre con Outlook, oppure con programma associato
        let result = Command::new("cmd")
            .args(&["/C", "start", "", &decoded])
            .spawn();

        if let Err(e) = result {
            eprintln!("Errore apertura file: {}", e);
        }
    } else {
        println!("❌ Nessun file passato.");
    }
}
