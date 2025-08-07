use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use parsidate::ParsiDate;

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
    
    pub fn transaction_type_display_name(&self) -> String {
        TransactionType::from_str(&self.transaction_type)
            .display_name()
            .to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionForm {
    pub amount: f64,
    pub transaction_type: String,
    pub description: Option<String>,
    pub transaction_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Cash,
    CardTransfer,
    Cheque,
    Credit,
    Other,
}

impl TransactionType {
    pub fn all() -> Vec<TransactionType> {
        vec![
            TransactionType::Cash,
            TransactionType::CardTransfer,
            TransactionType::Cheque,
            TransactionType::Credit,
            TransactionType::Other,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Cash => "Cash",
            TransactionType::CardTransfer => "CardTransfer",
            TransactionType::Cheque => "Cheque",
            TransactionType::Credit => "Credit",
            TransactionType::Other => "Other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Cash" => TransactionType::Cash,
            "CardTransfer" => TransactionType::CardTransfer,
            "Cheque" => TransactionType::Cheque,
            "Credit" => TransactionType::Credit,
            _ => TransactionType::Other,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TransactionType::Cash => "نقدی",
            TransactionType::CardTransfer => "کارت به کارت",
            TransactionType::Cheque => "چک",
            TransactionType::Credit => "اعتباری",
            TransactionType::Other => "سایر",
        }
    }
}