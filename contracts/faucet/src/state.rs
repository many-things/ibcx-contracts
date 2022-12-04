use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Item, Map};
use ibc_interface::faucet::Action;

use crate::error::ContractError;

#[cw_serde]
pub enum TokenConfig {
    Managed { admin: Addr },
    Unmanaged {},
}

#[cw_serde]
pub struct Token {
    pub id: u64,
    pub denom_v: String,
    pub denom_r: String,
    pub config: TokenConfig,
}

impl Token {
    pub fn check_role(
        &self,
        storage: &dyn Storage,
        sender: &Addr,
        action: Action,
    ) -> Result<(), ContractError> {
        if let TokenConfig::Managed { admin } = &self.config {
            if ROLES_GLOBAL
                .may_load(storage, (self.id, action.to_string()))?
                .unwrap_or_default()
            {
                return Ok(());
            }

            let has_role = ROLES
                .may_load(storage, (self.id, sender.clone(), action.to_string()))?
                .unwrap_or_default();

            if sender != admin && !has_role {
                return Err(ContractError::Unauthorized {});
            }
        }

        Ok(())
    }

    pub fn check_admin(&self, sender: &Addr) -> Result<(), ContractError> {
        if let TokenConfig::Managed { admin } = &self.config {
            if sender != admin {
                return Err(ContractError::Unauthorized {});
            }
        } else {
            return Err(ContractError::UnmanagedToken {});
        }

        Ok(())
    }
}

pub const TMP_TOKEN_DENOM: Item<String> = Item::new("tmp_token_denom");
pub const LAST_TOKEN_ID: Item<u64> = Item::new("last_token_id");

pub const TOKENS: Map<u64, Token> = Map::new("tokens");
pub const ALIASES: Map<String, u64> = Map::new("aliases");
pub const ROLES: Map<(u64, Addr, String), bool> = Map::new("roles");
pub const ROLES_GLOBAL: Map<(u64, String), bool> = Map::new("roles_global");

pub fn get_token(storage: &dyn Storage, denom: String) -> Result<Token, ContractError> {
    match ALIASES.may_load(storage, denom.clone())? {
        Some(id) => Ok(TOKENS.load(storage, id)?),
        None => Err(ContractError::TokenNotFound(denom)),
    }
}
