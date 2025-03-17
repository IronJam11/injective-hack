#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use std::str::FromStr;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Timestamp, Env, MessageInfo, Response, StdResult, Uint128, Addr};
use cw_storage_plus::Bound;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, ClaimResponse, OrganizationResponse, TotalCarbonCreditsResponse, ClaimsResponse, OrganizationListItem,OrganizationsResponse};
use crate::state::{Config, CONFIG, CLAIMS, VOTES, CLAIM_COUNTER, ORGANIZATIONS, Claim, ClaimStatus, OrganizationInfo, VoteOption};
use zero_knowledge_proofs::eligibility_proof;
use std::convert::TryFrom;
use cosmwasm_std::StdError;
use cw_storage_plus::Map;
use hex;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
        voting_period: msg.voting_period,
        total_carbon_credits: Uint128::zero(),
    };
    CONFIG.save(deps.storage, &config)?;
    CLAIM_COUNTER.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("voting_period", msg.voting_period.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateClaim { longitudes, latitudes, time_started, time_ended, demanded_tokens, ipfs_hashes } => {
            execute_create_claim(deps, env, info, longitudes, latitudes, time_started, time_ended, demanded_tokens, ipfs_hashes)
        },
        ExecuteMsg::CastVote { claim_id, vote } => {
            execute_cast_vote(deps, env, info, claim_id, vote)
        },
        ExecuteMsg::FinalizeVoting { claim_id } => {
            execute_finalize_voting(deps, env, info, claim_id)
        },
        ExecuteMsg::LendTokens { borrower, amount } => {
            execute_lend_tokens(deps, env, info, borrower, amount)
        },
        ExecuteMsg::RepayTokens { lender, amount } => {
            execute_repay_tokens(deps, env, info, lender, amount)
        },
        ExecuteMsg::VerifyEligibility { borrower, amount, lender} => {
            execute_verify_eligibility(deps, env, info, borrower,lender,amount)
        },
        ExecuteMsg::UpdateOrganizationName { name } => {
            execute_update_organization_name(deps, env, info, name)
        },
        ExecuteMsg::AddOrganizationEmission { emissions } => {
            add_organization_emission(deps, env, info, emissions) // Add this handler
        }
    }
}

pub fn execute_create_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    longitudes: Vec<String>,
    latitudes: Vec<String>,
    time_started: u64,
    time_ended: u64,
    demanded_tokens: Uint128,
    ipfs_hashes: Vec<String>,
) -> Result<Response, ContractError> {
    let mut claim_counter = CLAIM_COUNTER.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;
    let claim = Claim {
        id: claim_counter,
        organization: info.sender.clone(),
        longitudes,
        latitudes,
        time_started,
        time_ended,
        demanded_tokens,
        ipfs_hashes,
        status: ClaimStatus::Active,
        voting_end_time: env.block.time.seconds() + config.voting_period,
        yes_votes: Uint128::zero(),
        no_votes: Uint128::zero(),
    };
    CLAIMS.save(deps.storage, claim_counter, &claim)?;
    claim_counter += 1;
    CLAIM_COUNTER.save(deps.storage, &claim_counter)?;
    
    Ok(Response::new()
        .add_attribute("method", "create_claim")
        .add_attribute("claim_id", claim_counter.to_string())
        .add_attribute("organization", info.sender)
        .add_attribute("voting_end_time", claim.voting_end_time.to_string()))
}

pub fn execute_cast_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    claim_id: u64,
    vote: VoteOption,
) -> Result<Response, ContractError> {
    let mut claim = CLAIMS.load(deps.storage, claim_id)?;
    
    if env.block.time.seconds() > claim.voting_end_time {
        return Err(ContractError::VotingEnded {});
    }

    if VOTES.has(deps.storage, (claim_id, &info.sender)) {
        return Err(ContractError::AlreadyVoted {});
    }
    VOTES.save(deps.storage, (claim_id, &info.sender), &vote)?;

    match vote {
        VoteOption::Yes => claim.yes_votes += Uint128::new(1),
        VoteOption::No => claim.no_votes += Uint128::new(1),
    }
    CLAIMS.save(deps.storage, claim_id, &claim)?;
    
    Ok(Response::new()
        .add_attribute("method", "cast_vote")
        .add_attribute("claim_id", claim_id.to_string())
        .add_attribute("voter", info.sender))
}

