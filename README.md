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

## Configuration

### Database Connection

The application is configured to connect to a MongoDB database. It attempts to read the `MONGO_URI` environment variable to establish a connection. If `MONGO_URI` is not set, it defaults to connecting to a local MongoDB instance.

For local development, you can change the default connection string in the `src/db.rs` file. Just make sure not to commit any sensitive information!

Otherwise, to set an environment variable for the MongoDB URI, follow the instructions below.

#### Setting the `MONGO_URI`

To connect to your MongoDB instance, you need to set the MONGO_URI environment variable to your database’s URI string. Here's how you can do it:

**On UNIX/Linux/MacOS:**
Open your terminal and enter the following command (replacing `<your_mongo_uri>` with your actual MongoDB URI):

```bash
export MONGO_URI=<your_mongo_uri>
```

**On Windows:**

Open Command Prompt and execute the following command (again, replace `<your_mongo_uri>` with your MongoDB URI):

```cmd
set MONGO_URI=<your_mongo_uri>
```

## Usage

The application exposes several HTTP endpoints:

- `GET /`: Returns a simple greeting. Used to check if the server is running.
- `POST /owner`: Creates a new owner.
- `POST /dog`: Adds a new dog, associating it with an owner.
- `POST /booking`: Creates a new booking.
- `GET /bookings`: Retrieves all bookings.
- `PUT /booking/{id}/cancel`: Cancels a specific booking.

You can use tools like Postman or cURL to interact with the API.
