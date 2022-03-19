use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub cw_gateway_contract_addr: Addr,
    pub reentrancy_prevention_flag: bool,
}

pub const STATE: Item<State> = Item::new("state");
