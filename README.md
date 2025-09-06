# Actix JWT

A RESTful API server built with Actix Web that provides JWT (JSON Web Token) authentication functionality. The application includes token generation, validation, and protected route middleware with integrated API documentation.

## Features

- JWT token encoding and decoding
- Authentication middleware for protected routes
- Custom extractors for token validation
- Integrated API documentation using Scalar
- Request logging middleware
- Environment-based configuration

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd actix-jwt
cargo build
```

## Configuration

The application uses environment variables for configuration:

- `SECRET`: JWT secret key for token signing (defaults to "VeryGoodSecret" if not set)
- `RUST_LOG`: Log level configuration (defaults to "info")

Set these variables in your environment or create a `.env` file:

```bash
export SECRET="your-jwt-secret-key"
export RUST_LOG="info"
```

## Usage

### Starting the Server

Run the application:

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`.

### API Endpoints

#### Documentation
- `GET /` - Interactive API documentation (Scalar UI)
- `GET /openapi` - OpenAPI specification in JSON format

#### Authentication
- `GET /user/encode-token/{id}` - Generate a JWT token for the given user ID
- `POST /user/decode-token` - Decode and validate a JWT token
- `GET /user/protected` - Protected endpoint requiring valid JWT authentication

### API Usage Examples

#### Generate a Token
```bash
curl -X GET http://127.0.0.1:8080/user/encode-token/123
```

#### Decode a Token
```bash
curl -X POST http://127.0.0.1:8080/user/decode-token \
  -H "Content-Type: application/json" \
  -d '{"token": "your-jwt-token-here"}'
```

#### Access Protected Route
```bash
curl -X GET http://127.0.0.1:8080/user/protected \
  -H "Authorization: Bearer your-jwt-token-here"
```

## Project Structure

```
src/
├── main.rs                 # Application entry point and server configuration
├── openapi.json           # OpenAPI specification
├── extractors/            # Custom request extractors
│   ├── mod.rs
│   └── authentication_token.rs  # JWT token extraction logic
├── middlewares/           # Custom middleware components
│   ├── mod.rs
│   └── protect.rs         # Authentication middleware
└── scopes/               # Route organization
    ├── mod.rs
    └── user.rs           # User-related endpoints
```

## Architecture

The application follows a modular architecture:

- **Extractors**: Handle request data extraction and validation, including JWT token parsing
- **Middlewares**: Provide cross-cutting concerns like authentication and request protection
- **Scopes**: Organize related endpoints into logical groups

## Dependencies

Key dependencies include:

- `actix-web`: Web framework
- `jsonwebtoken`: JWT token handling
- `chrono`: Date and time utilities
- `serde`: Serialization and deserialization
- `scalar-doc`: API documentation generation
- `env_logger`: Logging functionality

## Development

### Building

```bash
cargo build
```

### Running in Development

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

## API Documentation

The application provides interactive API documentation accessible at the root endpoint (`/`). The documentation is powered by Scalar and automatically reflects the current API specification defined in `openapi.json`.

## Security Considerations

- JWT tokens have expiration times to limit their validity period
- The secret key should be kept secure and not exposed in production
- Use HTTPS in production environments
- Implement proper token rotation and revocation strategies for production use

## Production Deployment

For production deployment:

1. Set a strong, unique `SECRET` environment variable
2. Configure appropriate log levels
3. Use a reverse proxy (nginx, Apache) for SSL termination
4. Consider implementing token blacklisting for enhanced security
5. Monitor and log authentication attempts
