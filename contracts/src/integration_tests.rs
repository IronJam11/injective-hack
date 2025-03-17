#[cfg(test)]

mod tests {
    use cosmwasm_std::{Addr, Uint128, testing::{mock_dependencies, mock_env, mock_info}, from_binary};
    use crate::{contract::{instantiate, execute, query}, msg::{InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse, ClaimResponse,OrganizationsResponse,OrganizationResponse, TotalCarbonCreditsResponse, ClaimsResponse}, state::{VoteOption, ClaimStatus}};
    use cosmwasm_std::coins;
    use crate::state::ORGANIZATIONS;
    use crate::contract::add_organization_emission;
    use crate::ContractError;
    use cosmwasm_std::OverflowError;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        
        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let config_query = QueryMsg::GetConfig {};
        let config_res: ConfigResponse = from_binary(&query(deps.as_ref(), mock_env(), config_query).unwrap()).unwrap();
        assert_eq!(config_res.owner, Addr::unchecked("creator"));
        assert_eq!(config_res.voting_period, 86400);
        assert_eq!(config_res.total_carbon_credits, Uint128::zero());
    }
    
    #[test]
    fn create_claim_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        
        let res = execute(deps.as_mut(), env.clone(), info, create_claim_msg).unwrap();
        assert_eq!(4, res.attributes.len());
        let claim_query = QueryMsg::GetClaim { id: 0 };
        let claim_res: ClaimResponse = from_binary(&query(deps.as_ref(), env.clone(), claim_query).unwrap()).unwrap();
        
        assert_eq!(claim_res.id, 0);
        assert_eq!(claim_res.organization, Addr::unchecked("creator"));
        assert_eq!(claim_res.demanded_tokens, Uint128::new(100));
        assert_eq!(claim_res.status, ClaimStatus::Active);
        assert_eq!(claim_res.yes_votes, Uint128::zero());
        assert_eq!(claim_res.no_votes, Uint128::zero());
    }
    
    #[test]
    fn voting_works() {
        let mut deps = mock_dependencies();
        let mut env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Create a claim
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        
        execute(deps.as_mut(), env.clone(), info, create_claim_msg).unwrap();
        
        // Cast votes
        let voter1_info = mock_info("voter1", &[]);
        let vote1_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), voter1_info, vote1_msg).unwrap();
        
        let voter2_info = mock_info("voter2", &[]);
        let vote2_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::No,
        };
        execute(deps.as_mut(), env.clone(), voter2_info, vote2_msg).unwrap();
        
        let voter3_info = mock_info("voter3", &[]);
        let vote3_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), voter3_info, vote3_msg).unwrap();
        let claim_query = QueryMsg::GetClaim { id: 0 };
        let claim_res: ClaimResponse = from_binary(&query(deps.as_ref(), env.clone(), claim_query).unwrap()).unwrap();
        env.block.time = env.block.time.plus_seconds(86401);
        
        
        let finalize_msg = ExecuteMsg::FinalizeVoting {
            claim_id: 0,
        };
        let finalize_info = mock_info("anyone", &[]);
        execute(deps.as_mut(), env.clone(), finalize_info, finalize_msg).unwrap();
        let claim_query = QueryMsg::GetClaim { id: 0 };
        let claim_res: ClaimResponse = from_binary(&query(deps.as_ref(), env.clone(), claim_query).unwrap()).unwrap();
        assert_eq!(claim_res.yes_votes, Uint128::new(2));
        assert_eq!(claim_res.no_votes, Uint128::new(1));
        
        assert_eq!(claim_res.status, ClaimStatus::Approved);
        let total_query = QueryMsg::GetTotalCarbonCredits {};
        let total_res: TotalCarbonCreditsResponse = from_binary(&query(deps.as_ref(), env.clone(), total_query).unwrap()).unwrap();
        
        assert_eq!(total_res.total, Uint128::new(100));
        let org_query = QueryMsg::GetOrganization { address: Addr::unchecked("creator") };
        let org_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env.clone(), org_query).unwrap()).unwrap();
        
        assert_eq!(org_res.carbon_credits, Uint128::new(100));
        
        let voter1_query = QueryMsg::GetOrganization { address: Addr::unchecked("voter1") };
        let voter1_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env.clone(), voter1_query).unwrap()).unwrap();
        
        assert_eq!(voter1_res.reputation_score, Uint128::new(1));
        
        let voter2_query = QueryMsg::GetOrganization { address: Addr::unchecked("voter2") };
        let voter2_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env.clone(), voter2_query).unwrap()).unwrap();
        
        assert_eq!(voter2_res.reputation_score, Uint128::zero());
    }
    
    #[test]
    fn lending_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator_info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        instantiate(deps.as_mut(), env.clone(), creator_info.clone(), msg).unwrap();
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        
        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_claim_msg).unwrap();
        let voter_info = mock_info("voter", &[]);
        let vote_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), voter_info, vote_msg).unwrap();
        let mut env2 = env.clone();
        env2.block.time = env.block.time.plus_seconds(86401);
        
        let finalize_msg = ExecuteMsg::FinalizeVoting {
            claim_id: 0,
        };
        execute(deps.as_mut(), env2.clone(), creator_info.clone(), finalize_msg).unwrap();
        let lend_msg = ExecuteMsg::LendTokens {
            borrower: Addr::unchecked("borrower"),
            amount: Uint128::new(50),
        };
        execute(deps.as_mut(), env2.clone(), creator_info.clone(), lend_msg).unwrap();
        let creator_query = QueryMsg::GetOrganization { address: Addr::unchecked("creator") };
        let creator_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env2.clone(), creator_query).unwrap()).unwrap();
        
        assert_eq!(creator_res.carbon_credits, Uint128::new(50));
        let borrower_query = QueryMsg::GetOrganization { address: Addr::unchecked("borrower") };
        let borrower_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env2.clone(), borrower_query).unwrap()).unwrap();
        
        assert_eq!(borrower_res.carbon_credits, Uint128::new(50));
        assert_eq!(borrower_res.debt, Uint128::new(50));
        assert_eq!(borrower_res.times_borrowed, 1);
        assert_eq!(borrower_res.total_borrowed, Uint128::new(50));
        let borrower_info = mock_info("borrower", &[]);
        let repay_msg = ExecuteMsg::RepayTokens {
            lender: Addr::unchecked("creator"),
            amount: Uint128::new(30),
        };
        execute(deps.as_mut(), env2.clone(), borrower_info, repay_msg).unwrap();
        let creator_query = QueryMsg::GetOrganization { address: Addr::unchecked("creator") };
        let creator_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env2.clone(), creator_query).unwrap()).unwrap();
        
        assert_eq!(creator_res.carbon_credits, Uint128::new(80));
        
        let borrower_query = QueryMsg::GetOrganization { address: Addr::unchecked("borrower") };
        let borrower_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env2.clone(), borrower_query).unwrap()).unwrap();
        
        assert_eq!(borrower_res.carbon_credits, Uint128::new(20));
        assert_eq!(borrower_res.debt, Uint128::new(20));
        assert_eq!(borrower_res.total_returned, Uint128::new(30));
    }
    
    
    #[test]
    fn query_claims_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        let creator_info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, // 1 day
        };
        instantiate(deps.as_mut(), env.clone(), creator_info.clone(), msg).unwrap();
        for i in 0..3 {
            let create_claim_msg = ExecuteMsg::CreateClaim {
                longitudes: vec![format!("123.{}", i)],
                latitudes: vec![format!("78.{}", i)],
                time_started: 1000 + i,
                time_ended: 2000 + i,
                demanded_tokens: Uint128::new(100 + i as u128),
                ipfs_hashes: vec![format!("QmHash{}", i)],
            };
            
            execute(deps.as_mut(), env.clone(), creator_info.clone(), create_claim_msg).unwrap();
        }
        
        // Query all claims
        let claims_query = QueryMsg::GetClaims { start_after: None, limit: None };
        let claims_res: ClaimsResponse = from_binary(&query(deps.as_ref(), env.clone(), claims_query).unwrap()).unwrap();
        
        assert_eq!(claims_res.claims.len(), 3);
        assert_eq!(claims_res.claims[0].id, 0);
        assert_eq!(claims_res.claims[1].id, 1);
        assert_eq!(claims_res.claims[2].id, 2);
        
        // Query with pagination
        let paginated_query = QueryMsg::GetClaims { start_after: Some(1), limit: Some(1) };
        let paginated_res: ClaimsResponse = from_binary(&query(deps.as_ref(), env.clone(), paginated_query).unwrap()).unwrap();
        assert_eq!(paginated_res.claims.len(), 1);
        assert_eq!(paginated_res.claims[0].id, 2);
        // Approve one claim
        let vote_msg = ExecuteMsg::CastVote {
            claim_id: 1,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), creator_info.clone(), vote_msg).unwrap();
        
        let mut env2 = env.clone();
        env2.block.time = env.block.time.plus_seconds(86401);
        
        let finalize_msg = ExecuteMsg::FinalizeVoting {
            claim_id: 1,
        };
        execute(deps.as_mut(), env2.clone(), creator_info, finalize_msg).unwrap();
        
        // Query by status
        let status_query = QueryMsg::GetClaimsByStatus { status: ClaimStatus::Approved, start_after: None, limit: None };
        let status_res: ClaimsResponse = from_binary(&query(deps.as_ref(), env2.clone(), status_query).unwrap()).unwrap();
        
        assert_eq!(status_res.claims.len(), 1);
        assert_eq!(status_res.claims[0].id, 1);
        assert_eq!(status_res.claims[0].status, ClaimStatus::Approved);
        
        let active_query = QueryMsg::GetClaimsByStatus { status: ClaimStatus::Active, start_after: None, limit: None };
        let active_res: ClaimsResponse = from_binary(&query(deps.as_ref(), env2, active_query).unwrap()).unwrap();
        
        assert_eq!(active_res.claims.len(), 2);
        assert!(active_res.claims.iter().all(|c| c.status == ClaimStatus::Active));
    }

    #[test]
    fn test_double_voting_prevention() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        execute(deps.as_mut(), env.clone(), info.clone(), create_claim_msg).unwrap();
        let voter_info = mock_info("voter1", &[]);
        let vote_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), voter_info.clone(), vote_msg).unwrap();
        let vote_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        let res = execute(deps.as_mut(), env.clone(), voter_info, vote_msg);
        assert!(res.is_err()); 
    }

    #[test]
    fn test_voting_period_enforcement() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        execute(deps.as_mut(), env.clone(), info.clone(), create_claim_msg).unwrap();
        let finalize_msg = ExecuteMsg::FinalizeVoting { claim_id: 0 };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), finalize_msg);
        assert!(res.is_err()); 
    }

    #[test]
    fn test_reputation_boost_for_correct_voters() {
        let mut deps = mock_dependencies();
        let mut env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, // 1 day
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        execute(deps.as_mut(), env.clone(), info.clone(), create_claim_msg).unwrap();

        // Cast votes
        let voter1_info = mock_info("voter1", &[]);
        let vote1_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), voter1_info.clone(), vote1_msg).unwrap();

        let voter2_info = mock_info("voter2", &[]);
        let vote2_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::No,
        };
        execute(deps.as_mut(), env.clone(), voter2_info.clone(), vote2_msg).unwrap();
        env.block.time = env.block.time.plus_seconds(86401);
        let finalize_msg = ExecuteMsg::FinalizeVoting { claim_id: 0 };
        execute(deps.as_mut(), env.clone(), info.clone(), finalize_msg).unwrap();
        let voter1_query = QueryMsg::GetOrganization { address: Addr::unchecked("voter1") };
        let voter1_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env.clone(), voter1_query).unwrap()).unwrap();
        assert_eq!(voter1_res.reputation_score, Uint128::new(1)); // Voted correctly

        let voter2_query = QueryMsg::GetOrganization { address: Addr::unchecked("voter2") };
        let voter2_res: OrganizationResponse = from_binary(&query(deps.as_ref(), env.clone(), voter2_query).unwrap()).unwrap();
        assert_eq!(voter2_res.reputation_score, Uint128::zero()); // Voted incorrectly
    }

    #[test]
    fn test_lending_edge_cases() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            voting_period: 86400, 
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let create_claim_msg = ExecuteMsg::CreateClaim {
            longitudes: vec!["123.456".to_string()],
            latitudes: vec!["78.90".to_string()],
            time_started: 1000,
            time_ended: 2000,
            demanded_tokens: Uint128::new(100),
            ipfs_hashes: vec!["QmHash1".to_string()],
        };
        execute(deps.as_mut(), env.clone(), info.clone(), create_claim_msg).unwrap();
        let voter_info = mock_info("voter", &[]);
        let vote_msg = ExecuteMsg::CastVote {
            claim_id: 0,
            vote: VoteOption::Yes,
        };
        execute(deps.as_mut(), env.clone(), voter_info, vote_msg).unwrap();
        let mut env2 = env.clone();
        env2.block.time = env.block.time.plus_seconds(86401);

        let finalize_msg = ExecuteMsg::FinalizeVoting { claim_id: 0 };
        execute(deps.as_mut(), env2.clone(), info.clone(), finalize_msg).unwrap();
        let lend_msg = ExecuteMsg::LendTokens {
            borrower: Addr::unchecked("borrower"),
            amount: Uint128::new(150),
        };
        let res = execute(deps.as_mut(), env2.clone(), info.clone(), lend_msg);
        assert!(res.is_err()); 
        let borrower_info = mock_info("borrower", &[]);
        let repay_msg = ExecuteMsg::RepayTokens {
            lender: Addr::unchecked("creator"),
            amount: Uint128::new(150),
        };
        let res = execute(deps.as_mut(), env2.clone(), borrower_info, repay_msg);
        assert!(res.is_err());
    }
    #[test]
    fn update_organization_name() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg {
            voting_period: 86400,
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();
        let query_msg = QueryMsg::GetOrganization {
            address: Addr::unchecked("creator"),
        };
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let org_response: OrganizationResponse = from_binary(&res).unwrap();
        assert_eq!(org_response.address, Addr::unchecked("creator"));
        assert_eq!(org_response.reputation_score, Uint128::zero());
        assert_eq!(org_response.carbon_credits, Uint128::zero());
        assert_eq!(org_response.debt, Uint128::zero());
        let update_name_msg = ExecuteMsg::UpdateOrganizationName {
            name: "Green Earth Foundation".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, update_name_msg).unwrap();
        let query_all_msg = QueryMsg::GetAllOrganizations {
            start_after: None,
            limit: None,
        };
        let res = query(deps.as_ref(), mock_env(), query_all_msg).unwrap();
        let orgs_response: OrganizationsResponse = from_binary(&res).unwrap();
        assert_eq!(orgs_response.organizations.len(), 1);
        assert_eq!(orgs_response.organizations[0].address, Addr::unchecked("creator"));
        assert_eq!(orgs_response.organizations[0].name, "Green Earth Foundation");
        assert_eq!(orgs_response.organizations[0].reputation_score, Uint128::zero());
    }
    
    #[test]
    fn query_all_organizations_pagination() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg {
            voting_period: 86400, 
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        let info = mock_info("creator", &coins(1000, "earth"));
        let update_name_msg = ExecuteMsg::UpdateOrganizationName {
            name: "Creator Org".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, update_name_msg).unwrap();
        let org_addresses = vec!["org1", "org2", "org3", "org4", "org5"];
        let org_names = vec!["First Org", "Second Org", "Third Org", "Fourth Org", "Fifth Org"];
        
        for i in 0..5 {
            let info = mock_info(org_addresses[i], &coins(1000, "earth"));
            let update_name_msg = ExecuteMsg::UpdateOrganizationName {
                name: org_names[i].to_string(),
            };
            let _res = execute(deps.as_mut(), mock_env(), info, update_name_msg).unwrap();
        }
        let query_all_msg = QueryMsg::GetAllOrganizations {
            start_after: None,
            limit: None,
        };
        let res = query(deps.as_ref(), mock_env(), query_all_msg).unwrap();
        let orgs_response: OrganizationsResponse = from_binary(&res).unwrap();
        assert_eq!(orgs_response.organizations.len(), 6);
        let query_page1 = QueryMsg::GetAllOrganizations {
            start_after: None,
            limit: Some(2),
        };
        let res = query(deps.as_ref(), mock_env(), query_page1).unwrap();
        let page1: OrganizationsResponse = from_binary(&res).unwrap();
        
        assert_eq!(page1.organizations.len(), 2);
        assert_eq!(page1.organizations[0].address, Addr::unchecked("creator"));
        assert_eq!(page1.organizations[1].address, Addr::unchecked("org1"));
        let query_page2 = QueryMsg::GetAllOrganizations {
            start_after: Some(Addr::unchecked("org1")),
            limit: Some(2),
        };
        let res = query(deps.as_ref(), mock_env(), query_page2).unwrap();
        let page2: OrganizationsResponse = from_binary(&res).unwrap();
        
        assert_eq!(page2.organizations.len(), 2);
        assert_eq!(page2.organizations[0].address, Addr::unchecked("org2"));
        assert_eq!(page2.organizations[1].address, Addr::unchecked("org3"));
        let query_page3 = QueryMsg::GetAllOrganizations {
            start_after: Some(Addr::unchecked("org3")),
            limit: Some(2),
        };
        let res = query(deps.as_ref(), mock_env(), query_page3).unwrap();
        let page3: OrganizationsResponse = from_binary(&res).unwrap();
        
        assert_eq!(page3.organizations.len(), 2);
        assert_eq!(page3.organizations[0].address, Addr::unchecked("org4"));
        assert_eq!(page3.organizations[1].address, Addr::unchecked("org5"));
    }   
}