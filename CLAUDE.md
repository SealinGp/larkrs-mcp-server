# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Lark (Feishu) MCP Server that provides Model Context Protocol integration for Feishu APIs. It consists of two main components:

1. **larkrs-mcp** - The main MCP server binary that exposes Feishu functionality as MCP tools
2. **larkrs-client** - A Rust client library for the Lark (Feishu) API

## Common Commands

### Building and Development
```bash
# Build the entire project
cargo build

# Build for release
cargo build --release

# Run tests
cargo test

# Run tests for the client library only
cd larkrs-client && cargo test

# Run the MCP server
cargo run
```

### Testing Environment Setup
Set up environment variables by copying `.env.example` to `.env` and filling in your Feishu credentials:
```bash
cp .env.example .env
# Edit .env with your FEISHU_APP_ID and FEISHU_APP_SECRET
```

## Architecture

### Project Structure
- `src/main.rs` - MCP server entry point and tool definitions
- `larkrs-client/` - Standalone Feishu API client library
  - `src/auth.rs` - Token management with automatic refresh
  - `src/bitable/` - Bitable (spreadsheet) operations
  - `src/bot/` - Chat and messaging functionality

### Key Components

**MCP Server (`src/main.rs`)**
- Uses `poem-mcpserver` framework with STDIO transport
- Implements `#[Tools]` trait on `LarkServer` struct
- Each public async method becomes an MCP tool
- All tools return `Json<T>` wrapped responses

**Authentication (`larkrs-client/src/auth.rs`)**
- `FeishuTokenManager` handles tenant access token lifecycle
- Automatic token refresh with configurable buffer time
- Thread-safe token caching using Arc<Mutex<>>
- Requires `FEISHU_APP_ID` and `FEISHU_APP_SECRET` environment variables

**Bitable Operations (`larkrs-client/src/bitable/`)**
- `BitableTableClient` for spreadsheet operations
- Record CRUD operations with filtering and sorting
- Field metadata retrieval with type information
- Batch operations for efficiency

**Bot Operations (`larkrs-client/src/bot/`)**
- `ChatClient` for messaging functionality
- Text and markdown message sending
- Chat group listing and management

### MCP Tools Available

1. `table_records_list` - Get records from Bitable tables
2. `create_table_records_json` - Batch create records in Bitable
3. `table_fields_info` - Get field metadata for tables
4. `chat_group_list` - List available chat groups
5. `send_text_message` - Send text messages to chats
6. `send_markdown_message` - Send formatted markdown messages

### Data Flow
1. Environment variables loaded via `dotenvy`
2. Token manager handles Feishu authentication
3. Client modules use token manager for API calls
4. MCP server exposes client functionality as tools
5. All responses follow Lark API standard format with code/msg/data structure

### URL Format
Feishu URLs follow this pattern: `https://xxx.feishu.cn/base/{app_token}?table={table_id}&view={view_id}`
- Extract `app_token` and `table_id` from these URLs for API calls

### Error Handling
- Custom error types in `auth.rs` with thiserror
- API responses include success checking via `is_success()` method
- Network and JSON parsing errors are properly wrapped and propagated