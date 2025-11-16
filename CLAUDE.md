# sym-rest-rc

A Rust-based CLI client for interacting with the Symetry machine learning REST API and WebSocket services.

## Overview

This CLI tool provides a complete interface to the Symetry machine learning platform, supporting:
- Project management
- Data learning (REST and WebSocket)
- Model building
- Predictions (REST and WebSocket)
- Job status monitoring

## Installation

```bash
cargo build --release
```

The binary will be available at `./target/release/sym-rest-rc`

## Configuration

The tool requires a configuration file in TOML format. Configuration can be loaded in three ways:

1. Via `--config` flag: `sym-rest-rc --config=/path/to/config.toml`
2. Via environment variable: `export SML_CONFIG_FILE=/path/to/config.toml`
3. Default locations (checked in order):
   - `./rc.conf`
   - `~/.config/sym-rest-rc/config.toml`
   - `~/.sym-rest-rc/config.toml`

### Configuration File Format

```toml
host = "charm"
port = 8080
user = "c1"
secretkey = "A1ciUrDJgm5LIJU710bxeQ=="
use_ws_for_learn = true
use_ws_for_predit = true
```

### Environment Variable Overrides

The secret key can be overridden using the `SML_SK` environment variable, which takes precedence over the config file:

```bash
export SML_SK="YourSecretKeyHere=="
sym-rest-rc config  # Will use SML_SK value instead of config file
```

This is useful for:
- Keeping secrets out of config files
- CI/CD pipelines
- Different environments (dev, staging, production)

## Commands

### config

Display the currently loaded configuration.

```bash
sym-rest-rc config
```

### create

Create a new project using the REST API.

```bash
sym-rest-rc create --name=project1 --type=cpu [--params="key=value,key2=value2"] [--hist]
```

**Parameters:**
- `--name` (required): Project name
- `--type` (required): Project type (e.g., "cpu")
- `--params` (optional): Additional parameters as comma-separated key=value pairs
- `--hist` (optional): Enable histogram (default: false)

**Example:**
```bash
sym-rest-rc create --name=iris --type=cpu --params="enable_histogram=true" --hist
```

### learn

Learn or push data to a project using REST API or WebSocket.

```bash
sym-rest-rc learn --project=<name> --file=<path> --types=<types> [--use-ws]
```

**Parameters:**
- `--project` (required): Project name
- `--file` (required): Path to CSV data file
- `--types` (required): Comma-separated attribute types (C=continuous, B=binary)
- `--use-ws` (optional): Use WebSocket instead of REST API (default: false)

**Example:**
```bash
# Using REST API
sym-rest-rc learn --project=iris --file=./data.csv --types=C,C,C,C,B,B,B

# Using WebSocket
sym-rest-rc learn --project=iris --file=./data.csv --types=C,C,C,C,B,B,B --use-ws
```

### build

Create a new machine learning model using REST API (asynchronous operation).

The build command supports two forms:
1. **ID-based**: Use column indices for targets and inputs
2. **Name-based**: Use attribute names for targets and inputs

```bash
# ID-based
sym-rest-rc build --project=<name> --name=<model> --type=<algo> --targets="<ids>" --inputs="<ids>" [--params="<params>"]

# Name-based
sym-rest-rc build --project=<name> --name=<model> --type=<algo> --target-names="<names>" --input-names="<names>" [--params="<params>"]
```

**Parameters:**
- `--project` (required): Project name
- `--name` (required): Model name/ID
- `--type` (required): Algorithm type (e.g., "lda", "hba")
- `--targets` (optional): Target column IDs (comma-separated integers)
- `--inputs` (optional): Input column IDs (comma-separated integers)
- `--target-names` (optional): Target attribute names (comma-separated)
- `--input-names` (optional): Input attribute names (comma-separated)
- `--params` (optional): Model parameters as comma-separated key=value pairs

**Examples:**
```bash
# Using column IDs
sym-rest-rc build --project=iris --name=model1 --type=lda --targets="12" --inputs="0,1,2,3"

# Using attribute names
sym-rest-rc build --project=iris --name=model2 --type=lda \
  --target-names="Iris_setosa" \
  --input-names="sepal_length,sepal_width,petal_length,petal_width"

# With parameters
sym-rest-rc build --project=iris --name=model3 --type=lda --targets="12" --inputs="0,1,2,3" \
  --params="matrix_use_pseudoinv=false,sml_rcond_use=false"
```

**Note:** The build operation is asynchronous and returns a job ID. Use the `job` command to check status.

### job

Check the status of an asynchronous job (e.g., from build command).

```bash
sym-rest-rc job --id=<job_id>
```

**Parameters:**
- `--id` (required): Job ID returned from async operations

**Example:**
```bash
sym-rest-rc job --id=123
```

### predict

Make predictions with a trained model using REST API or WebSocket.