pub fn execute_finalize_voting(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    claim_id: u64,
) -> Result<Response, ContractError> {
    let mut claim = CLAIMS.load(deps.storage, claim_id)?;
    if env.block.time.seconds() <= claim.voting_end_time {
        return Err(ContractError::VotingNotEnded {});
    }
    let approved = claim.yes_votes >= claim.no_votes;
    claim.status = if approved { ClaimStatus::Approved } else { ClaimStatus::Rejected };
    let mut config = CONFIG.load(deps.storage)?;
    
    if approved {
        let mut org_info = ORGANIZATIONS.may_load(deps.storage, &claim.organization)?
            .unwrap_or(OrganizationInfo {
                reputation_score: Uint128::zero(),
                carbon_credits: Uint128::zero(),
                debt: Uint128::zero(),
                times_borrowed: 0,
                total_borrowed: Uint128::zero(),
                total_returned: Uint128::zero(),
                name: "".to_string(),
                emissions: Uint128::zero(),
            });
        
        org_info.carbon_credits += claim.demanded_tokens;
        ORGANIZATIONS.save(deps.storage, &claim.organization, &org_info)?;
        
        config.total_carbon_credits += claim.demanded_tokens;
        CONFIG.save(deps.storage, &config)?;
    }
    let voters: Vec<(u64, Addr)> = VOTES
        .prefix(claim_id)
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|key| key.map(|addr| (claim_id, addr)))
        .collect::<Result<Vec<_>, _>>()?;
    
    for (_, voter_addr) in voters {
        let vote = VOTES.load(deps.storage, (claim_id, &voter_addr))?;
        let vote_correct = (vote == VoteOption::Yes && approved) || (vote == VoteOption::No && !approved);
        
        if vote_correct {
            let mut org_info = ORGANIZATIONS.may_load(deps.storage, &voter_addr)?
                .unwrap_or(OrganizationInfo {
                    reputation_score: Uint128::zero(),
                    carbon_credits: Uint128::zero(),
                    debt: Uint128::zero(),
                    times_borrowed: 0,
                    total_borrowed: Uint128::zero(),
                    total_returned: Uint128::zero(),
                    name: "".to_string(),
                    emissions: Uint128::zero(),
                });
            org_info.reputation_score += Uint128::new(1);
            ORGANIZATIONS.save(deps.storage, &voter_addr, &org_info)?;
        }
    }
    CLAIMS.save(deps.storage, claim_id, &claim)?;
    
    Ok(Response::new()
        .add_attribute("method", "finalize_voting")
        .add_attribute("claim_id", claim_id.to_string())
        .add_attribute("status", format!("{:?}", claim.status)))
}
pub fn execute_lend_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    borrower: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut lender_info = ORGANIZATIONS.may_load(deps.storage, &info.sender)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    
    let mut borrower_info = ORGANIZATIONS.may_load(deps.storage, &borrower)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    
    if lender_info.carbon_credits < amount {
        return Err(ContractError::NotEnoughCredits {});
    }
    
    lender_info.carbon_credits -= amount;
    borrower_info.carbon_credits += amount;
    borrower_info.debt += amount;
    borrower_info.times_borrowed += 1;
    borrower_info.total_borrowed += amount;

    ORGANIZATIONS.save(deps.storage, &info.sender, &lender_info)?;
    ORGANIZATIONS.save(deps.storage, &borrower, &borrower_info)?;
    
    Ok(Response::new()
        .add_attribute("method", "lend_tokens")
        .add_attribute("lender", info.sender)
        .add_attribute("borrower", borrower)
        .add_attribute("amount", amount))
}

pub fn execute_repay_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    lender: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut borrower_info = ORGANIZATIONS.may_load(deps.storage, &info.sender)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    
    let mut lender_info = ORGANIZATIONS.may_load(deps.storage, &lender)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    

    if borrower_info.carbon_credits < amount {
        return Err(ContractError::NotEnoughCredits {});
    }
    if borrower_info.debt < amount {
        return Err(ContractError::NotEnoughCredits {});
    }

    borrower_info.carbon_credits -= amount;
    borrower_info.debt -= amount;
    borrower_info.total_returned += amount;
    lender_info.carbon_credits += amount;
    ORGANIZATIONS.save(deps.storage, &info.sender, &borrower_info)?;
    ORGANIZATIONS.save(deps.storage, &lender, &lender_info)?;
    
    Ok(Response::new()
        .add_attribute("method", "repay_tokens")
        .add_attribute("borrower", info.sender)
        .add_attribute("lender", lender)
        .add_attribute("amount", amount))
}

