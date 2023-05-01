use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use ibcx_interface::periphery::SwapInfo;

#[cw_serde]
pub enum Context {
    Mint {
        // TODO: do something
    },
    Burn {
        core: Addr,
        sender: Addr,
        input: Coin,
        min_output: Coin,
        redeem_amounts: Vec<Coin>,
        swap_info: Vec<SwapInfo>,
    },
}

impl Context {
    pub fn kind(self) -> String {
        match self {
            Context::Mint {} => "mint".to_string(),
            Context::Burn { .. } => "burn".to_string(),
        }
    }
}

pub const CONTEXT: Item<Context> = Item::new("context");
