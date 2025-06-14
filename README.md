# SimpleGrep 🕵️‍♂️📄

A simple command-line text search tool in Rust, inspired by `grep`.  
Supports both **case-sensitive** and **case-insensitive** search.

---

## 🚀 Getting Started

Clone the repository and run with `cargo`:

```bash
cargo run -- <query> <input_file> > <output_file>
````

### Example

```bash
cargo run -- Rust input.txt > output.txt
```

This will search for the word `Rust` in `input.txt` with **case-sensitive** matching and writes the result to `output.txt` file if there is no any error. Otherwise, it prints out the error message in standard error console

---

## 🔍 Case Insensitive Search

To perform a **case-insensitive** search, set the environment variable `IGNORE_CASE` to `1`:

```bash
IGNORE_CASE=1 cargo run -- Rust input.txt > output.txt
```

---

## 🧪 Running Tests

The app comes with a built-in test suite. Run all tests using:

```bash
cargo test
```

---

## 🛠 Project Structure

* `main.rs`: Entry point that parses CLI arguments and calls the search logic.
* `lib.rs`: Contains the `Config` struct and the `run`, `search`, and `search_case_insensitive` functions.
* `tests/lib_test.rs`: Unit tests for both case-sensitive and case-insensitive search.
