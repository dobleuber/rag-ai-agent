# RAG AI Agent

A Rust-based AI agent that implements Retrieval-Augmented Generation (RAG) using OpenAI and Qdrant vector database. This project allows you to interact with documents through natural language queries, leveraging the power of embeddings for semantic search.

## Features

- Document embedding generation using OpenAI
- Vector storage and retrieval using Qdrant
- Natural language queries processing
- Asynchronous operations with Tokio runtime

## Prerequisites

- Rust (edition 2021)
- OpenAI API key
- Qdrant instance (cloud or local)

## Environment Variables

The project includes an `.env.example` file that shows the required environment variables. Copy this file to create your own `.env`:

```bash
cp .env.example .env
```

Then edit the `.env` file with your actual values:

```env
OPENAI_API_KEY=your_openai_api_key
QDRANT_API_KEY=your_qdrant_api_key
QDRANT_URL=your_qdrant_url
```

## Installation

1. Clone the repository
2. Install dependencies:
```bash
cargo build
```

## Usage

Run the application:
```bash
cargo run
```

The agent will prompt you for questions that it can answer based on the embedded documents in the vector database.

## Project Structure

- `src/main.rs` - Application entry point and runtime configuration
- `src/agents.rs` - AI agent implementation with OpenAI and Qdrant integration
- `src/file.rs` - File handling utilities
- `src/io_utils.rs` - Input/Output utilities
- `src/state.rs` - Application state management

## Dependencies

- `anyhow` - Error handling
- `async-openai` - OpenAI API client
- `qdrant-client` - Qdrant vector database client
- `tokio` - Async runtime
- `dotenv` - Environment variable management
- `serde` - Serialization/Deserialization
- `uuid` - Unique identifier generation

## License

This project is licensed under the MIT License - see the LICENSE file for details.
