## 🚀 Key Features

- **Process Spawning:** Uses `std::process::Command` to execute system binaries.
- **Piping (`|`):** Supports chaining multiple commands together, passing the `stdout` of one process into the `stdin` of the next.
- **I/O Redirection:**
    - Output Redirection (`>`): Writes command output to a specified file.
    - Input Redirection (`<`): Reads command input from a specified file.
- **Built-in Commands:**
    - `cd`: Changes the current working directory using `std::env`.
    - `exit`: Terminates the shell session.
- **Signal Handling:** Integrates `ctrlc` crate to handle `SIGINT` (Ctrl+C) gracefully, ensuring the shell doesn't crash and remains interactive.
- **State Management:** Uses a Read-Eval-Print Loop (REPL) and tracks `Child` processes to manage pipeline execution.

## 🛠️ Requirements

- **Rust Toolchain:** (rustc, cargo)
- **Crate Dependencies:** `ctrlc = "3.4"` (ensure this is in your `Cargo.toml`)