# rust-web-server-tutorial

This project is features an HTTP server written in Rust using the Actix-Web framework. It provides a booking management system with endpoints for managing owners, dogs, and bookings.

For a detailed tutorial on how to build this project, check out my [Rust Web Server Tutorial](https://bretcameron.medium.com/) on Medium.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. You can check by running:

```bash
rustc --version
cargo --version
```

If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/BretCameron/rust-web-server-tutorial
```

Navigate into the project directory. For project development, you can use the following command:

```bash
cargo watch -c -w src -x run
```

The server will start running on http://127.0.0.1:5001.

## Usage

The application exposes several HTTP endpoints:

- `GET /`: Returns a simple greeting. Used to check if the server is running.
- `POST /owner`: Creates a new owner.
- `POST /dog`: Adds a new dog, associating it with an owner.
- `POST /booking`: Creates a new booking.
- `GET /bookings`: Retrieves all bookings.
- `PUT /booking/{id}/cancel`: Cancels a specific booking.

You can use tools like Postman or cURL to interact with the API.
