# Geser
![license](https://img.shields.io/github/license/hex0x13h/Geser)
![language count](https://img.shields.io/github/languages/count/hex0x13h/Geser)
![repo size](https://img.shields.io/github/repo-size/hex0x13h/Geser)

Geser is a Gemini protocol server written in Rust. This project serves Markdown pages and static assets (such as images) from a designated folder, converting Markdown content into Gemini-compliant text. **This is my very first Rust project**, and it showcases Rust’s asynchronous I/O, TLS support with hot reloading, and a modular code structure.

## Features

- **Gemini Protocol Server**  
  Implements the Gemini protocol to securely serve content over TLS.

- **Markdown to Gemini Conversion**  
  Converts Markdown pages into Gemini format using `pulldown-cmark`. Headings, paragraphs, links, images, and inline code are processed and transformed into a Gemini-friendly text output.

- **Static File Serving**  
  Serves static assets (e.g., images) stored in a designated folder.

- **TLS Support with Hot Reload**  
  Uses TLS for secure communication and includes a background task for periodic certificate reloading.

- **Asynchronous I/O**  
  Built on Tokio for non-blocking, high-performance asynchronous operations.

- **Caching**  
  Implements in-memory caching (using DashMap) to reduce disk I/O for frequently accessed content.

- **Modular Structure**  
  The code is organized into multiple modules (`config`, `server`, `tls`, `pages`, `cache`, `util`) to improve maintainability and scalability.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/)
- OpenSSL (for generating certificates)

### Installation

Clone the repository:

```bash
git clone https://github.com/iaaaannn0/Geser.git
cd Geser
# Generate a self-signed certificate for testing (make sure your OpenSSL version supports -addext):
openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -subj "/CN=localhost" -addext "subjectAltName = IP:127.0.0.1"
```
### Configuration
The project uses a configuration file (config.toml) to set parameters such as the listening address, certificate paths, pages directory, and TLS reload interval. An example config.toml is provided:

```
address = "0.0.0.0:1965"
cert_path = "cert.pem"
key_path = "key.pem"
pages_dir = "pages"
tls_reload_interval_secs = 300
```
You can also override these settings using environment variables with the `GEMINI_` prefix.

### Running the Server
Build and run the project in release mode:
```
cargo run --release
```

The server will start and listen on the address specified in your configuration file.

Testing

Use a Gemini client, such as Lagrange or Amfora, to test the server:
```
gemini://127.0.0.1:1965
```

### Project Structure

```
Geser/
├── Cargo.toml
├── config.toml         # Configuration file
├── pages/              # Directory for Markdown pages and static assets
│   ├── index.md
│   └── images/
│       └── example.jpg
└── src/
    ├── main.rs         # Entry point
    ├── config.rs       # Configuration management
    ├── server.rs       # Server and connection handling
    ├── tls.rs          # TLS configuration and hot reload
    ├── pages.rs        # Markdown and static file serving
    ├── cache.rs        # In-memory caching
    └── util.rs         # Utility functions (e.g., path sanitization)
```

### About
Geser is my very first Rust project. I created it as an experiment to learn Rust's capabilities in asynchronous programming, secure network communication, and modular software design. so any feedback, bug reports, or contributions are welcome.

### Contributing
Contributions are welcome! If you have ideas or improvements, please open an issue or submit a pull request on GitHub. For major changes, it’s best to open an issue first to discuss what you’d like to change.

### License
This project is licensed under the MIT License.
