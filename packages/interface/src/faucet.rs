use core::fmt;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::types::RangeOrder;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum Action {
    Mint,
    Burn,
}

impl Action {
    pub const VALUES: [Action; 2] = [Action::Mint, Action::Burn];
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Mint => write!(f, "mint"),
            Action::Burn => write!(f, "burn"),
        }
    }
}

#[cw_serde]
pub enum TokenCreationConfig {
    Managed { admin: String },
    Unmanaged {},
}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        denom: String,
        config: TokenCreationConfig,
    },

    Mint {
        denom: String,
        amount: Uint128,
    },
    Burn {
        denom: String,
    },

    Grant {
        denom: String,
        grantee: String,
        action: Action,
    },
    Revoke {
        denom: String,
        revokee: String,
        action: Action,
    },
    Release {
        denom: String,
        action: Action,
    },
    Block {
        denom: String,
        action: Action,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TokenResponse)]
    Token { denom: String },

    #[returns(TokensResponse)]
    Tokens {
        start_after: Option<u64>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(LastTokenIdResponse)]
    LastTokenId {},

    #[returns(RoleResponse)]
    Role { denom: String, account: String },

    #[returns(RolesResponse)]
    Roles {
        denom: String,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },
}

#[cw_serde]
pub struct TokenResponse {
    pub id: u64,
    pub denom_v: String,
    pub denom_r: String,
    pub config: TokenCreationConfig,
}

#[cw_serde]
pub struct TokensResponse(pub Vec<TokenResponse>);

#[cw_serde]
pub struct LastTokenIdResponse(pub u64);

#[cw_serde]
pub struct RoleResponse {
    pub denom: String,
    pub account: String,
    pub roles: Vec<(Action, bool)>,
}

#[cw_serde]
pub struct RolesResponse(pub Vec<(String, String, bool)>);

#[cw_serde]
pub struct MigrateMsg {}
