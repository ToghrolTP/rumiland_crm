use parsidate::ParsiDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub customer_id: i64,
    pub amount: f64,
    pub transaction_type: String,
    pub description: Option<String>,
    pub transaction_date: String,
}

impl Transaction {
    pub fn formatted_amount(&self) -> String {
        let amount_str = format!("{:.0}", self.amount);
        format!(
            "{} تومان",
            crate::utils::localization::to_persian_digits(&amount_str)
        )
    }

    pub fn formatted_date_shamsi(&self) -> String {
        if self.transaction_date.is_empty() {
            return "".to_string();
        }

        match chrono::NaiveDate::parse_from_str(&self.transaction_date, "%Y-%m-%d %H:%M:%S") {
            Ok(gregorian_date) => ParsiDate::from_gregorian(gregorian_date)
                .unwrap()
                .format("%Y/%m/%d")
                .to_string(),
            Err(_) => self.transaction_date.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionForm {
    pub amount: f64,
    pub transaction_type: String,
    pub description: Option<String>,
    pub transaction_date: String,
}
