# 0-Shell

A minimalist, fully functional Unix-like shell implemented entirely in Rust from scratch. 

This project explores the foundations of system programming, process execution, and file system abstractions by mimicking standard shell behaviors without relying on any external binaries (e.g., no spanning of `/bin/ls` or `/bin/bash`).

## Features

**0-shell** features a robust Read-Eval-Print Loop (REPL) that gracefully handles terminal inputs, quotes, and standard shell behaviors.

### Built-in Commands
The following commands are implemented purely via Rust's standard library and system calls:

- `cd` — Change directory (supports fallback to `$HOME`).
- `pwd` — Print working directory.
- `echo` — Print arguments to stdout (safely handles `'` and `"` quotes).
- `ls` — List directory contents. Supports flags:
  - `-a`: Show hidden dotfiles.
  - `-F`: Append file type indicators (`/`, `*`, `@`).
  - `-l`: Long format displaying permissions, UID/GID, size, and modified time.
- `cat` — Output the contents of files.
- `mkdir` — Create directories recursively.
- `touch` — Create new empty files.
- `cp` — Copy files into destinations or directories.
- `mv` — Move or rename files and directories.
- `rm` — Remove files (supports `-r` for recursive directory removal).
- `clear` — Clear the terminal natively using ANSI escape sequences.
- `exit` — Terminate the shell cleanly (also responds to `Ctrl+D`).

## Installation & Execution

Ensure you have Rust and Cargo installed.

```bash
# Clone the repository and navigate into it
cd 0-shell

# Run the shell in development mode
cargo run

# Or build an optimized standalone binary
cargo build --release
./target/release/0-shell
```

## Architecture

The project is broken into three atomic components for clean maintainability:
- `src/main.rs`: The entry point and primary REPL loop. Handles IO streams and command routing.
- `src/parser.rs`: A custom tokenizer that correctly groups arguments and preserves whitespace within string literals wrapped in quotes.
- `src/builtins.rs`: The core logic mapping Unix operations to standard Rust abstractions (e.g., `std::fs`, `std::env`, `std::os::unix::fs`).

## Dependencies
- [Chrono](https://crates.io/crates/chrono): Used strictly to format Unix timestamps into human-readable strings for the `ls -l` command.

## License

This project is proprietary. The code is publicly visible for portfolio and viewing purposes only. No unauthorized copying, modification, or distribution is permitted. See [License](LICENSE) for details.
