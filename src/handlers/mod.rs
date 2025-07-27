pub mod auth;
pub mod customers;
pub mod health;
pub mod users;

use axum::Router;
use sqlx::{Pool, Sqlite};

/// Configure all routes for the application
pub fn configure_routes(pool: Pool<Sqlite>) -> Router {
    use axum::routing::{get, post, head};
    use tower_http::services::ServeDir;
    
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/login", get(auth::show_login).post(auth::do_login))
        .route("/api/health", head(health::health_check))
        .nest_service("/static", ServeDir::new("static"));
    
    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/", get(customers::list_customers))
        .route("/add", get(customers::show_add_form).post(customers::add_customer))
        .route("/customer/:id", get(customers::view_customer))
        .route("/delete/:id", post(customers::delete_customer))
        .route("/edit/:id", get(customers::show_edit_form).post(customers::update_customer))
        .route("/logout", post(auth::logout))
        // Admin only routes
        .route("/users", get(users::list_users))
        .route("/users/add", get(users::show_add_user_form).post(users::add_user))
        .route("/users/delete/:id", post(users::delete_user))
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            crate::middleware::auth_middleware,
        ));
    
    // Combine routes
    public_routes
        .merge(protected_routes)
        .with_state(pool)
}