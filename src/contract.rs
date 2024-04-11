use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, NOIS_PROXY, PARTICIPANT_COUNT, TEST_WINNERS, WINNERS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use nois::pick;
use nois::{NoisCallback, ProxyExecuteMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy_address)
        .map_err(|_| ContractError::InvalidProxyAddress {})?;
    if msg.participant_count == 0 {
        return Err(ContractError::InvalidParticipantCount {});
    }
    PARTICIPANT_COUNT.save(deps.storage, &msg.participant_count)?;
    NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;
    ADMIN.save(deps.storage, &info.sender)?;

    let res = Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("participant_count", msg.participant_count.to_string())
        .add_attribute("nois_proxy_address", msg.nois_proxy_address)
        .add_attribute("nois_proxy_addr", nois_proxy_addr.to_string())
        .add_attribute("creator", info.sender.to_string());

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RequestRandomness { job_id } => {
            execute_request_randomness(deps, env, info, job_id)
        }
        ExecuteMsg::NoisReceive { callback } => execute_pick_winners(deps, env, info, callback),
    }
}

pub fn execute_request_randomness(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: String,
) -> Result<Response, ContractError> {
    let nois_proxy = NOIS_PROXY.load(deps.storage)?;

    if info.sender != ADMIN.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let res = Response::new().add_message(WasmMsg::Execute {
        contract_addr: nois_proxy.into(),
        msg: to_json_binary(&ProxyExecuteMsg::GetNextRandomness { job_id })?,
        funds: info.funds.clone(),
    });
    Ok(res)
}

pub fn execute_pick_winners(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    callback: NoisCallback,
) -> Result<Response, ContractError> {
    let proxy = NOIS_PROXY.load(deps.storage)?;
    ensure_eq!(info.sender, proxy, ContractError::Unauthorized {});

    let NoisCallback {
        job_id, randomness, ..
    } = callback;

    let randomness: [u8; 32] = randomness
        .to_array()
        .map_err(|_| ContractError::InvalidRandomness {})?;

    let participant_count = PARTICIPANT_COUNT.load(deps.storage)?;
    let participant_arr = (0..participant_count).collect::<Vec<u32>>();

    let winners = pick(randomness, 100, participant_arr);
    let winners_string = winners
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    match job_id.as_str() {
        "test" => {
            TEST_WINNERS.save(deps.storage, &winners)?;
        }
        _ => {
            let old_winners = WINNERS.may_load(deps.storage)?;
            if old_winners.is_some() {
                return Err(ContractError::WinnersAlreadyPicked {});
            }
            WINNERS.save(deps.storage, &winners)?;
        }
    }

    let res = Response::new().add_attribute("winners", winners_string);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ParticipantCount {} => to_json_binary(&PARTICIPANT_COUNT.load(_deps.storage)?),
        QueryMsg::Winners {} => to_json_binary(&WINNERS.load(_deps.storage)?),
        QueryMsg::Admin {} => to_json_binary(&ADMIN.load(_deps.storage)?),
        QueryMsg::NoisProxy {} => to_json_binary(&NOIS_PROXY.load(_deps.storage)?),
        QueryMsg::TestWinners {} => to_json_binary(&TEST_WINNERS.load(_deps.storage)?),
    }
}

#[cfg(test)]
mod tests {}
