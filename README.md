# My Rusty Web

A proof-of-concept RESTful web service built with Rust, Actix-web, and SQLite. This application provides a simple API for managing routes.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd my-rusty-web
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Running the Application

Run the application in development mode:

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080` by default.

You should see the following output:
```
Server running at http://127.0.0.1:8080
```

The application will automatically create and initialize the SQLite database file (`routes.db`) if it doesn't exist.

## API Endpoints

The application provides the following RESTful endpoints:

### Routes API

| Method | Endpoint       | Description                 | Request Body                                                | Response                      |
|--------|----------------|-----------------------------|-------------------------------------------------------------|-------------------------------|
| POST   | /routes        | Create a new route          | `{"name": "string", "path": "string", "is_enabled": bool, "method": "string"}` | 201 Created with route ID     |
| GET    | /routes        | Get all routes              | None                                                        | 200 OK with array of routes   |
| GET    | /routes/{id}   | Get a specific route by ID  | None                                                        | 200 OK with route details     |
| PUT    | /routes/{id}   | Update a specific route     | `{"name": "string", "path": "string", "is_enabled": bool, "method": "string"}` | 200 OK or 404 Not Found       |
| DELETE | /routes/{id}   | Delete a specific route     | None                                                        | 200 OK or 404 Not Found       |

### Example Route Object

```json
{
  "id": 1,
  "name": "Get Users",
  "path": "/api/users",
  "is_enabled": true,
  "method": "GET"
}
```

## Configuration

The application uses a simple configuration system with default values:

- Database URL: `routes.db` (SQLite file in the project root)
- Server Address: `127.0.0.1:8080`

These values are currently hardcoded in the application. To modify them, you would need to change the default values in `src/config/config.rs`.

## Development

### Project Structure

- `src/main.rs` - Application entry point and server setup
- `src/config/` - Configuration handling
- `src/db/` - Database connection and operations
- `src/models/` - Data models
- `src/handlers/` - API endpoint handlers
- `src/errors/` - Error handling

### Building for Production

To build the application in release mode:

```bash
cargo build --release
```

The optimized binary will be available at `target/release/my-rusty-web`.
