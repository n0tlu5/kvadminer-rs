# KVAdminer-RS


KVAdminer-RS is a simple web-based admin tool for managing keys in a Redis database. It allows you to connect to a Redis instance, view keys, create and edit keys, and delete keys.

## Description

KVAdminer-RS is built using the Actix-web framework in Rust and provides a simple web interface for interacting with a Redis database. The web interface includes pages for connecting to the database, listing all keys, editing a key, and creating or deleting keys.

## Features

- Connect to a Redis database by specifying the host, port, username, and password.
- List all keys in the Redis database with pagination.
- Create new keys with specified values.
- Edit existing keys.
- Delete keys.


## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Docker (for running Redis)
- Docker Compose

### Building


1. Clone the repository:


    ```bash
    git clone https://github.com/yourusername/kvadminer-rs.git
    cd kvadminer-rs
    ```

2. Build the project using Cargo:


    ```bash
    cargo build --release
    ```

### Running

1. Start a Redis instance using Docker Compose:

    ```bash
    docker-compose up -d
    ```

2. Run the application:


    ```bash
    cargo run --release
    ```


3. Open your web browser and navigate to `http://localhost:8080/static/index.html` to access the web interface.


### Docker Setup


Alternatively, you can run the entire setup using Docker:

1. Create a shell script named `build_and_run.sh` in the root of your project directory:

    ```sh
    #!/bin/sh


    # Install dependencies
    apt-get update && apt-get install -y \
        libssl-dev \
        pkg-config

    # Build the application
    cargo build --release

    # Run the application
    cargo run --release
    ```

2. Ensure the script is executable:

    ```bash
    chmod +x build_and_run.sh
    ```

3. Create a `docker-compose.yml` file with the following content:


    ```yaml
    version: '3.8'

    services:
      web:
        image: rust:1.79.0-slim
        ports:
          - "8080:8080"
        environment:
          - REDIS_URL=redis://redis:6379/
        depends_on:
          - redis
        volumes:
          - .:/usr/src/app
          - cargo-cache:/usr/local/cargo # Cache the Cargo build directory to speed up builds
        working_dir: /usr/src/app
        entrypoint: ["./build_and_run.sh"]


      redis:

        image: "redis:alpine"
        ports:
          - "6379:6379"

    volumes:
      cargo-cache:
    ```

4. Run the application using Docker Compose:

    ```bash
    docker-compose up --build
    ```

### Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

Please make sure your code adheres to the coding standards and includes tests for new features.


### License

This project is licensed under the MIT License.

### Acknowledgments

- [Actix-web](https://actix.rs/)
- [Redis](https://redis.io/)
- [Docker](https://www.docker.com/)