```bash
sym-rest-rc predict --project=<name> --model=<model> [--df=<json>] [--file=<path>] [--use-ws]
```

**Parameters:**
- `--project` (required): Project name
- `--model` (required): Model name/ID
- `--df` (optional): JSON dataframe string
- `--file` (optional): Path to CSV file with data
- `--use-ws` (optional): Use WebSocket instead of REST API (default: false)

**Note:** Must specify either `--df` or `--file`, but not both.

**Examples:**
```bash
# REST API with JSON
sym-rest-rc predict --project=iris --model=lda1 \
  --df='{"attributeNames":["sepal_length","sepal_width"],"data":[["4.3","3"]],"attributeTypes":["C","C"]}'

# REST API with file
sym-rest-rc predict --project=iris --model=lda1 --file=./data.csv

# WebSocket with file
sym-rest-rc predict --project=iris --model=lda1 --file=./data.csv --use-ws
```

## Architecture

### Authentication

All requests use HMAC-SHA256 authentication with the following headers:
- `Content-MD5`: Base64-encoded MD5 hash of request body
- `Sym-date`: Timestamp in format `YYYY-MM-DD HH:MM:SS;nanoseconds`
- `Customer-ID`: User/customer ID from configuration
- `Authorization`: Base64-encoded HMAC-SHA256 signature

### REST API Endpoints

- **Projects**: `POST /symetry/rest/{user}/projects`
- **Learn**: `POST /symetry/rest/{user}/projects/{project}/dss/{ds}/learn`
- **Build**: `POST /symetry/rest/{user}/projects/{project}/build`
- **Predict**: `POST /symetry/rest/{user}/projects/{project}/predict/{model}`
- **Jobs**: `GET /symetry/rest/{user}/jobs/{job_id}`

### WebSocket Endpoints

- **Learn**: `ws://{host}:{port}/symetry/ws/learn`
- **Predict**: `ws://{host}:{port}/symetry/ws/predict`

#### WebSocket Message Format

WebSocket messages use a special format:
```
{header_length},{headers_json}{payload_json}
```

Where:
- `header_length`: Length of the headers JSON object
- `headers_json`: JSON object with authentication headers and extra keys
- `payload_json`: The actual data payload

Example:
```
147,{"headers":["2025-11-16 20:11:50;294669000","QIrb...","5+38...","c1"],"extraKeys":["project1"]}{"attributeNames":[...],"data":[...],...}
```

### Data Structures

#### MLContext (Build Command)

```json
{
  "targets": [12],
  "inputAttributes": [0, 1, 2, 3],
  "inputAttributeNames": ["sepal_length", "sepal_width"],
  "targetAttributeNames": ["Iris_setosa"],
  "extraParameters": {
    "matrix_use_pseudoinv": "false",
    "sml_rcond_use": "false"
  }
}
```

#### DataFrame (Learn/Predict)

```json
{
  "attributeNames": ["sepal_length", "sepal_width", "petal_length", "petal_width"],
  "data": [
    ["4.3", "3", "1.1", "0.1"],
    ["4.8", "3", "1.4", "0.1"]
  ],
  "attributeTypes": ["C", "C", "C", "C"],
  "errorHandling": 1
}
```

## Project Structure

```
src/
├── main.rs              # Entry point and command routing
├── cli.rs               # Command-line argument definitions
├── config.rs            # Configuration loading and management
├── auth.rs              # HMAC authentication implementation
└── commands/
    ├── mod.rs           # Module exports
    ├── create_project.rs   # Project creation (REST)
    ├── learn_rest.rs       # Data learning (REST)
    ├── learn_ws.rs         # Data learning (WebSocket)
    ├── build_rest.rs       # Model building (REST)
    ├── job_rest.rs         # Job status checking (REST)
    ├── predict_rest.rs     # Predictions (REST)
    └── predict_ws.rs       # Predictions (WebSocket)
```

## Dependencies

- `clap`: Command-line argument parsing
- `serde` & `serde_json`: JSON serialization/deserialization
- `toml`: Configuration file parsing
- `tokio`: Async runtime
- `reqwest`: HTTP client for REST API
- `tokio-tungstenite`: WebSocket client
- `hmac` & `sha2`: HMAC-SHA256 authentication
- `md5`: MD5 hashing for content verification
- `base64`: Base64 encoding/decoding
- `chrono`: Timestamp generation

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Check compilation
cargo check

# Run with example config
export SML_CONFIG_FILE=./etc/rc.conf
cargo run -- config
```

### Adding New Commands

1. Add command variant to `Commands` enum in `src/cli.rs`
2. Create corresponding `Args` struct in `src/cli.rs`
3. Implement handler in `src/commands/`
4. Add module export to `src/commands/mod.rs`
5. Add command routing in `src/main.rs`

## License

[Add license information here]

## Authors

Developed with assistance from Claude (Anthropic).
