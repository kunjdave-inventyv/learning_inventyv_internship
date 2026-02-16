# Learnings_Inventyv_InternShip
This repository contains assignment folders covering different aspects of software development: Logic Building, (HTML/CSS) Page , Database Management Systems (DBMS) , Javascript Assignments , Exercism problems (Learn mode) Javascript , Rust Programming Language

---

## 📁 Repository Structure

```text
Inventyv_Training/
│
├── 01_LOGIC_BUILDING/
│   └── flowcharts.md
│
├── 02_HTML_CSS/
│   ├── index.html
│   ├── style.css
|   ├── README.md
│
├── 03_SQL_QUERIES/
│   ├── queries.md
│
├── 04_JAVASCRIPT/
│   ├── task1.js
|   ├── task2.html
│   ├── task3.js
|
├── 05_EXERCISM_JS/
|   ├── solutions of javascript learn mode of exercism
|
├── 06_RUST/
|   ├── _1_Loops_Task
|   ├── _2_Struct_Task
|   ├── _3_serde_json
|   ├── _4_ownership
|   ├── _5_Server_Request_Tracker
|   ├── _6_module_system
|   ├── _7_HashMap_Task
|   ├── _8_Inventory_Task
|   ├── _9_Inventory_with_Lifetime_Closure
|   ├── _10_Multithreaded_Application
|   ├── _11_http_server
└── README.md
```

---

## 📁 Repository Overview

### 01_LOGIC_BUILDING

**Description:** This folder contains flowcharts.

**Contents:**

- `flowcharts.md` - Contains a link to a Figma flowchart board that visualizes various flowcharts.

---

### 02_HTML_CSS_Task

**Description:** This folder contains a complete login/registration page implementation only using HTML and CSS.

**Contents:**

- `index.html` - The main HTML structure for the login/registration page
- `style.css` - Stylesheet containing all the styling and responsive design rules

---

### 03_sql_queries

**Description:** This folder contains a comprehensive MySQL database assignment focused on movie database management with 50 SQL queries.

**Contents:**


- `queries.md` - Contians both question and answer of the queries implemented.

---

### 04_javascript

**Description:** This folder contains Assignments related to javascript.

**Contents:**

- `Task1.js` - Promise implementation with array manipulation and conditional resolution
- `Task2.js` - Pattern printing problem
- `Task3.js` - Promise task using Symbol data types

---

### 05_Exercism_js

**Description:** This folder contains solutions of javascript(Learn mode) of exercism.org website.

---

### 06_Rust_Tasks

This folder contains Rust programs covering fundamental Rust concepts including loops, structs, serialization, concurrency, and module systems.

## 📚 Contents

### 1. **_1_Loops_Task**
Demonstrates different looping constructs in Rust:
- `for` loops
- `while` loops
- `loop` (infinite loops)
- Labeled loops
- Loop control with `break` and `continue`

### 2. **_2_Struct_Task**
Covers structs and their implementations:
- Defining structs
- Nested structs
- Getters and setters
- Associated methods
- Method implementations

### 3. **_3_serde_json**
Demonstrates serialization and deserialization using Serde:

#### _3_1_serialize_json
- Converting Rust data structures to JSON
- Using `serde` and `serde_json` crates
- Serialization with `#[derive(Serialize)]`

#### _3_2_deserialize_json
- Converting JSON to Rust data structures
- Deserialization with `#[derive(Deserialize)]`
- Error handling during deserialization

### 4. **_4_ownership**
Explores Rust's ownership system:
- Ownership rules
- Borrowing and references
- Mutable and immutable references
- The borrow checker

### 5. **_5_Server_Request_Tracker**
Covers concurrent programming with shared state management:

#### _5_1_Using_Mutex
- Using `Mutex<T>` for mutual exclusion
- Thread-safe shared state
- Lock acquisition and release

#### _5_2_Using_Rwlock
- Using `RwLock<T>` for read-write locks
- Multiple readers, single writer pattern
- Performance optimization for read-heavy workloads

### 6. **_6_module_system**
Demonstrates Rust's module organization:
- Using `mod` keyword to declare modules
- Using `use` keyword to import items
- Module visibility with `pub`
- Code organization and namespacing

### 7. **_7_HashMap_Task**
Explores Rust's hash-based collections:

#### _7_1_HashMap
- Creating and using `HashMap<K, V>`
- Inserting, updating, and retrieving values

#### _7_2_HashSet
- Creating and using `HashSet<T>`
- Set operations (union, intersection, difference)
- Checking membership
- Removing duplicates

### 8. _8_Inventory_Task
A comprehensive project integrating multiple Rust concepts:
- Practical application of structs, methods,traits and collections
- Inventory management system implementation
- Error handling and input validation

### 9. _9_Inventory_with_Lifetime_Closure
- Lifetime was added in generics implementation and also Closure is used instead of normal function in Display_All function

### 10. _10_Multithreaded_Application
- This application demonstrates safe, concurrent state management using Arc and Mutex in Rust.
- Concurrent Record Creation: A background thread generates unique data records every 10 seconds, utilizing an atomic counter for thread-safe ID generation.
- Real-Time State Monitoring: A dedicated printer thread snapshots the shared memory every 5 seconds, displaying records with human-readable timestamps (HH:MM:SS am/pm).
- Automated Memory Cleanup: Two independent janitor threads monitor the collection every second, removing records based on ID parity (Even/Odd) once they exceed a 20-second lifespan.
- Live Data Auditing: Separate counter threads continuously calculate and report the total number of even and odd records currently stored in the shared vector.

### 11. _11_http_server
- This application demonstrates a high-performance REST API server for movie management using Rust's async runtime and thread-safe concurrency primitives.
- Dual Thread Pool Design: Operates separate READ (4 workers) and WRITE (2 workers) thread pools, using `Arc<RwLock<Vec<Movie>>>` to enable concurrent GET operations while ensuring exclusive POST access for data modifications.
- All-POST API Pattern: Implements all CRUD operations via POST endpoints (`/movies/get`, `/movies/add`, `/movies/update`, `/movies/delete`) with automatic JSON persistence and UUID-based record management.
- Real-Time Thread Monitoring: Custom middleware logs each request with thread identification, demonstrating true concurrent processing across worker threads handling simultaneous client requests.
---