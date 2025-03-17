use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub voting_period: u64, // in seconds
    pub total_carbon_credits: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum VoteOption {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
    pub voter: Addr,
    pub vote: VoteOption,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ClaimStatus {
    Active,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum LentStatus {
    Active, 
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Claim {
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
pub struct LendRequest {
    pub id: u64,
    pub borrower: Addr,
    pub lender: Addr,
    pub amount: Uint128,
    pub eligibility_score: Uint128, 
    pub proof_data: String, 
    pub status: LentStatus,
    pub time: u64,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrganizationInfo {
    pub reputation_score: Uint128,
    pub carbon_credits: Uint128,
    pub debt: Uint128,
    pub times_borrowed: u32,
    pub total_borrowed: Uint128,
    pub total_returned: Uint128,
    pub name: String,
    pub emissions: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendRequestResponse {
    pub id: u64,
    pub borrower: Addr,
    pub lender: Addr,
    pub status: LentStatus,
    pub eligibility_score: Uint128,
    pub proof_data: String,
    pub time: u64,
    pub amount: Uint128,
    pub role: String,  // "borrower" or "lender"
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserLendRequestsResponse {
    pub lend_requests: Vec<LendRequestResponse>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const CLAIMS: Map<u64, Claim> = Map::new("claims");
pub const LEND_REQUESTS : Map<u64, LendRequest> = Map::new("lend_requests");
pub const VOTES: Map<(u64, &Addr), VoteOption> = Map::new("votes");
pub const CLAIM_COUNTER: Item<u64> = Item::new("claim_counter");
pub const ORGANIZATIONS: Map<&Addr, OrganizationInfo> = Map::new("organizations");
pub const LEND_REQUEST_COUNTER: Item<u64> = Item::new("lend_request_counter");