pub fn execute_verify_eligibility(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    borrower: Addr,
    lender: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let borrower_info = ORGANIZATIONS.may_load(deps.storage, &borrower)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    let lender_info = ORGANIZATIONS.may_load(deps.storage, &lender)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    let borrower_emissions = u32::try_from(borrower_info.emissions.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for emissions")))?;
    let borrower_returned = u32::try_from(borrower_info.total_returned.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for total_returned")))?;
    let borrower_total_borrowed = u32::try_from(borrower_info.total_borrowed.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for total_borrowed")))?;
    let borrower_debt = u32::try_from(borrower_info.debt.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for debt")))?;
    let borrower_credits = u32::try_from(borrower_info.carbon_credits.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for carbon_credits")))?;
    let borrower_reputation = u32::try_from(borrower_info.reputation_score.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for reputation_score")))?;
    let lender_credits = u32::try_from(lender_info.carbon_credits.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for lender carbon_credits")))?;
    let lender_debt = u32::try_from(lender_info.debt.u128())
        .map_err(|_| ContractError::Std(StdError::generic_err("Conversion error for lender debt")))?;
    
    // Call the eligibility_proof function to get score and proof
    let (eligibility_score, proof_data) = eligibility_proof(
        borrower_emissions,
        borrower_returned,
        borrower_total_borrowed,
        borrower_debt,
        borrower_credits,
        borrower_reputation,
        lender_credits,
        lender_debt,
    );
    
    if eligibility_score <= 0 {
        return Err(ContractError::BorrowerNotEligible {});
    }
    
    let proof_hex = hex::encode(&proof_data);
    
    PROOFS.save(deps.storage, (&borrower, &lender), &proof_data)?;

    Ok(Response::new()
        .add_attribute("method", "verify_eligibility")
        .add_attribute("borrower", borrower)
        .add_attribute("lender", lender)
        .add_attribute("amount", amount)
        .add_attribute("eligibility_score", eligibility_score.to_string())
        .add_attribute("proof_hex", proof_hex))
}


pub const PROOFS: Map<(&Addr, &Addr), Vec<u8>> = Map::new("proofs");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetClaim { id } => to_binary(&query_claim(deps,_env,id)?),
        QueryMsg::GetOrganization { address } => to_binary(&query_organization(deps, address)?),
        QueryMsg::GetTotalCarbonCredits {} => to_binary(&query_total_carbon_credits(deps)?),
        QueryMsg::GetClaims { start_after, limit } => to_binary(&query_claims(deps,_env,start_after, limit)?),
        QueryMsg::GetClaimsByStatus { status, start_after, limit } => to_binary(&query_claims_by_status(deps, _env,status, start_after, limit)?),
        QueryMsg::GetAllOrganizations { start_after, limit } => to_binary(&query_all_organizations(deps, start_after, limit)?),
    }

}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        voting_period: config.voting_period,
        total_carbon_credits: config.total_carbon_credits,
    })
}

fn query_claim(deps: Deps, env: Env, id: u64) -> StdResult<ClaimResponse> {
    let claim = CLAIMS.load(deps.storage, id)?;
    
    // Convert u64 to Timestamp for comparison
    let voting_end_timestamp = Timestamp::from_seconds(claim.voting_end_time);
    
    // Check if voting has ended using env.block.time
    let (yes_votes, no_votes) = if env.block.time >= voting_end_timestamp {
        (claim.yes_votes, claim.no_votes)
    } else {
        (Uint128::zero(), Uint128::zero())
    };
    
    Ok(ClaimResponse {
        id: claim.id,
        organization: claim.organization,
        longitudes: claim.longitudes,
        latitudes: claim.latitudes,
        time_started: claim.time_started,
        time_ended: claim.time_ended,
        demanded_tokens: claim.demanded_tokens,
        ipfs_hashes: claim.ipfs_hashes,
        status: claim.status,
        voting_end_time: claim.voting_end_time,
        yes_votes,
        no_votes,
    })
}

fn query_organization(deps: Deps, address: Addr) -> StdResult<OrganizationResponse> {
    let org_info = ORGANIZATIONS.may_load(deps.storage, &address)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    
    Ok(OrganizationResponse {
        address,
        reputation_score: org_info.reputation_score,
        carbon_credits: org_info.carbon_credits,
        debt: org_info.debt,
        times_borrowed: org_info.times_borrowed,
        total_borrowed: org_info.total_borrowed,
        total_returned: org_info.total_returned,
        name: org_info.name,
        emissions: org_info.emissions,
    })
}

fn query_total_carbon_credits(deps: Deps) -> StdResult<TotalCarbonCreditsResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(TotalCarbonCreditsResponse {
        total: config.total_carbon_credits,
    })
}

fn query_claims(deps: Deps, env: Env, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ClaimsResponse> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.map(|s| Bound::exclusive(s));
    
    let claims: Vec<ClaimResponse> = CLAIMS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, claim) = item?;
            
            // Convert u64 to Timestamp for comparison
            let voting_end_timestamp = Timestamp::from_seconds(claim.voting_end_time);
            
            // Check if voting has ended using env.block.time
            let (yes_votes, no_votes) = if env.block.time >= voting_end_timestamp {
                (claim.yes_votes, claim.no_votes)
            } else {
                (Uint128::zero(), Uint128::zero())
            };
            
            Ok(ClaimResponse {
                id: claim.id,
                organization: claim.organization,
                longitudes: claim.longitudes,
                latitudes: claim.latitudes,
                time_started: claim.time_started,
                time_ended: claim.time_ended,
                demanded_tokens: claim.demanded_tokens,
                ipfs_hashes: claim.ipfs_hashes,
                status: claim.status,
                voting_end_time: claim.voting_end_time,
                yes_votes,
                no_votes,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;
    
    Ok(ClaimsResponse { claims })
}

fn query_claims_by_status(deps: Deps, env: Env, status: ClaimStatus, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ClaimsResponse> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.map(|s| Bound::exclusive(s));
    
    let claims: Vec<ClaimResponse> = CLAIMS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .filter(|item| {
            match item {
                Ok((_, claim)) => claim.status == status,
                _ => false,
            }
        })
        .take(limit)
        .map(|item| {
            let (_, claim) = item?;
            
            // Convert u64 to Timestamp for comparison
            let voting_end_timestamp = Timestamp::from_seconds(claim.voting_end_time);
            
            // Check if voting has ended using env.block.time
            let (yes_votes, no_votes) = if env.block.time >= voting_end_timestamp {
                (claim.yes_votes, claim.no_votes)
            } else {
                (Uint128::zero(), Uint128::zero())
            };
            
            Ok(ClaimResponse {
                id: claim.id,
                organization: claim.organization,
                longitudes: claim.longitudes,
                latitudes: claim.latitudes,
                time_started: claim.time_started,
                time_ended: claim.time_ended,
                demanded_tokens: claim.demanded_tokens,
                ipfs_hashes: claim.ipfs_hashes,
                status: claim.status,
                voting_end_time: claim.voting_end_time,
                yes_votes,
                no_votes,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;
    
    Ok(ClaimsResponse { claims })
}
pub fn execute_update_organization_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    // Load organization info
    let mut org_info = ORGANIZATIONS.may_load(deps.storage, &info.sender)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    
    // Update organization name
    org_info.name = name.clone();
    
    // Save updated organization info
    ORGANIZATIONS.save(deps.storage, &info.sender, &org_info)?;
    
    Ok(Response::new()
        .add_attribute("method", "update_organization_name")
        .add_attribute("organization", info.sender)
        .add_attribute("name", name))
}
fn query_all_organizations(deps: Deps, start_after: Option<Addr>, limit: Option<u32>) -> StdResult<OrganizationsResponse> {
    let limit = limit.unwrap_or(30) as usize;
    
    let start = match start_after {
        Some(addr) => Some(Bound::ExclusiveRaw(addr.to_string().into())),
        None => None,
    };
    
    let organizations: Vec<OrganizationListItem> = ORGANIZATIONS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .map(|item| {
            let (addr, org_info) = item?;
            Ok(OrganizationListItem {
                address: addr,
                name: org_info.name,
                reputation_score: org_info.reputation_score,
                carbon_credits: org_info.carbon_credits
            })
        })
        .collect::<StdResult<Vec<_>>>()?;
    
    Ok(OrganizationsResponse { organizations })
    
}
pub fn add_organization_emission(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    emissions: String,
) -> Result<Response, ContractError> {
    let mut org_info = ORGANIZATIONS.may_load(deps.storage, &info.sender)?
        .unwrap_or(OrganizationInfo {
            reputation_score: Uint128::zero(),
            carbon_credits: Uint128::zero(),
            debt: Uint128::zero(),
            times_borrowed: 0,
            total_borrowed: Uint128::zero(),
            total_returned: Uint128::zero(),
            name: "".to_string(),
            emissions: Uint128::zero(),
        });
    let new_emissions = Uint128::from_str(&emissions)?;
    org_info.emissions = org_info.emissions.checked_add(new_emissions)?;
    ORGANIZATIONS.save(deps.storage, &info.sender, &org_info)?;

    Ok(Response::new()
        .add_attribute("method", "add_organization_emission")
        .add_attribute("organization", info.sender)
        .add_attribute("emissions_added", emissions))
}