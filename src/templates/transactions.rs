use crate::models::{ Customer, Transaction, TransactionType, User };
use askama::Template;

#[derive(Template)]
#[template(path = "add_transaction.html")]
pub struct AddTransactionTemplate {
    pub customer: Customer,
    pub current_user: Option<User>,
    pub active_page: &'static str,
    pub transaction_types: Vec<TransactionType>,
}

#[derive(Template)]
#[template(path = "edit_transaction.html")]
pub struct EditTransactionTemplate {
    pub customer: Customer,
    pub transaction: Transaction,
    pub current_user: Option<User>,
    pub active_page: &'static str,
    pub transaction_types: Vec<TransactionType>,
}