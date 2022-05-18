use cosmwasm_std::{
    entry_point, to_binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,Binary,
    StdResult, Uint128,CosmosMsg,WasmMsg,BankMsg
};

use cw2::set_contract_version;
use cw20::{ Cw20ExecuteMsg};


use crate::error::{ContractError};
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State,CONFIG, PRICE_INFO, PriceInfo};


const CONTRACT_NAME: &str = "ORACLE_CONTRACT";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
   
    let state = State {
        owner : info.sender.to_string(),
    };
    PRICE_INFO.save(deps.storage, &msg.price)?;
    CONFIG.save(deps.storage,&state)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::UpdatePrice {new_price}=>execute_update_price(deps,env,info,new_price),

    }
}

fn execute_update_price(
    deps: DepsMut,
    env:Env,
    info: MessageInfo,
    new_price:Uint128
) -> Result<Response, ContractError> {
    let  state = CONFIG.load(deps.storage)?;
    
    if info.sender.to_string() != state.owner{
        return Err(ContractError::Unauthorized {})
    }

    PRICE_INFO.update(deps.storage, 
        |mut price_info|->StdResult<_>{
            price_info.last_updated_time = env.block.time.seconds();
            price_info.price = new_price;
            Ok(price_info)
        }
    )?;

    Ok(Response::default())
}





#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice {} => to_binary(&query_get_price(deps)?),
    }   
}

pub fn query_get_price(deps:Deps) -> StdResult<PriceInfo>{
    let price_info = PRICE_INFO.load(deps.storage)?;
    Ok(price_info)
}




#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{ CosmosMsg, Coin};

    #[test]
    fn instantiate_contract() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let instantiate_msg = InstantiateMsg {
           price:PriceInfo { price: Uint128::new(100), last_updated_time: env.block.time.seconds() }
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let crr_price = query_get_price(deps.as_ref()).unwrap();
        assert_eq!(crr_price,PriceInfo{
            price:Uint128::new(100),
            last_updated_time :  env.block.time.seconds()
        });

    }

    #[test]
    fn update_price() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let instantiate_msg = InstantiateMsg {
           price:PriceInfo { price: Uint128::new(100), last_updated_time: env.block.time.seconds() }
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let crr_price = query_get_price(deps.as_ref()).unwrap();
        assert_eq!(crr_price,PriceInfo{
            price:Uint128::new(100),
            last_updated_time :  env.block.time.seconds()
        });

       let info = mock_info("creator", &[]);
       let msg = ExecuteMsg::UpdatePrice { new_price: Uint128::new(125) };
       let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
          assert_eq!(0, res.messages.len());

        let crr_price = query_get_price(deps.as_ref()).unwrap();
        assert_eq!(crr_price,PriceInfo{
            price:Uint128::new(125),
            last_updated_time :  env.block.time.seconds()
        });
       
    }
}
 