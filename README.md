# KVAdminer-RS

KVAdminer-RS is a simple web application for managing Redis key-value stores. It provides functionalities to connect to a Redis database, view and manage keys, and edit key values.

## Features

- Connect to Redis with configurable host, port, username, and password
- View and manage keys with pagination
- Edit existing keys
- Create and delete keys

## Prerequisites

- Docker

## Installation

### Using Docker

You can run KVAdminer-RS using Docker.

```sh
docker run --rm -d -p 8080:8080 --name kvadminer-rs n0tlu5/kvadminer-rs:latest
```

## Usage

Once the application is running, you can access it at http://localhost:8080.

### Connect to Redis

Navigate to the connect page to configure the Redis connection by providing the host, port, username, and password.

### Manage Keys

On the keys management page, you can view and manage keys with pagination. You can also search for specific keys using the search box.

### Edit Keys

Click on the edit button next to a key to update its value.

### Create Keys

Use the form at the bottom of the keys management page to create new keys.

## Development

### Building from Source

To build the application from source, ensure you have Rust installed. Then, clone the repository and run:

```sh
cargo build --release
```

## Running Locally

You can run the application locally with:

```sh
cargo run
```

## Directory Structure
- `src/`: Contains the Rust source code
- `static/`: Contains static files (HTML, CSS, JS)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the GPL-2.0 license.
