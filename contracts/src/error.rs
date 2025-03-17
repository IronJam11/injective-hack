use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::OverflowError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unauthorized")]
    InvalidResponse{},

    #[error("RequestNotActive")]
    RequestNotActive{},

    #[error("Request not found")]
    RequestNotFound {},

    #[error("Claim not found")]
    ClaimNotFound {},

    #[error("Voting period has ended")]
    VotingEnded {},

    #[error("Voting period not ended yet")]
    VotingNotEnded {},

    #[error("Already voted")]
    AlreadyVoted {},

    #[error("Invalid proof")]
    InvalidProof {},

    #[error("Not enough carbon credits")]
    NotEnoughCredits {},

    #[error("Not enough reputation")]
    NotEnoughReputation {},

    #[error("Borrower not eligible")]
    BorrowerNotEligible {},

    #[error("Overflow: {0}")]
    Overflow(#[from] OverflowError),
}