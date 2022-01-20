use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub anchor_token_addr: String,
    pub anchor_market_addr: String,
    pub aterra_token_addr: String,
    pub anchor_oracle_addr: String,
    pub anchor_overseer_addr: String,
    pub basset_vault_for_bluna_addr: String,
    pub nluna_token_addr: String,
    pub basset_vault_for_beth_addr: String,
    pub neth_token_addr: String,
    pub bluna_hub_addr: String,
    pub bluna_token_addr: String,
    pub anchor_custody_bluna_addr: String,
    pub beth_token_addr: String,
    pub anchor_custody_beth_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAddresses {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub anchor_token_addr: String,
    pub anchor_market_addr: String,
    pub aterra_token_addr: String,
    pub anchor_oracle_addr: String,
    pub anchor_overseer_addr: String,
    pub basset_vault_for_bluna_addr: String,
    pub nluna_token_addr: String,
    pub basset_vault_for_beth_addr: String,
    pub neth_token_addr: String,
    pub bluna_hub_addr: String,
    pub bluna_token_addr: String,
    pub anchor_custody_bluna_addr: String,
    pub beth_token_addr: String,
    pub anchor_custody_beth_addr: String,
}