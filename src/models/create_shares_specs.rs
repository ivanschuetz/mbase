use super::share_amount::ShareAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateSharesSpecs {
    pub token_name: String,
    pub supply: ShareAmount,
}
