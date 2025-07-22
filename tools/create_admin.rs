use bcrypt::{hash, DEFAULT_COST};
use std::io::{self, Write};

fn main() {
    println!("=== Rumiland CRM Admin User Creator ===\n");
    
    // Get username
    print!("Enter admin username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    
    // Get password
    print!("Enter admin password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    // Get full name
    print!("Enter full name (e.g., مدیر سیستم): ");
    io::stdout().flush().unwrap();
    let mut full_name = String::new();
    io::stdin().read_line(&mut full_name).unwrap();
    let full_name = full_name.trim();
    
    // Generate password hash
    println!("\nGenerating password hash...");
    let password_hash = hash(password, DEFAULT_COST).unwrap();
    
    // Generate SQL command
    println!("\n✅ Password hash generated successfully!");
    println!("\nRun this SQL command to create your admin user:\n");
    
    println!("sqlite3 rumiland.db << 'EOF'");
    println!("DELETE FROM users WHERE username = '{}';", username);
    println!("INSERT INTO users (username, password_hash, full_name, role, created_at)");
    println!("VALUES (");
    println!("    '{}',", username);
    println!("    '{}',", password_hash);
    println!("    '{}',", full_name);
    println!("    'admin',");
    println!("    datetime('now')");
    println!(");");
    println!("EOF");
    
    println!("\n✨ Done! Copy and run the SQL command above.");
}