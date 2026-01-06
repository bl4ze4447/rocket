# 🚀 Rocket

A high-performance, **native**, and **cross-platform** file manager built from the ground up in **Rust**. Rocket focuses on speed, memory safety, and a seamless user experience across different environments.

## Features

* **Native & Cross-platform**: Developed to run natively with high efficiency on multiple operating systems.
* **Built with Rust & egui**: Utilizes the power of **Rust** for core logic and the **egui** library for a responsive, immediate-mode graphical user interface.
* **Concurrent Action Modals**: Unlike traditional file managers, Rocket uses a non-blocking modal system. You can perform multiple actions (Copy, Move, Rename, etc.) simultaneously, with each operation having its own informative modal window.
* **Parallelized Search**: Features an ultra-fast search engine that leverages Rust's concurrency model to scan the file system in parallel, delivering results blazingly fast.
* **Modern UI**: A clean, dark-themed interface designed for clarity and ease of use.

## Getting Started

### Prerequisites

You must have the Rust toolchain installed. If you haven't installed it yet, visit [rustup.rs](https://rustup.rs/).

### Installation

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/bl4ze4447/rocket.git
    ```
2.  **Navigate to the project folder**:
    ```bash
    cd rocket
    ```
3.  **Build and Run**:
    ```bash
    cargo run --release
    ```

## Contributing

Contributions are welcome! If you have ideas for improvements or find any issues, feel free to open a pull request or an issue on the [GitHub repository](https://github.com/bl4ze4447/rocket).

---
