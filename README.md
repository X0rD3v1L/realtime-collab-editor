# Real Time Collob Editor

This project is a real-time collaborative document editing system built with Rust, leveraging the Actix Web framework and WebSockets for real-time communication. The system allows multiple users to edit a document simultaneously, with changes being synchronized across all connected clients in real-time.

## Features

- **Real-Time Collaboration**: Multiple users can edit the document simultaneously, and changes are reflected in real-time across all clients.
- **WebSocket-Based Communication**: Utilizes WebSockets for efficient, low-latency communication between the server and clients.
- **Modular Design**: The project is divided into separate modules for WebSocket handling, error management, and session management.
- **Responsive UI**: A modern, responsive user interface using the Quill.js rich text editor.

## Project Structure

- **`actor` Module**: Contains the implementation of the WebSocket actor, responsible for handling WebSocket connections and messages.
- **`errors` Module**: Defines custom error types for handling different error scenarios, including server creation errors.
- **`session_manager` Module**: Manages active WebSocket sessions, allowing for coordinated updates across all connected clients.

## Getting Started

### Prerequisites

- Rust (stable)
- Cargo (Rust's package manager)

### Installation

1. **Clone the repository**:

    ```sh
    git clone https://github.com/X0rD3v1L/realtime-collab-editor.git
    cd realtime-collab-editor
    ```

2. **Build the project**:

    ```sh
    cargo build --release
    ```

3. **Run the server**:

    ```sh
    cargo run --release
    ```

4. **Open the client**:

    Open your web browser and navigate to `http://localhost:8080/` to start using the collaborative editor.

### Configuration

- The server listens on `0.0.0.0:8080` by default. You can change the address in the `main` function in `src/main.rs`.

## How It Works

### Server

- **WebSocket Handling**: The server handles WebSocket connections through the `WebSocket` actor. This actor manages individual client connections and communicates with the `WsSessionManager` to propagate updates across all clients.

- **Session Management**: `WsSessionManager` keeps track of all active WebSocket sessions and ensures that updates are broadcasted to all connected clients.

### Client

- **Quill.js Integration**: The client uses Quill.js, a modern WYSIWYG editor, to provide rich text editing capabilities. Changes made in the editor are sent to the server via WebSocket, where they are processed and broadcasted to other clients.

- **Real-Time Updates**: The client listens for updates from the server and applies them to the editor, ensuring that all clients have a consistent view of the document.