# Skills

Skills is a simple and powerful micro-server that allows you to execute Python code remotely. It's built in Rust using the Actix web framework. This tool is designed to be a foundational component for creating more complex AI agents and automation systems, allowing them to offload Python execution to a secure and controlled environment.

## Features

- Execute arbitrary Python code.
- Receive `stdout` and `stderr` from the executed code.
- Simple JSON-based API.

## Security Warning

**This server allows remote code execution and is highly insecure by nature. Do not expose it to untrusted networks or users. It is intended for use in controlled, isolated environments.**

## Build

To build the project, you need to have Rust and Cargo installed.

```bash
cargo build --release
```

## Run

To run the server, use the following command:

```bash
cargo run --release
```

The server will start on `127.0.0.1:8080`.

## Usage

You can send a POST request to the `/execute` endpoint with a JSON payload containing the Python code you want to execute.

### Example

Here's an example using `curl`:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"code": "print(\\"Hello from Skills!\\")"}' http://127.0.0.1:8080/execute
```

### Response

The server will respond with a JSON object containing the `stdout` and `stderr` of the executed code.

```json
{
  "stdout": "Hello from Skills!\n",
  "stderr": ""
}
```
