use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub anchor_token_addr: Addr,
    pub anchor_market_addr: Addr,
    pub aterra_token_addr: Addr,
    pub anchor_oracle_addr: Addr,
    pub anchor_overseer_addr: Addr,
    pub basset_vault_for_bluna_addr: Addr,
    pub nluna_token_addr: Addr,
    pub basset_vault_for_beth_addr: Addr,
    pub neth_token_addr: Addr,
    pub bluna_hub_addr: Addr,
    pub bluna_token_addr: Addr,
    pub anchor_custody_bluna_addr: Addr,
    pub beth_token_addr: Addr,
    pub anchor_custody_beth_addr: Addr,
}

static KEY_CONFIG: Item<Config> = Item::new("config");

pub fn load_config(storage: &dyn Storage) -> StdResult<Config> {
    KEY_CONFIG.load(storage)
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    KEY_CONFIG.save(storage, config)
}
