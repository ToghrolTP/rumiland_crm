# Rumiland CRM 🏢

A modern, modular CRM (Customer Relationship Management) web application built with Rust, featuring Persian/RTL interface and role-based authentication.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)

## 🏗️ Architecture

This project follows a clean, modular architecture with clear separation of concerns:

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── error.rs          # Centralized error handling
├── models/           # Data models and entities
├── db/               # Database layer
├── handlers/         # HTTP request handlers
├── middleware/       # Middleware (authentication, etc.)
├── templates/        # Template definitions
└── utils/            # Utility functions
```

## ✨ Features

- **Full CRUD Operations**: Complete customer management
- **Authentication System**: Secure session-based auth
- **Role-Based Access**: Admin and regular user roles
- **Persian/RTL Support**: Native Persian interface
- **Modular Architecture**: Clean, maintainable code structure
- **Error Handling**: Centralized error management
- **Type Safety**: Leveraging Rust's type system

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- SQLite3

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rumiland-crm.git
cd rumiland-crm
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

### Configuration

The application can be configured via environment variables:

```bash
# Database URL (default: sqlite:rumiland.db?mode=rwc)
DATABASE_URL=sqlite:mydb.db

# Server host and port
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Session duration in hours (default: 24)
SESSION_DURATION_HOURS=24
```

### Creating Admin User

On first run, a default admin is created:
- Username: `admin`
- Password: `admin123`

To create a custom admin user:
```bash
cargo run create-admin
```

## 📁 Project Structure

### Models (`src/models/`)
- `customer.rs`: Customer entity and forms
- `user.rs`: User entity, roles, and authentication forms
- `session.rs`: Session management

### Database (`src/db/`)
- `connection.rs`: Database connection pool
- `migrations.rs`: SQL migrations and schema

### Handlers (`src/handlers/`)
- `auth.rs`: Login/logout handlers
- `customers.rs`: Customer CRUD operations
- `users.rs`: User management (admin only)

### Middleware (`src/middleware/`)
- `auth.rs`: Authentication middleware

### Templates (`src/templates/`)
- Template structs for Askama rendering

### Utils (`src/utils/`)
- `password.rs`: Password hashing utilities

## 🔧 Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Adding New Features

1. **Add a new model**: Create in `src/models/`
2. **Add handlers**: Create in `src/handlers/`
3. **Add routes**: Update `src/handlers/mod.rs`
4. **Add templates**: Create in `src/templates/` and `templates/`

## 🚢 Deployment

### Building for Production
```bash
cargo build --release
```

### Running in Production
```bash
DATABASE_URL=sqlite:/path/to/db.db \
SERVER_HOST=0.0.0.0 \
SERVER_PORT=80 \
./target/release/rumiland_crm
```

### Using systemd
See `deployment/rumiland.service` for systemd configuration.

## 🔒 Security

- Passwords are hashed using bcrypt
- Sessions expire after 24 hours
- SQL injection protection via parameterized queries
- XSS protection in templates
- CSRF protection via SameSite cookies

## 📝 API Structure

### Public Routes
- `GET /login` - Login page
- `POST /login` - Authentication
- `GET /static/*` - Static assets

### Protected Routes
- `GET /` - Customer list
- `GET /add` - Add customer form
- `POST /add` - Create customer
- `GET /customer/:id` - View customer
- `GET /edit/:id` - Edit form
- `POST /edit/:id` - Update customer
- `POST /delete/:id` - Delete customer
- `POST /logout` - Logout

### Admin Routes
- `GET /users` - User list
- `GET /users/add` - Add user form
- `POST /users/add` - Create user
- `POST /users/delete/:id` - Delete user

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Styled with [Gruvbox](https://github.com/morhetz/gruvbox) color scheme
- Persian font: [Vazirmatn](https://github.com/rastikerdar/vazirmatn)