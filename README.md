# Rumiland CRM 🏢

A modern, modular CRM (Customer Relationship Management) web application built with Rust, featuring a Persian/RTL interface, role-based authentication, and a complete product and transaction management system.

## 🏗️ Architecture

This project follows a clean, modular architecture with clear separation of concerns:

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── error.rs          # Centralized error handling
├── models/           # Data models (Customer, Product, User, Transaction)
├── db/               # Database layer (connection, migrations)
├── handlers/         # HTTP request handlers (controllers)
├── middleware/       # Middleware (authentication, etc.)
├── templates/        # Askama template definitions
└── utils/            # Utility functions (validation, formatting)
```

## ✨ Features

- **Full Customer Management (CRUD)**: Create, view, update, and delete customer records.
- **Full Product Catalog (CRUD)**: A complete digital catalog to create, view, update, and delete products.
- **Transaction Tracking**: Add financial transactions (payments, credits, etc.) for each customer.
- **Secure Authentication**: Robust session-based authentication using secure cookies.
- **Role-Based Access Control (RBAC)**: Distinct "Admin" and "User" roles with different permissions.
- **Persian/RTL Support**: Native Persian interface designed for right-to-left reading.
- **Type-Safe Database Operations**: Leveraging Rust's type system with SQLx for safe and reliable database queries.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- SQLite3

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/rumiland-crm.git
    cd rumiland-crm
    ```
2.  Build the project:
    ```bash
    cargo build --release
    ```
3.  Run the application:
    ```bash
    cargo run
    ```

The application will be available at `http://localhost:3000`.

### Creating an Admin User

On the first run, a default admin is created:

- **Username**: `admin`
- **Password**: `admin123`

To create a custom admin user, you can use the interactive command-line tool:

```bash
cargo run create-admin
```

## 📁 Project Structure

### Models (`src/models/`)

- `customer.rs`: Defines the `Customer` entity and its associated forms.
- `product.rs`: Defines the `Product` entity and its forms for creating and editing.
- `transaction.rs`: Defines the `Transaction` entity, `TransactionType` enum, and associated forms.
- `user.rs`: Defines the `User` entity, user roles, and authentication forms.
- `session.rs`: Handles session management for user authentication.

### Handlers (`src/handlers/`)

- `auth.rs`: Manages user login and logout.
- `customers.rs`: Handles all CRUD operations for customers.
- `catalog.rs`: Manages all CRUD operations for the product catalog.
- `transactions.rs`: Handles adding new transactions for customers.
- `users.rs`: Manages user administration (Admin only).

## 🔒 Security

- Passwords are securely hashed using **bcrypt**.
- Sessions are stored in the database and linked via secure, HTTP-only cookies, expiring after 24 hours.
- Parameterized queries with SQLx prevent SQL injection vulnerabilities.
- Askama templates provide automatic output escaping to protect against XSS attacks.

## 📝 API Structure

### Public Routes

- `GET /login`: Renders the login page.
- `POST /login`: Authenticates user credentials and creates a session.
- `GET /static/*`: Serves static assets like CSS and JavaScript.

### Protected Routes (Login Required)

- `POST /logout`: Logs the user out and destroys the session.
- `GET /`: Displays the list of all customers.
- `GET /add`: Shows the form to add a new customer.
- `POST /add`: Creates a new customer.
- `GET /customer/:id`: Displays the detail page for a specific customer, including their transaction history.
- `POST /delete/:id`: Deletes a customer.
- `GET /edit/:id`: Shows the form to edit a customer.
- `POST /edit/:id`: Updates a customer's information.
- `GET /customer/:id/add-transaction`: Shows the form to add a transaction for a customer.
- `POST /customer/:id/add-transaction`: Creates a new transaction.

### Product Catalog Routes (Login Required)

- `GET /catalog`: Displays the product catalog grid.
- `GET /catalog/add`: Shows the form to add a new product.
- `POST /catalog/add`: Creates a new product.
- `GET /catalog/product/:id`: Displays the detail page for a single product.
- `GET /catalog/edit/:id`: Shows the form to edit a product.
- `POST /catalog/edit/:id`: Updates a product's information.
- `POST /catalog/delete/:id`: Deletes a product.

### Admin Routes (Admin Role Required)

- `GET /users`: Displays the list of all users.
- `GET /users/add`: Shows the form to add a new user.
- `POST /users/add`: Creates a new user.
- `POST /users/delete/:id`: Deletes a user.

## 🤝 Contributing

1.  Fork the repository.
2.  Create a feature branch.
3.  Commit your changes.
4.  Push to the branch.
5.  Open a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
