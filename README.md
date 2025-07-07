# Cognito

Cognito is a blazing-fast, cross-platform application launcher built with Rust, designed from the ground up for ultimate extensibility and customization through WebAssembly (Wasm) extensions. Forget bloated, monolithic launchers – Cognito empowers you to tailor your experience by enabling only the features you need, exactly when you need them.

### Features

* ~**Cross-Platform Compatibility:** Cognito runs seamlessly across various operating systems.~
* **WebAssembly Extensibility:** Leverage the power and security of WebAssembly to create, share, and integrate new features as extensions.
* **Modular Design:** Every feature is an independent extension, allowing for a lightweight core and a highly customizable user experience.
* **Performance-Oriented:** Rust's performance capabilities ensure a snappy and responsive user experience.

### How It Works

Cognito's core is written in Rust, providing a robust and efficient foundation. Its true power lies in its extensibility model, which utilizes WebAssembly. This allows developers to write extensions in various languages that compile to Wasm (like Rust, C/C++, Go, AssemblyScript, etc.), ensuring high performance, security, and portability. When you enable an extension, Cognito loads and executes its Wasm module, integrating its functionality directly into the launcher.

### Getting Started

To get started with Cognito, you'll need to have Rust and Cargo installed.

1. **Clone the repository:**

    ```bash
    git clone https://github.com/LuizSSampaio/cognito.git
    cd cognito
    ```

2. **Build the application:**

    ```bash
    cargo build --release
    ```

3. **Run Cognito:**

    ```bash
    ./target/release/cognito
    ```

### Customization and Extensions

Cognito is designed to be externally customizable. You can manage extensions through a configuration interface (details on this will be added as the project evolves).

Examples of planned and potential extensions include:

* **File Search:** Quickly find files and folders on your system.
* **Calculator:** Perform quick calculations directly from the launcher.
* **Web Search:** Integrate with your favorite search engines.
* **Application Launcher:** Launch installed applications with ease.
* **Clipboard History:** Access your past copied items.
* **AI Chat**: Chat with your favorite llm directly from the launcher.

Stay tuned for documentation on how to develop your own WebAssembly extensions for Cognito!
