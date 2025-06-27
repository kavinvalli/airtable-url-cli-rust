# Shortlink

A command-line tool for managing shortlinks using Airtable as a backend. This Rust application allows you to create, retrieve, and manage URL shortlinks through Airtable's API.

## Features

- 🔗 Create shortlinks with custom slugs
- 📋 List all existing shortlinks
- 🔍 Look up specific shortlinks by slug
- 🛡️ Secure configuration via environment variables
- 🎨 Colored output for better user experience

## Prerequisites

- Rust (latest stable version)
- An Airtable account with:
  - A base (workspace)
  - A table with `slug` and `target` fields
  - An API key (Personal Access Token)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd shortlink
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/shortlink`

## Configuration

You can configure the application using environment variables or command-line arguments. The following environment variables are required:

- `AIRTABLE_SECRET_TOKEN`: Your Airtable Personal Access Token
- `AIRTABLE_BASE`: Your Airtable base ID
- `AIRTABLE_TABLE`: Your Airtable table name

### Setting up environment variables

Create a `.env` file in the project root:

```env
AIRTABLE_SECRET_TOKEN=your_airtable_token_here
AIRTABLE_BASE=your_base_id_here
AIRTABLE_TABLE=your_table_name_here
```

### Airtable Setup

1. Create a new base in Airtable
2. Create a table with the following fields:
   - `slug` (Single line text) - The short identifier
   - `target` (Single line text) - The full URL to redirect to
3. Get your base ID from the API documentation
4. Generate a Personal Access Token in your Airtable account settings

## Usage

### Basic Commands

```bash
# List all shortlinks
shortlink get-all-records

# Look up a specific shortlink by slug
shortlink get-record-by-slug <slug>

# Create a new shortlink
shortlink create-record <slug> <target-url>
```

### Examples

```bash
# List all existing shortlinks
shortlink get-all-records

# Look up the target URL for slug "gh"
shortlink get-record-by-slug gh

# Create a new shortlink from "docs" to "https://docs.example.com"
shortlink create-record docs https://docs.example.com

# Using command-line arguments instead of environment variables
shortlink --secret-token your_token --base your_base --table your_table get-all-records
```

### Command-line Options

- `-s, --secret-token`: Airtable secret token (overrides `AIRTABLE_SECRET_TOKEN`)
- `-b, --base`: Airtable base (overrides `AIRTABLE_BASE`)
- `-t, --table`: Airtable table (overrides `AIRTABLE_TABLE`)

## Development

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Cargo

```bash
cargo run -- get-all-records
cargo run -- create-record test https://example.com
```
