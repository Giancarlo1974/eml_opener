# 📩 eml_opener

**eml_opener** is a Rust-based utility designed to work with a Chrome extension. 
It allows automatic opening of `.eml` and `.msg` email files directly with Outlook (or any default handler), avoiding manual download and file handling.

---

## 🚀 Features

- Handles `emlopen://...` custom URLs from the browser
- Decodes and downloads `.eml` or `.msg` files
- Opens the file with the default associated program (e.g., Outlook)
- Logs operations in `C:\eml_opener\emlopen_log.txt`

---

## 🧱 How It Works

1. User clicks a `.eml` or `.msg` link in the browser
2. Chrome extension rewrites it to `emlopen://<encoded-URL>`
3. Windows invokes `eml_opener.exe` via protocol handler
4. The app:
   - Decodes the URL
   - Downloads the file
   - Opens it via `start` command
   - Logs the process

---

## 🛠 Requirements

- Windows
- Rust (to compile)
- Outlook or another program associated with `.eml` or `.msg` files

---

## 🔧 Build

```sh
git clone https://github.com/your-username/eml_opener.git
cd eml_opener
cargo build --release
```

Output: `target/release/eml_opener.exe`

---

## 🖥 Register the Protocol in Windows

To associate the `emlopen://` protocol, add this to the Registry (as a `.reg` file):

```reg
Windows Registry Editor Version 5.00

[HKEY_CLASSES_ROOT\emlopen]
@="URL:Open EML"
"URL Protocol"=""

[HKEY_CLASSES_ROOT\emlopen\shell\open\command]
@="\"C:\\eml_opener\\eml_opener.exe\" \"%1\""
```

🧠 Make sure the path to the `.exe` file is correct.

---

## 📝 Logging

Logs are saved to:

```
C:\eml_opener\emlopen_log.txt
```

---

## 🧪 Example

Input from the extension:

```
emlopen://https%3A%2F%2Fexample.com%2Fmail.msg
```

Log output:

```
📥 Opening: https://example.com/mail.msg
✅ File downloaded: C:\eml_opener\downloaded_file.msg
✅ File launched.
```

---

## 📄 License

MIT

---

## 🙋‍♂️ Author

[Giancarlo1974](https://github.com/Giancarlo1974)
