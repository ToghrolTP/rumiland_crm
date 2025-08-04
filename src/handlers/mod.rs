pub mod auth;
pub mod catalog;
pub mod customers;
pub mod transactions;
pub mod users;

use axum::Router;
use sqlx::{Pool, Sqlite};

/// Configure all routes for the application
pub fn configure_routes(pool: Pool<Sqlite>) -> Router {
    use axum::routing::{get, post};
    use tower_http::services::ServeDir;

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/login", get(auth::show_login).post(auth::do_login))
        .nest_service("/static", ServeDir::new("static"));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/", get(customers::list_customers))
        .route(
            "/add",
            get(customers::show_add_form).post(customers::add_customer),
        )
        .route("/customer/:id", get(customers::view_customer))
        .route(
            "/customer/:id/add-transaction",
            get(transactions::show_add_transaction_form).post(transactions::add_transaction),
        )
        .route("/delete/:id", post(customers::delete_customer))
        .route(
            "/edit/:id",
            get(customers::show_edit_form).post(customers::update_customer),
        )
        .route("/export/customers", get(customers::export_customer))
        .route("/logout", post(auth::logout))
        // Catalog routes
        .route("/catalog", get(catalog::show_catalog))
        .route(
            "/catalog/add",
            get(catalog::show_add_product_form).post(catalog::add_product),
        )
        // Admin only routes
        .route("/users", get(users::list_users))
        .route(
            "/users/add",
            get(users::show_add_user_form).post(users::add_user),
        )
        .route("/users/delete/:id", post(users::delete_user))
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            crate::middleware::auth_middleware,
        ));

    // Combine routes
    public_routes.merge(protected_routes).with_state(pool)
}
