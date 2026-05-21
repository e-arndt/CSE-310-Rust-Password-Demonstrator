# RustPassLab

## Overview

RustPassLab is an educational brute-force password strength demonstrator written in Rust. The purpose of this project is to explore systems-level programming concepts while demonstrating how password complexity affects the difficulty and time required for brute-force attacks.

Brute-force attacks are one method of defeating passwords by systematically attempting every possible password combination until the correct password is found. This project demonstrates how weak passwords can be discovered quickly, while increased password complexity using longer lengths, uppercase letters, numbers, and symbols dramatically increases the search space and computational difficulty.

The software uses deterministic brute-force searching and SHA-256 hashing to demonstrate how password verification systems work. Passwords are hashed and compared using their SHA-256 hash values rather than comparing plaintext passwords directly.

This project was also created to further explore Rust programming concepts such as:

* Ownership and borrowing
* Functions and modular design
* Structs and impl blocks
* Error handling and validation
* External crates
* Performance measurement
* Command-line application development
* Cross-platform terminal behavior

Current project features include:

* Deterministic brute-force password generation
* SHA-256 password hashing
* Input validation and retry handling
* Modular Rust architecture
* Performance timing and statistics
* Cross-platform terminal clearing support
* Configurable centralized project constants

The current implementation focuses on lowercase alphabetical passwords with a maximum length of five characters in order to keep the demonstration educational, controlled, and practical for short runtime demonstrations.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

## Tools Used

* Rust
* Cargo
* Visual Studio Code
* Git
* GitHub
* PowerShell Terminal

## Programming Language and Libraries

This project was written in Rust using the Rust Cargo build system.

External crates used:

* `sha2` crate for SHA-256 hashing support

Standard library features used:

* File and module organization
* Input/output handling
* Timing and performance measurement
* Cross-platform process execution

# Project Architecture

The project is separated into multiple Rust modules for organization and maintainability.

```text
src/
├── main.rs
├── bruteforce.rs
├── hashing.rs
├── models.rs
└── config.rs
```

## Module Responsibilities

### `main.rs`

Handles:

* User interaction
* Input validation
* Program flow
* Screen display
* Final result reporting

### `bruteforce.rs`

Handles:

* Deterministic brute-force searching
* Password generation
* Search iteration logic
* Attempt counting
* Hash comparison

### `hashing.rs`

Handles:

* SHA-256 password hashing

### `models.rs`

Handles:

* CrackResult data structure
* Performance calculations

### `config.rs`

Handles:

* Centralized configuration values
* Character set definitions
* Maximum password length settings

# Example Output

```text
RustPassLab
Educational brute-force password strength demonstrator
Target password length limit: 5
Charset: lowercase a-z

Target SHA-256 hash: 254a82f42b25369e5371db7213fee93838baf8f5e53f25d03376b93fcf597376

Password found!
Password: bells
Attempts: 1010275
Elapsed time: 0.1025 seconds
Average rate: 9859612 guesses/sec
```

# Useful Websites

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust Documentation](https://doc.rust-lang.org/)
* [Crates.io](https://crates.io/)
* [sha2 Crate Documentation](https://docs.rs/sha2/latest/sha2/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

# Future Work

Potential future improvements include:

* Configurable character sets
* Uppercase, lowercase, numeric, and symbol combinations
* Password search-space estimation
* Estimated brute-force completion time calculations
* Optional multithreaded brute-force searching
* Simple graphical or web-based interface
* Benchmark comparisons across different hardware
* Configurable password length limits
* Additional hashing algorithms for comparison
* Educational statistics and password complexity analysis
