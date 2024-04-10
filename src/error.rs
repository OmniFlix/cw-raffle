use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid proxy address")]
    InvalidProxyAddress {},

    #[error("Invalid participant count")]
    InvalidParticipantCount {},

    #[error("Invalid randomness")]
    InvalidRandomness {},
}
