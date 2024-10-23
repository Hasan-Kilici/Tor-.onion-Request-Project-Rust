# Tor .onion Request Project in Rust

This project demonstrates how to make HTTP GET requests to .onion sites through the Tor network using Rust. The code utilizes the `reqwest` library to handle HTTP requests and implements retry logic to ensure robustness in the face of potential connection issues.

## Features

- Connects to Tor's SOCKS5 proxy.
- Implements retry logic with a maximum of 3 attempts and a 5-second delay between retries.
- Configurable request timeout (set to 60 seconds).

## Requirements

- Rust (1.79 or later)
- Cargo (comes with Rust)
- Tor service running on your machine

## Setup Instructions

1. **Install Rust:**
   If you haven't installed Rust yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
   or

   ```bash
    sudo pacman -S Rust
   ```

2. **Install Tor:**
   To install Tor on Arch Linux, use the following command:

   ```bash
    sudo pacman -S tor
    ```

3. **Start Tor Service: Enable and start the Tor service:**

    ```bash
    sudo systemctl enable tor
    sudo systemctl start tor
    ```

4. **Clone the Repository: Clone this repository to your local machine:**

    ```bash
    git clone https://github.com/Ferivonus/Tor-.onion-Request-Project-Rust.git
    cd Tor-.onion-Request-Project-Rust
    ```

5. **build the Application: Use Cargo to build the application:**

    ```bash
    sudo cargo build
    ```

6. **Run the Application: Use Cargo to run the application:**

    ```bash
    sudo cargo run
    ```
