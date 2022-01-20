use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult,
};

use crate::state::{load_config, store_config, Config};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        anchor_token_addr: deps.api.addr_validate(&msg.anchor_token_addr)?,
        anchor_market_addr: deps.api.addr_validate(&msg.anchor_market_addr)?,
        aterra_token_addr: deps.api.addr_validate(&msg.aterra_token_addr)?,
        anchor_oracle_addr: deps.api.addr_validate(&msg.anchor_oracle_addr)?,
        anchor_overseer_addr: deps.api.addr_validate(&msg.anchor_overseer_addr)?,
        basset_vault_for_bluna_addr: deps.api.addr_validate(&msg.basset_vault_for_bluna_addr)?,
        nluna_token_addr: deps.api.addr_validate(&msg.nluna_token_addr)?,
        basset_vault_for_beth_addr: deps.api.addr_validate(&msg.basset_vault_for_beth_addr)?,
        neth_token_addr: deps.api.addr_validate(&msg.neth_token_addr)?,
        bluna_hub_addr: deps.api.addr_validate(&msg.bluna_hub_addr)?,
        bluna_token_addr: deps.api.addr_validate(&msg.bluna_token_addr)?,
        anchor_custody_bluna_addr: deps.api.addr_validate(&msg.anchor_custody_bluna_addr)?,
        beth_token_addr: deps.api.addr_validate(&msg.beth_token_addr)?,
        anchor_custody_beth_addr: deps.api.addr_validate(&msg.anchor_custody_beth_addr)?,
    };
    store_config(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: ExecuteMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAddresses {} => to_binary(&load_config(deps.storage)?),
    }
}