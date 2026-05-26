# RustPassLab

## Overview

RustPassLab is an educational password strength demonstrator written in Rust. The project shows how brute-force password attacks work by generating password guesses, hashing each guess with SHA-256, and comparing the generated hash against a target hash.

The purpose of this software is to explore Rust as a systems programming language while also demonstrating an important cybersecurity concept: password length and character-set complexity dramatically increase the number of possible combinations an attacker must search.

This project includes two ways to run the software:

* A command-line interface (CLI) educational demo
* A browser-based web GUI powered by a Rust backend API

The CLI version demonstrates the password-strength workflow in the terminal. The web GUI expands the project into a frontend/backend application where the browser communicates with a Rust Axum server.

The current web application includes:

* Weak password brute-force demo using lowercase letters
* Moderate password brute-force demo using lowercase letters, uppercase letters, and digits
* Strong password estimator using lowercase letters, uppercase letters, digits, and common symbols
* SHA-256 hash comparison
* Measured local CPU guesses-per-second rate
* Default rate fallback before a local benchmark is measured
* Shared in-memory server state for the latest measured CPU rate
* Browser-based result display with target and matched hash comparison
* Input validation in both the frontend and backend
* One-demo-at-a-time control lock to avoid overlapping brute-force runs

The Strong Password Estimator calculates an average-case search-space estimate using:

** charset size ^ password length / 2 **


After either the Weak Demo or Moderate Demo runs, the Rust backend stores the latest measured local guesses-per-second rate from your CPU into memory. The estimator then uses that measured local rate for estimations.

