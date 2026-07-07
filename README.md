# Rust Exercises

A collection of 15 exercises to practice Rust, following along with **"The Rust Programming Language"** book. Each exercise focuses on core concepts (ownership, borrowing, generics, traits, error handling, concurrency, async, FFI) while building something concrete.

## How to use this repo

Suggested structure — one binary/crate per exercise:

```
rust-exercises/
├── ex01_linked_list/
├── ex02_quicksort/
├── ex03_binary_search_tree/
├── ex04_stack_queue/
├── ex05_calculator_cli/
├── ex06_todo_cli/
├── ex07_minesweeper/
├── ex08_multithreaded_word_counter/
├── ex09_generic_traits_shapes/
├── ex10_mini_key_value_store/
├── ex11_gpx_route_processor/
├── ex12_football_probability_simulator/
├── ex13_in_memory_api_cache/
├── ex14_system_dashboard_tui/
└── ex15_ffi_optimization/
```

Each folder can be its own `cargo new` project, or you can set them up as members of a Cargo workspace with a root `Cargo.toml`:

```toml
[workspace]
members = [
    "ex01_linked_list",
    "ex02_quicksort",
    "ex03_binary_search_tree",
    "ex04_stack_queue",
    "ex05_calculator_cli",
    "ex06_todo_cli",
    "ex07_minesweeper",
    "ex08_multithreaded_word_counter",
    "ex09_generic_traits_shapes",
    "ex10_mini_key_value_store",
    "ex11_gpx_route_processor",
    "ex12_football_probability_simulator",
    "ex13_in_memory_api_cache",
    "ex14_system_dashboard_tui",
    "ex15_ffi_optimization",
]
```

---

## Exercise 1 — Singly Linked List

**Goal:** Implement a generic singly linked list from scratch (`push_front`, `pop_front`, `peek`, `len`, `is_empty`, and an iterator).

**Concepts:** ownership, `Box<T>`, `Option<T>`, pattern matching, recursive data structures, implementing `Iterator`.

**Bonus:** Add a `Drop` implementation to avoid stack overflow on deep recursive drops, and try implementing a doubly linked list using `Rc<RefCell<T>>`.

---

## Exercise 2 — Sorting Algorithms (Quicksort & Mergesort)

**Goal:** Implement `quicksort` and `mergesort` for a `Vec<T>` where `T: Ord`, sorting in place (quicksort) and out of place (mergesort).

**Concepts:** generics, trait bounds (`Ord`, `PartialOrd`), slices, recursion, in-place swapping with indices.

**Bonus:** Benchmark both against `Vec::sort()` using `std::time::Instant` on random data of different sizes.

---

## Exercise 3 — Binary Search Tree

**Goal:** Build a generic Binary Search Tree supporting `insert`, `contains`, `remove`, and an in-order traversal that returns a sorted `Vec<T>`.

**Concepts:** recursive enums (`enum Tree<T> { Leaf, Node(...) }`), `Box<T>`, generics with trait bounds, recursion.

**Bonus:** Implement pre-order and post-order traversal iterators.

---

## Exercise 4 — Stack & Queue (with a Vec vs. VecDeque comparison)

**Goal:** Implement a `Stack<T>` and a `Queue<T>` as thin wrappers, then use them to solve a classic problem: check if parentheses/brackets in a string are balanced.

**Concepts:** `Vec<T>`, `VecDeque<T>`, encapsulation with structs, trait implementations (`impl<T> Stack<T>`).

**Bonus:** Implement a queue using two stacks.

---

## Exercise 5 — Command-Line Calculator

**Goal:** Build a CLI that reads an expression like `3 + 4 * 2` from stdin (or args) and evaluates it, respecting operator precedence.

**Concepts:** parsing strings, `enum` for tokens, error handling with `Result<T, E>` and custom error types, `std::env::args`, `match`.

**Bonus:** Support parentheses and implement a simple recursive-descent parser instead of shunting-yard.

---

## Exercise 6 — To-Do List CLI with Persistence

**Goal:** A command-line to-do app (`add`, `list`, `done`, `remove`) that persists tasks to a JSON or plain-text file between runs.

**Concepts:** structs + `derive(Debug, Serialize, Deserialize)`, file I/O (`std::fs`), `Result`/`?` operator, external crates (`serde`, `serde_json`), `clap` for argument parsing.

**Bonus:** Add due dates and sort tasks by priority/date.

---

## Exercise 7 — Minesweeper (terminal-based)

**Goal:** Implement classic Minesweeper playable in the terminal: generate a grid with randomly placed mines, reveal cells, flag cells, compute adjacent mine counts, and detect win/loss.

**Concepts:** 2D vectors (`Vec<Vec<T>>`), enums for cell state, randomness (`rand` crate), recursion or BFS/DFS for flood-fill reveal of empty cells, reading user input in a loop.

**Bonus:** Add a simple text-based interface with colored output using the `colored` or `crossterm` crate.

---

## Exercise 8 — Multithreaded Word Counter

**Goal:** Read one or more text files and count word frequencies, splitting the work across multiple threads (e.g., one thread per file, or split a large file into chunks), then merge results.

