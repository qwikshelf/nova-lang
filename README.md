# Nova Programming Language 🌌

> **"Write like Python, Run like C."**

Nova is a new general-purpose programming language built in **Rust**. It combines the rapid development feel of scripting languages with the safety and strictness of systems languages.

**Current Status:** v0.1 (Tree-Walk Interpreter)

## 🚀 Key Features (v0.1)

* **C-Family Syntax:** Familiar curly braces `{}` and syntax.
* **Variable Bindings:** `let` for immutable (default) and `mut` for mutable data.
* **First-Class Functions:** Functions are values, supporting closures and high-order logic.
* **Expressions:** Everything is an expression (e.g., `if` returns a value).
* **Safety:** Built on Rust's memory safety guarantees.

## 🛠 Installation & Usage

### Prerequisites
* [Rust & Cargo](https://rustup.rs/) installed.

### Build and Run
Clone the repository and run the REPL (Read-Eval-Print Loop):

```bash
git clone [https://github.com/qwikshelf/nova-lang.git](https://github.com/qwikshelf/nova-lang.git)
cd nova-lang
cargo run