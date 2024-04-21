# Rusty-Swag

## Description
`rusty-swag` is a high-performance command-line interface (CLI) tool designed to detect the presence of Swagger UI documentation on a list of specified hosts. Built with Rust and leveraging asynchronous programming, `rusty-swag` efficiently checks multiple hosts concurrently for Swagger UI endpoints, enhancing productivity and response times for developers and systems administrators managing API documentation across various servers.

## Features
- **Concurrent Requests**: Utilizes a configurable number of worker tasks to make simultaneous HTTP requests, improving the scanning throughput.
- **Timeout Management**: Implements request timeouts to avoid hanging operations due to unresponsive servers.
- **Error Handling**: Gracefully handles network-related errors and logs them without causing the program to crash.
- **Result Logging**: Successfully identified Swagger UI endpoints are logged into a file for further inspection.

## Installation

### Prerequisites
- Rust Programming Language: Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://rust-lang.org).
- Cargo (Rust's package manager and build tool): Comes with Rust installation.

### Build
Follow these steps to build the `rusty-swag` tool:

1. Clone the repository:
   ```bash
   git clone https://github.com/copyleftdev/rusty-swag.git
   ```
2. Change into the project directory:
   ```bash
   cd rusty-swag
   ```
3. Compile the project using Cargo:
   ```bash
   cargo build --release
   ```
   This command compiles the code and outputs an executable in the `./target/release/` directory.

## Usage
To use `rusty-swag`, navigate to the `./target/release/` directory where the executable is located. You can run the executable directly from this location or add it to your system path for easier access.

### Running the Tool
Use the following command syntax to run `rusty-swag`:
```bash
./rusty-swag --hosts <path_to_hosts_file> --routefile <path_to_routes_file> [options]
```

### Command-Line Options
- `--hosts <path_to_hosts_file>`: Specifies the path to the file containing newline-separated host URLs.
- `--routefile <path_to_routes_file>`: Specifies the path to the file containing the relative paths to check for Swagger UI on each host.
- `--workers <number>`: (Optional) Specifies the number of concurrent workers to use for sending requests. Default is 10.

### Example Command
```bash
./rusty-swag --hosts hosts.txt --routefile routes.txt --workers 20
```

## Contributing
Contributions to `rusty-swag` are welcome! Whether it's bug reports, feature suggestions, or code contributions, please feel free to make your mark. Follow the steps below to contribute:

1. Fork the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/<your-github-username>/rusty-swag.git
   ```
3. Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b your-branch-name
   ```
4. Make your changes and commit them:
   ```bash
   git commit -am "Add some feature"
   ```
5. Push the changes to your GitHub fork:
   ```bash
   git push origin your-branch-name
   ```
6. Submit a pull request from your fork’s branch to our repository.

## License
`rusty-swag` is available under the MIT License. See the LICENSE file for more info.
