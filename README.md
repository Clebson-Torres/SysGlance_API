# SysGlance API ✨

A simple and lightweight microservice built in Rust using `actix-web` and `sysinfo` to quickly provide essential system information about the machine it's running on.

## Features

*   Retrieves basic operating system details (Name, Kernel Version, Hostname).
*   Shows the CPU model name and current usage percentage.
*   Displays total and used RAM (in Gigabytes, rounded to one decimal place).
*   Calculates total and available disk space (in Gigabytes, rounded to one decimal place) across all mounted disks.
*   Shows the system uptime formatted into hours, minutes, and seconds.

## Endpoint

The API exposes a single endpoint:

*   **`GET /system_info`**

    Returns a JSON object containing the system information.

    **Example Response:**

    ```json
    {
      "os_name": "Ubuntu",
      "kernel_version": "5.15.0-78-generic",
      "host_name": "my-server-hostname",
      "cpu_name": "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz",
      "cpu_usage": 15.3,
      "total_memory_gb": 31.3,
      "used_memory_gb": 8.7,
      "total_disk_gb": 470.4,
      "available_disk_gb": 250.1,
      "uptime_formatted": "12h 34m 56s"
    }
    ```
    *(Values are examples and will vary depending on the system)*

## How to Run

### Prerequisites

*   Rust (including Cargo) installed. You can get it from rustup.rs.

### Steps

1.  **Clone the repository (if applicable) or navigate to the project directory:**
    ```bash
    https://github.com/Clebson-Torres/SysGlance_API.git
    cd /SysGlance_API
    ```

2.  **Build the project for release (recommended for performance):**
    ```bash
    cargo build --release
    ```

3.  **Run the server:**
    ```bash
    cargo run --release
    ```
    Alternatively, run the compiled binary directly:
    ```bash
    # The exact binary name might vary slightly based on your project structure
    ./target/release/api
    ```

4.  The server will start listening on `http://0.0.0.0:8080`.

## Accessing the API

You can query the endpoint using tools like `curl` or your web browser:

```bash
curl http://localhost:8080/system_info
