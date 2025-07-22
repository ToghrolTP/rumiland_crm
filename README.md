# Rumiland CRM 🏢

A simple, modern CRM (Customer Relationship Management) web application built with Rust, featuring a Persian/RTL interface and beautiful Gruvbox Dark theme.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)

## Features ✨

- **Full CRUD Operations**: Create, Read, Update, and Delete customers
- **User Authentication**: Secure login system with sessions
- **Role-Based Access**: Admin and regular user roles
- **Persian/RTL Support**: Complete right-to-left interface in Persian
- **Modern UI**: Clean, minimal design with Gruvbox Dark theme
- **Responsive Design**: Works perfectly on desktop and mobile devices
- **Fast & Lightweight**: Built with Rust and SQLite for optimal performance
- **Server-Side Rendering**: Using Askama templating engine

## Tech Stack 🛠️

- **Backend**: Rust with Axum web framework
- **Database**: SQLite with sqlx
- **Templating**: Askama
- **Styling**: Custom CSS with Gruvbox color scheme
- **Font**: Vazirmatn for beautiful Persian typography

## Prerequisites 📋

- Rust 1.70 or higher
- Cargo (comes with Rust)

## Installation 🚀

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

4. Open your browser and navigate to:
```
http://127.0.0.1:3000
```

## Project Structure 📁

```
rumiland_crm/
├── src/
│   └── main.rs          # Application entry point and routes
├── templates/           # HTML templates
│   ├── base.html       # Base template for consistent layout
│   ├── list.html       # Customer list page
│   ├── add.html        # Add customer form
│   ├── detail.html     # Customer detail view
│   └── edit.html       # Edit customer form
├── static/
│   └── css/
│       └── styles.css  # Gruvbox theme and styles
├── Cargo.toml          # Rust dependencies
├── askama.toml         # Template engine configuration
└── rumiland.db         # SQLite database (created on first run)
```

## Usage 💡

### Default Login
On first run, a default admin account is created:
- **Username**: admin
- **Password**: admin123

⚠️ **Important**: Change the default password after first login!

### User Management (Admin Only)
1. Click "کاربران" (Users) in the navigation
2. Add new users with different roles:
   - **Admin**: Full access to system and user management
   - **User**: Can manage customers only

### Adding a Customer
1. Click "افزودن مشتری جدید" (Add New Customer) button
2. Fill in the required fields
3. Click "ذخیره مشتری" (Save Customer)

### Viewing Customers
- All customers are displayed on the home page
- Click "مشاهده" (View) to see detailed information

### Editing a Customer
1. From the detail view, click "ویرایش" (Edit)
2. Update the information
3. Click "ذخیره تغییرات" (Save Changes)

### Deleting a Customer
1. From the detail view, click "حذف" (Delete)
2. Confirm the deletion in the popup dialog

## Development 🔧

To run in development mode with auto-reload:

```bash
cargo watch -x run
```

## License 📄

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments 🙏

- Built with love using Rust and its amazing ecosystem
- Gruvbox color scheme by [morhetz](https://github.com/morhetz/gruvbox)
- Vazirmatn font by [rastikerdar](https://github.com/rastikerdar)