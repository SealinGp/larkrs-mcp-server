# larkrs-client

[![Crates.io](https://img.shields.io/crates/v/larkrs-client.svg)](https://crates.io/crates/larkrs-client)
[![Documentation](https://docs.rs/larkrs-client/badge.svg)](https://docs.rs/larkrs-client)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Rust client library for the Lark (Feishu) API.

## Features

- Authentication: Tenant access token management with automatic refresh
- Bitable: Read and write operations for Feishu Bitable
- Bot: Send messages and interact with chats

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
larkrs-client = "0.1.1"
```

## Usage

### Authentication

```rust
use larkrs_client::auth::FeishuTokenManager;

// Set FEISHU_APP_ID and FEISHU_APP_SECRET environment variables
let token_manager = FeishuTokenManager::new();
let token = token_manager.get_token().await?;
```

### Bitable Operations

```rust
use larkrs_client::bitable::table::BitableTableClient;

let client = BitableTableClient::new();

// Get records from a table
let records = client.get_records_list("app_token", "table_id").await?;

// Create records in a table
let records_json = r#"[
    {"field1": "value1", "field2": "value2"},
    {"field1": "value3", "field2": "value4"}
]"#;
client.batch_create_records_json("app_token", "table_id", records_json).await?;
```

### Bot Operations

```rust
use larkrs_client::bot::chat::ChatClient;

let client = ChatClient::new();

// Get list of chats
let chats = client.get_chat_group_list().await?;

// Send a text message
client.send_text_message("chat_id", "Hello from Rust!").await?;

// Send a markdown message
client.send_markdown_message(
    "chat_id",
    "Title",
    "# Heading\n**Bold text**\n- List item 1\n- List item 2"
).await?;
```

## Environment Variables

The library requires the following environment variables:

- `FEISHU_APP_ID`: Your Feishu application ID
- `FEISHU_APP_SECRET`: Your Feishu application secret

## License

This project is licensed under the MIT License - see the LICENSE file for details.