**Concepts:** `std::thread`, `Arc<Mutex<T>>` or channels (`mpsc`), `HashMap<String, usize>`, ownership across threads, closures with `move`.

**Bonus:** Compare a single-threaded vs. multi-threaded version and print timing results.

---

## Exercise 9 — Generics & Traits: Shapes Library

**Goal:** Define a `Shape` trait with methods like `area()` and `perimeter()`, then implement it for `Circle`, `Rectangle`, and `Triangle`. Store a collection of mixed shapes using trait objects (`Box<dyn Shape>`) and print a summary.

**Concepts:** traits, trait objects (`dyn`), generics vs. dynamic dispatch, default trait methods, operator overloading (optional, via `std::ops`).

**Bonus:** Add a `Summable` trait to compute the total area of all shapes, and implement `Display` for pretty printing.

---

## Exercise 10 — Mini In-Memory Key-Value Store with a Simple TCP Interface

**Goal:** Build a small key-value store (backed by a `HashMap`) exposed over a simple text-based TCP protocol (e.g., `SET key value`, `GET key`, `DEL key`), similar in spirit to a tiny Redis.

**Concepts:** `std::net::TcpListener`/`TcpStream`, concurrency (`Arc<Mutex<HashMap<...>>>` or async with `tokio`), error handling, parsing simple protocols, basic client-server architecture.

**Bonus:** Add persistence (write commands to a log file) and a simple `--client` mode to connect and send commands interactively.

---

## Exercise 11 — Marathon Route Processor (GPX)

**Goal:** Build a CLI that reads a `.gpx` (XML) file from a running workout, extracts the spatial coordinates, and calculates the total distance, average pace, and cumulative elevation gain.

**Concepts:** file reading, XML parsing (`quick-xml`), advanced iterators (`fold`, `map`), distance calculation with the Haversine formula.

**Bonus:** Generate a JSON summary with per-kilometer splits.

---

## Exercise 12 — Football Odds Simulator

**Goal:** Consume a REST API (or a local file) with historical statistics for national teams (e.g., Mexico vs. England). Run a Monte Carlo simulation with 100,000 iterations to predict outcome probabilities.

**Concepts:** asynchronous HTTP client (`reqwest`), serialization (`serde`), data-parallel concurrency (`rayon`), random number generation (`rand`).

**Bonus:** Show a terminal progress bar (`indicatif`) while the simulations run.

---

## Exercise 13 — In-Memory API Cache (CRM-style)

**Goal:** Build a lightweight web server that acts as a reverse proxy. It receives `GET` requests looking up specific IDs in an executive hierarchy, queries a simulated external API, and stores the response in memory. Subsequent requests should be served from the cache in microseconds.

**Concepts:** asynchronous web servers (`axum` or `actix-web`), async concurrency (`tokio`), shared state management (`RwLock` or `Mutex` inside an `Arc`).

**Bonus:** Implement time-based cache invalidation (TTL) using `moka`.

---

## Exercise 14 — Terminal System Dashboard (TUI)

**Goal:** Create an interactive terminal interface that shows real-time CPU usage, RAM, and system temperature, similar to the `htop` tool.

**Concepts:** terminal UIs (`ratatui`), interacting with hardware/OS resources (`sysinfo`), asynchronous event loops.

**Bonus:** Add interactivity to sort processes by RAM usage using the keyboard.

---

## Exercise 15 — FFI Integration & Optimization

**Goal:** Write a computationally heavy math function in Rust, compile it as a static (C-compatible) library, and call it from a simple script in another language (such as C or Swift).

**Concepts:** Foreign Function Interface (FFI), `extern "C"`, `#[no_mangle]`, manual memory management without a garbage collector, raw pointers.

**Bonus:** Understand how this precompiled code can be integrated into mobile environments (iOS) to run high-performance logic.

---

## Suggested order

1. Exercise 4 (Stack & Queue) — warm-up with basic collections.
2. Exercise 9 (Shapes / Traits) — get comfortable with traits before harder data structures.
3. Exercise 1 (Linked List) — dive into ownership and `Box`.
4. Exercise 3 (Binary Search Tree) — build on recursive data structures.
5. Exercise 2 (Sorting Algorithms) — generics and slices.
6. Exercise 5 (Calculator CLI) — parsing and error handling.
7. Exercise 6 (To-Do CLI) — file I/O and external crates.
8. Exercise 7 (Minesweeper) — combine everything into a small game.
9. Exercise 8 (Multithreaded Word Counter) — concurrency.
10. Exercise 10 (Key-Value Store) — networking and concurrency together.
11. Exercise 11 (GPX Route Processor) — file parsing and iterators.
12. Exercise 13 (In-Memory API Cache) — async web servers and shared state.
13. Exercise 12 (Football Odds Simulator) — async HTTP clients and data-parallel concurrency.
14. Exercise 14 (System Dashboard TUI) — terminal UIs and async event loops.
15. Exercise 15 (FFI Integration) — foreign function interfaces and low-level memory management.

Good luck, and happy hacking with Rust! 🦀
