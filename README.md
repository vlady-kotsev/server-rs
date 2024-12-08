# 🚀 Rust HTTP Server

A lightweight, multi-threaded HTTP server written in Rust that serves static HTML files with blazing fast performance.

## ✨ Features

- 🔥 Multi-threaded request handling
- 📁 Static file serving
- 🎯 Custom directory support
- 🔄 Automatic index.html routing
- ⚡ Fast response times
- 🛠️ Configurable host and port

## 🚀 Quick Start

```bash
# Build the project
cargo build --release

# Run the server (with default settings)
./target/release/server

# Or run with custom settings
./target/release/server localhost 8000 html

```

## 📖 Usage

```bash
server [host] [port] [directory]
```

### Default Values

- Host: 127.0.0.1
- Port: 8787
- Directory: html

### Example

```bash
./server localhost 3000 public
```

## 🛠️ Requirements
- Rust 1.70 or higher
- Cargo
