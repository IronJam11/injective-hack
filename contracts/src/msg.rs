use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{ClaimStatus, OrganizationInfo, VoteOption};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub voting_period: u64, // in seconds
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateClaim {
        longitudes: Vec<String>,
        latitudes: Vec<String>,
        time_started: u64,
        time_ended: u64,
        demanded_tokens: Uint128,
        ipfs_hashes: Vec<String>,
    },
    CastVote {
        claim_id: u64,
        vote: VoteOption,
    },
    FinalizeVoting {
        claim_id: u64,
    },
    LendTokens {
        borrower: Addr,
        amount: Uint128,
    },
    RepayTokens {
        lender: Addr,
        amount: Uint128,
    },
    // ZK verification will be called from here
    VerifyEligibility {
        borrower: Addr,
        amount: Uint128,
        proof: String, // ZK proof
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetAllOrganizations { start_after: Option<String>, limit: Option<u32> },
    GetConfig {},
    GetClaim {
        id: u64,
    },
    GetOrganization {
        address: Addr,
    },
    GetTotalCarbonCredits {},
    GetClaims {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    GetClaimsByStatus {
        status: ClaimStatus,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: Addr,
    pub voting_period: u64,
    pub total_carbon_credits: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimResponse {
    pub id: u64,
    pub organization: Addr,
    pub longitudes: Vec<String>,
    pub latitudes: Vec<String>,
    pub time_started: u64,
    pub time_ended: u64,
    pub demanded_tokens: Uint128,
    pub ipfs_hashes: Vec<String>,
    pub status: ClaimStatus,
    pub voting_end_time: u64,
    pub yes_votes: Uint128,
    pub no_votes: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrganizationResponse {
    pub address: Addr,
    pub reputation_score: Uint128,
    pub carbon_credits: Uint128,
    pub debt: Uint128,
    pub times_borrowed: u32,
    pub total_borrowed: Uint128,
    pub total_returned: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalCarbonCreditsResponse {
    pub total: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimsResponse {
    pub claims: Vec<ClaimResponse>,
}
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct OrganizationsResponse {
//     pub organizations: Vec<OrganizationResponse>,
// }