[Software Demo Video](http://youtube.link.goes.here)

---

## Development Environment

Tools used:

* Rust
* Cargo
* Visual Studio Code
* Git
* GitHub
* PowerShell terminal
* VS Code Live Server extension
* Web browser

Programming languages and technologies used:

* Rust
* HTML
* CSS
* JavaScript

Rust crates used:

* `sha2` for SHA-256 hashing
* `axum` for the Rust backend API server
* `tokio` for the async runtime
* `tower-http` for CORS support
* `serde` for JSON request and response handling
* `num-format` for comma-formatted attempt and rate output

Rust concepts explored in this project include:

* Ownership and borrowing
* Functions and modular design
* Structs and impl blocks
* Pattern matching
* Error handling and validation
* External crates
* Shared library modules
* Multiple binary targets
* API routing
* In-memory shared server state
* Performance timing and measurement

---

## How to Run the Project

This project can be run as either a CLI program or as a web GUI with a Rust backend server.

### Prerequisites

Before running the project, install:

* Rust and Cargo
* Git
* Visual Studio Code
* VS Code Live Server extension

On Windows, Rust may also require:

* Visual Studio Build Tools
* Desktop development with C++

### Clone the Repository

Create or open a local folder for the project files. Then run:

** In the Terminal **
git clone https://github.com/e-arndt/CSE-310-Rust-Password-Demonstrator.git
and 
cd CSE-310-Rust-Password-Demonstrator - To open this folder in Visual Studio Code.

The project root should contain:

Cargo.toml
Cargo.lock
src
web
README.md

You can use DIR to confirm
---

## Running the Web GUI

The web GUI requires two parts to run at the same time:

1. The Rust backend server
2. The browser frontend

### Start the Rust Backend Server

From the project root folder in terminal, run:

cargo run --release --bin server

This gives the best brute-force performance.


If --release mode doesn't work, run: (Win 11 can be a real grouch)

cargo run --bin server


When the backend starts successfully, the terminal should show:

RustPassLab server running at:
http://127.0.0.1:3000

Leave this terminal running while using the web interface.


### Start the Web Interface

In Visual Studio Code:

1. Open `web/` folder and find `index.html`
2. Right-click on the `index.html` file
3. Select **Open with Live Server**

The browser should open to a local address similar to:

http://127.0.0.1:5500/web/index.html


### Web GUI Notes

The web interface includes:

* **Weak Password Demo** — brute-forces short lowercase-only passwords.
* **Moderate Password Demo** — brute-forces short passwords using lowercase letters, uppercase letters, and digits.
* **Strong Password Estimator** — estimates brute-force difficulty using password length, character set size, and local CPU rate.

Running the Strong Password Estimator before running a demo uses a default rate of 5,000,000 guesses per second.

After running either the Weak Demo or Moderate Demo, the Rust backend stores the latest measured local CPU guesses-per-second rate in memory. The Strong Password Estimator then uses that measured local rate to estimate how long that strong passward might take to Brute-Force using your CPU.

The Rust backend must be running at:

http://127.0.0.1:3000

If the backend is not running, the web page may still load, but the demo and estimator buttons will not be able to connect to the Rust API.

---

## Running the CLI Version

The CLI version can be run from the project root in the terminal with:

cargo run --bin rust-pass-lab


** For best performance use: **

cargo run --release --bin rust-pass-lab


The CLI version walks through the educational password-strength demonstration in the terminal.

---

## Project Architecture

The project uses shared Rust modules so the CLI and web backend can reuse the same core logic.

CSE-310-Rust-Password-Demonstrator/
├── src/
│   ├── bin/
│   │   └── server.rs
│   ├── bruteforce.rs
│   ├── config.rs
│   ├── hashing.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── menu.rs
│   ├── models.rs
│   └── strong_estimator.rs
│
├── web/
│   ├── app.js
│   ├── favicon.png
│   ├── index.html
│   └── styles.css
│
├── Cargo.toml
├── Cargo.lock
└── README.md


### Module Responsibilities

### `main.rs`

Runs the CLI version of the project.

Handles:

* Terminal program flow
* CLI workflow
* User interaction
* CLI result display

### `src/bin/server.rs`

Runs the Rust backend server for the web GUI.

Handles:

* Axum API server setup
* API routes
* JSON request and response handling
* Backend validation
* Shared measured-rate state
* Weak Demo API endpoint
* Moderate Demo API endpoint
* Strong Estimator API endpoint

### `bruteforce.rs`

Handles:

* Deterministic brute-force searching
* Password guess generation
* Attempt counting
* SHA-256 hash comparison
* Measured guesses-per-second rate

### `hashing.rs`

Handles:

* SHA-256 password hashing

### `models.rs`

Handles:

* Result data structures
* Performance calculations

### `config.rs`

Handles:

* Centralized character set definitions
* Password length limits
* Shared configuration constants

### `strong_estimator.rs`

Handles:

* Strong password estimate calculations
* Average-case estimated attempts
* Estimated crack-time formatting

### `web/index.html`

Defines the browser interface layout.

### `web/styles.css`

Defines the visual layout, card styling, responsive design, result boxes, hash comparison display, and footer styling.

### `web/app.js`

Handles:

* Browser input validation
* Fetch calls to the Rust backend
* Result rendering
* Hash display formatting
* Demo control locking
* Dynamic footer year

---

## Example Web GUI Workflow

1. Start the Rust backend server.
2. Open the web interface with Live Server.
3. Run the Strong Password Estimator first to see the default estimate rate.
4. Run the Weak Demo and/or Moderate Demo.
5. Run the Strong Password Estimator again to see the estimate using local CPU rate.

Example inputs:

Weak Demo: happy
Moderate Demo: Zxz9
Strong Estimator: Rust@123

---

## Example Output

Example Weak Demo result:

Status: found
Password Found: happy
Charset Size: 26
Attempts: 494,265
Elapsed Time: 0.0547 seconds
Local CPU Rate: 9,035,815 guesses/sec


Example Strong Estimator result:

Rust API Status: estimated

Password Length: 8
Charset: lowercase a-z, uppercase A-Z, digits 0-9, and common symbols
Charset Size: 89
Local Rate: 5,000,000 guesses/sec
Rate Source: default estimate

Estimated Attempts: 1,968,294,402,851,041
Estimated Time: 12.47 years

After running a Weak or Moderate demo, the estimator uses the measured local CPU rate instead of the default estimate.

---

## Useful Websites

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Used as a reference for Rust syntax, ownership, borrowing, modules, and project structure.
* [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) - Used for Rust language and standard library reference.
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Helpful examples for Rust syntax and common patterns.
* [sha2 Crate Documentation](https://docs.rs/sha2/latest/sha2/) - Used for SHA-256 hashing support.
* [Axum Documentation](https://docs.rs/axum/latest/axum/) - Used as a reference for the Rust backend API server.
* [Password Depot: Brute-force attacks](https://www.password-depot.de/en/know-how/brute-force-attacks.htm) - Helped explain brute-force attacks, search-space growth, average vs. maximum search time, and why password length greatly increases difficulty.
* [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html) - Helpful background on password hashing, salts, and secure password storage practices.

---

## Future Work

Future improvements could include:

* Add real-time backend progress polling for longer brute-force demonstrations.
* Add a real cancellation system that stops the Rust brute-force loop safely.
* Add optional multithreaded brute-force searching.
* Add more detailed charts showing password length vs. search-space growth.
* Add persistent benchmark history across server restarts.
* Add more hashing algorithm comparisons.
* Add additional UI polish for mobile layouts.
