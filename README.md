<div align="center">

![Leetify Banner](images/banner.png)

[![CI](https://github.com/muijf/leetify/workflows/CI/badge.svg)](https://github.com/muijf/leetify/actions)
[![crates.io](https://img.shields.io/crates/v/leetify.svg)](https://crates.io/crates/leetify)
[![docs.rs](https://docs.rs/leetify/badge.svg)](https://docs.rs/leetify)
[![license](https://img.shields.io/crates/l/leetify.svg)](https://github.com/muijf/leetify/blob/main/LICENSE)

**A Rust client library for the Leetify Public CS API**

*Type-safe, async API client with comprehensive error handling, builder pattern configuration, and an ergonomic extended Player API.*

[Documentation](https://docs.rs/leetify) • [Examples](examples/) • [Contributing](CONTRIBUTING.md) • [Code of Conduct](CODE_OF_CONDUCT.md)

</div>

---

**Key Features:** Type-safe IDs • Builder pattern • Extended Player API • Comprehensive error handling • Full async/await support • Automatic JSON deserialization • API key validation • Feature flags for customization

---

## Table of Contents

- [Installation](#installation)
- [Feature Flags](#feature-flags)
- [Quick Start](#quick-start)
- [API Methods](#api-methods)
- [Extended API](#extended-api)
- [Type Safety](#type-safety)
- [Error Handling](#error-handling)
- [Examples](#examples)
- [Development](#development)
- [License](#license)
- [Contributing](#contributing)

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leetify = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

> **Note**: This library requires an async runtime. Tokio is recommended, but any async runtime compatible with `reqwest` will work.

## Feature Flags

> Most features are optional to keep the core library lightweight. Enable only what you need.

**Core Features:**
- `default` - Enables all default features (`player`, `rustls-tls`)
- `player` - Enables the extended `Player` API for a more ergonomic interface
- `rustls-tls` - Uses `rustls` as the TLS backend for reqwest (default, recommended)
- `native-tls` - Uses `native-tls` as the TLS backend for reqwest

**Quick examples:**

```toml
# Default (includes player API and rustls)
leetify = "0.1.1"

# Minimal setup (without Player API)
leetify = { version = "0.1.1", default-features = false, features = ["rustls-tls"] }

# With native-tls instead of rustls
leetify = { version = "0.1.1", default-features = false, features = ["player", "native-tls"] }

# Custom feature combination
leetify = { version = "0.1.1", default-features = false, features = ["player", "rustls-tls"] }
```

> **Note**: The `player` feature enables the extended `Player` API which provides a more ergonomic interface. The core `Client` API is always available.

## Quick Start

```rust
use leetify::{Client, Id};

#[tokio::main]
async fn main() -> Result<(), leetify::Error> {
    // Create a client
    let client = Client::new();

    // Get player profile by Steam64 ID
    let profile = client
        .get_profile(Id::Steam64("76561198283431555".into()))
        .await?;

    println!("Player: {}", profile.name);
    println!("Winrate: {:.2}%", profile.winrate * 100.0);

    Ok(())
}
```

**Output:**
```
Player: PlayerName
Winrate: 52.34%
```

> For more examples and usage patterns, see the [examples](#examples).

## API Methods

### Get Player Profile

```rust
use leetify::{Client, Id};

let client = Client::new();

// By Steam64 ID
let profile = client
    .get_profile(Id::Steam64("76561198283431555".into()))
    .await?;

// By Leetify ID (UUID format)
let profile = client
    .get_profile(Id::Leetify("5ea07280-2399-4c7e-88ab-f2f7db0c449f".into()))
    .await?;

// Using automatic conversion (UUID format -> Leetify, numeric strings -> Steam64)
let id: Id = "76561198283431555".into();
let profile = client.get_profile(id).await?;
```

### Get Match History

```rust
use leetify::{Client, Id};

let client = Client::new();

let matches = client
    .get_profile_matches(Id::Steam64("76561198283431555".into()))
    .await?;

for match_details in matches {
    println!("Match: {} on {}", match_details.id, match_details.map_name);
}
```

### Get Match Details

```rust
use leetify::{Client, DataSource};

let client = Client::new();

// By game ID
let match_details = client
    .get_match_by_game_id("match-id-123".to_string())
    .await?;

// By data source
let match_details = client
    .get_match_by_data_source(DataSource::FACEIT, "faceit-match-id")
    .await?;
```

### Validate API Key

```rust
use leetify::Client;

let client = Client::with_api_key("your-api-key".to_string());

match client.validate_api_key().await {
    Ok(()) => println!("API key is valid"),
    Err(e) => eprintln!("Validation failed: {}", e),
}
```

### Using an API Key

API keys provide higher rate limits. You can obtain one at [https://leetify.com/app/developer](https://leetify.com/app/developer).

```rust
use leetify::Client;

let client = Client::with_api_key("your-api-key".to_string());
```

### Builder Pattern

For advanced configuration:

```rust
use leetify::Client;
use std::time::Duration;

let client = Client::builder()
    .api_key("your-api-key")
    .timeout(Duration::from_secs(60))
    .base_url("https://custom-api.example.com")
    .build()?;
```

## Extended API

> The extended `Player` API provides a more ergonomic interface by storing the player ID, allowing you to call methods without passing it each time. Enable the `player` feature to use this API.

For a more ergonomic API, use the `Player` struct which stores the id
and allows you to call methods without passing it each time:

```rust
use leetify::{Client, Id};

let client = Client::new();
let player = client.player(Id::Steam64("76561198283431555".into()));

// Get profile
let profile = player.profile().await?;

// Get matches
let matches = player.matches().await?;
```

## Type Safety

The library provides type-safe wrappers to prevent mixing up different ID types:

- `Id` - Enum for player identification (either `Steam64` or `Leetify`)
- `Steam64Id` - For Steam 64-bit IDs (numeric strings, typically 17 digits)
- `LeetifyId` - For Leetify user IDs (UUID format: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`)
- `DataSource` - Enum for data sources (FACEIT, Matchmaking, etc.)

The `Id` enum can be created from strings with automatic detection:
- Strings matching UUID format (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`) are treated as Leetify IDs
- Numeric strings (15+ digits) are treated as Steam64 IDs
- You can also explicitly use the enum variants (`Id::Steam64` or `Id::Leetify`) for clarity

## Error Handling

The library provides comprehensive error types:

```rust
use leetify::Error;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(Error::MissingParameter(msg)) => eprintln!("Missing required parameter: {}", msg),
    Err(Error::InvalidApiKey) => eprintln!("Invalid API key"),
    Err(Error::Http(e)) => eprintln!("HTTP error: {}", e),
    Err(Error::Api(status, msg)) => eprintln!("API error {}: {}", status, msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

> Run any example with: `cargo run --example <name>`

**Core Examples:**
- **[`basic_usage`](examples/basic_usage.rs)** - Basic API usage with all methods
- **[`player_api`](examples/player_api.rs)** - Extended Player API usage

### Basic Usage Example

```rust
use leetify::{Client, Id, DataSource};

#[tokio::main]
async fn main() -> Result<(), leetify::Error> {
    let client = Client::new();

    // Get profile
    let profile = client
        .get_profile(Id::Steam64("76561198283431555".into()))
        .await?;

    // Get matches
    let matches = client
        .get_profile_matches(Id::Steam64("76561198283431555".into()))
        .await?;

    // Get match details
    let match_details = client
        .get_match_by_data_source(DataSource::FACEIT, "faceit-match-id")
        .await?;

    Ok(())
}
```

### Extended Player API Example

```rust
use leetify::{Client, Id};

#[tokio::main]
async fn main() -> Result<(), leetify::Error> {
    let client = Client::new();
    let player = client.player(Id::Steam64("76561198283431555".into()));

    // No need to pass ID each time
    let profile = player.profile().await?;
    let matches = player.matches().await?;

    Ok(())
}
```

## Development

**Format code:**
```bash
cargo fmt --all
```
Formats all Rust code according to the official style guide.

**Lint code:**
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
Runs Clippy linter with all targets and features enabled, treating warnings as errors.

**Run tests:**
```bash
cargo test --all-features
```
Runs all tests with all features enabled to ensure comprehensive coverage.

**Run doc tests:**
```bash
cargo test --doc
```
Runs documentation tests to ensure all code examples compile and work correctly.

> **Editor setup**: Recommended extensions are available in [`.vscode/extensions.json`](.vscode/extensions.json). See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and pre-commit hooks.

## Rate Limits

- **Without an API key**: Subject to increased rate limits
- **With an API key**: Higher rate limits (check [Leetify documentation](https://leetify.com/app/developer) for current limits)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
