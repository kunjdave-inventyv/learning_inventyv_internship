# 🎬 Movie Management API (All-POST Multithreaded Server)

A high-performance multithreaded REST-like API built in Rust that manages a collection of movies stored in a JSON file. All operations (Create, Read, Update, Delete) are executed via POST requests for maximum control, thread safety, and firewall compatibility.

This project demonstrates:
* Thread-safe state management
* JSON file persistence
* Concurrent request handling
* API testing using Postman

---

## 📁 Project Structure

```
movie-api/
│
├── movies.json
├── src/
│   └── main.rs
├── postman_collection.json
└── README.md
```

---

## 📄 movies.json Format

```json
[
  {
    "id": "c3f9d2c3-3333-4ccc-dddd-123456789003",
    "title": "The Matrix",
    "director": "The Wachowskis",
    "year": 1999
  }
]
```


## 🧵 Multithreading Architecture

```
┌─────────────────┐
│ Client Request  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  HTTP Server    │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│ New Thread Spawned      │
└────────┬────────────────┘
         │
         ▼
┌───────────────────────────┐
│ Arc<RwLock<Vec<Movie>>>   │
└────────┬──────────────────┘
         │
         ▼
┌─────────────────┐
│  movies.json    │
└─────────────────┘
```

### Thread Pool Details:

- **Total Worker Threads:** 8
  - **READ Pool:** 5 threads (handles GET operations)
  - **WRITE Pool:** 3 threads (handles POST operations)
  
- **Synchronization:** `Arc<RwLock<Vec<Movie>>>`
  - Multiple concurrent reads allowed
  - Exclusive write access for modifications
  - Automatic data persistence on every write

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ 
- Cargo

### Installation

1. **Clone the repository:**


2. **Build the project:**
```bash
cargo build --release
```

3. **Run the server:**
```bash
cargo run
```

The server will start on `http://127.0.0.1:4500`

---

## 📊 API Response Codes

| Code | Status | Description |
|------|--------|-------------|
| 200 | OK | Request successful (Get operations) |
| 201 | Created | Movie created successfully |
| 404 | Not Found | Movie ID not found |
| 400 | Bad Request | Invalid request body |
| 500 | Internal Server Error | Server error |

---

## 🔒 Thread Safety Features

### Read Operations (GET)
- **Lock Type:** `RwLock::read()`
- **Concurrency:** Multiple simultaneous reads
- **Blocking:** Non-blocking between reads
- **Use Case:** High-frequency queries

### Write Operations (POST - Add/Update/Delete)
- **Lock Type:** `RwLock::write()`
- **Concurrency:** Exclusive access
- **Blocking:** One write at a time
- **Use Case:** Data modifications with safety


## 📝 Example Usage

### Using cURL

**Get All Movies:**
```bash
curl -X POST http://127.0.0.1:4500/movies \
-H "Content-Type: application/json" \
-d '{ "action": "get" }'
```

**Get Movie By Id :**
```bash
curl -X POST http://127.0.0.1:4500/movies \
-H "Content-Type: application/json" \
-d '{
  "action": "get",
  "id": "movie-uuid-here"
}'
```

**Add Movie:**
```bash
curl -X POST http://127.0.0.1:4500/movies \
-H "Content-Type: application/json" \
-d '{
  "action": "add",
  "movie": {
    "title": "Oppenheimer",
    "director": "Christopher Nolan",
    "year": 2023
  }
}'
```

**Update Movie:**
```bash
curl -X POST http://127.0.0.1:4500/movies \
-H "Content-Type: application/json" \
-d '{
  "action": "update",
  "id": "movie-uuid-here",
  "movie": {
    "title": "Oppenheimer IMAX",
    "director": "Christopher Nolan",
    "year": 2024
  }
}'

```

**Delete Movie:**
```bash
curl -X POST http://127.0.0.1:4500/movies \
-H "Content-Type: application/json" \
-d '{
  "action": "delete",
  "id": "movie-uuid-here"
}'
```


