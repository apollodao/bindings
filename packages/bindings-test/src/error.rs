use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid full denom '{full_denom}'")]
    InvalidFullDenom { full_denom: String },

    #[error("Invalid number of assets added to pool")]
    InvalidNumberOfAssets {},
}
