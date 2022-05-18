use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item,Map};

pub const CONFIG: Item<State> = Item::new("config_state");
pub const PRICE_INFO: Item<PriceInfo> = Item::new("config_price");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner:String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceInfo {
    pub price: Uint128,
    pub last_updated_time: u64,
}