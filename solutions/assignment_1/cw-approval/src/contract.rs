// use std::env;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, from_binary, WasmQuery,
};
use cw_data_store::msg::QueryMsg::QueryLeaves;
// use cw2::set_contract_version;

use crate::error::ContractError;

use crate::msg::{self, InstantiateMsg, Msg, QueryMsg};
use crate::state::{Announcement, LeaveRequest, State, OWNER, STATE, DATASTORE};

const CONTRACT_NAME: &str = "crates.io:college";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-approval";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let data = LeaveRequest {
        employee: msg.employee,
        reason: msg.reason,
        approved: msg.approved,
        feedback: msg.feedback,

    };

    STATE.save(deps.storage, &vec![data]).unwrap();
    OWNER.save(deps.storage, &info.sender.to_string()).unwrap();
    DATASTORE.save(deps.storage, &msg.contract_address).unwrap();
    Ok(Response::new())
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// fn leave_balance(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: StoreMsg,
// ) -> StdResult<Response> {
//     let mut state = STATE.load(deps.storage);
//     // let store = store.borrow_mut();
//     state.UpdateLeaveBalance(msg.emp_id, msg.leave_balance)?;
//     Ok(Response::default())
// }

//update_leave_balance

#[entry_point]

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Msg) -> StdResult<Response> {

    let mut state = STATE.load(deps.as_ref().storage)?;
    let owner = OWNER.load(deps.storage)?;

    let contract_address = DATASTORE.load(deps.as_ref().storage).unwrap();

    if info.sender != owner {
        return Err(StdError::NotFound {
            kind: "Invalid owner".to_string(),
        });
    }

    match msg {
       
        Msg::RequestLeave { emp_id , reason, leave_days } => {
            let query_msg = QueryLeaves{ emp_id: emp_id.clone() };
            let query = WasmQuery::Smart { contract_addr: contract_address.to_string(), msg: to_binary(&query_msg).unwrap() };
            
            let leave_balance:u32 = deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(query)).unwrap();
         
              if leave_days > leave_balance {
                return Err(StdError::generic_err("Not that amount of leave is left"));
            }
            if info.sender != owner {
                return Err(StdError::NotFound {
                    kind: "Invalid owner".to_string(),
                });
            }

            let leave_request = LeaveRequest {
                employee:emp_id,
                reason: reason,
                approved:String::from("pending"),
                feedback: String::new(),
               
            };
            
            let updated_leave_balance = leave_balance - leave_days;
            // let store_msg = StoreMsg::UpdateLeaveBalance { emp_id: emp_id.clone(), leave_balance: updated_leave_balance };
            // leave_balance(deps, env, info, store_msg)?;
            let mut state = STATE.load(deps.storage)?;
            state.push(leave_request);
            STATE.save(deps.storage, &state)?;


            Ok(Response::default())
        }
        
        Msg::ApproveLeave {
            employee,
            feedback,
        } => {
            if info.sender != owner {
                return Err(StdError::NotFound {
                    kind: "Invalid owner".to_string(),
                });
            }

            if let Some(request) = state
                // .leave_requests
                .iter_mut()
                .find(|r| r.employee == employee)
            {
                request.approved= String::from("approved");
                request.feedback = feedback;
                STATE.save(deps.storage, &state)?;
                Ok(Response::default())
            } else {
                Err(StdError::generic_err("Leave request not found"))
            }
        }
        Msg::RejectLeave {
            employee,
            feedback,
        } => {
            if info.sender != owner {
                return Err(StdError::NotFound {
                    kind: "Invalid owner".to_string(),
                });
            }

            if let Some(request) = state
                // .leave_requests
                .iter_mut()
                .find(|r| r.employee == employee)
            {
                request.approved = String::from("rejected");
                request.feedback = feedback;
                STATE.save(deps.storage, &state)?;
                Ok(Response::default())
            } else {
                Err(StdError::generic_err("Leave request not found"))
            }
        }
    
        Msg::PostAnnouncement { announcement } => {
            if info.sender != owner {
                return Err(StdError::NotFound {
                    kind: "Invalid owner".to_string(),
                });
            }

            //  state.push(announcement);
            STATE.save(deps.storage, &state).unwrap();
            Ok(Response::default())
        }


}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env,  msg: QueryMsg) -> StdResult<QueryResponse> {
    let mut state = STATE.load(deps.storage)?;
    let state = STATE.load(deps.storage).unwrap();
    let actual_owner = OWNER.load(deps.storage)?;
    match msg {
        QueryMsg::GetAnnouncements { feedback , owner} => {
            if actual_owner != owner {
                return Err(StdError::NotFound {
                    kind : "invalid owner".to_string(),
                });
            }
            //  let state = STATE.load(deps.storage).unwrap();
            // // let announcements_binary = Binary::from(&state.announcements);
            let final_announcements = state.iter().filter(|a| a.feedback == feedback).cloned().collect::<Vec<_>>();

             return Ok(to_binary(&final_announcements)?);
        }
      
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{from_binary, CosmosMsg, BankMsg, Addr};
    use crate::msg::{Msg, QueryMsg};
    use crate::state::LeaveRequest;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg { employee: 2.to_owned(), reason: "ok".to_string() , approved: "Ok".to_owned(), feedback: "Ok".to_owned(), owner: "hardik".to_owned(), contract_address: Addr::unchecked("foobar")};
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
     #[test]
     fn test_request_leave() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let alice = mock_info("alice", &[]);
        let msg = InstantiateMsg { employee: 1.to_owned(), reason: "outside".to_string(), approved: "yes".to_owned(), feedback: "ok go".to_owned(), owner: "hardik".to_owned(), contract_address: Addr::unchecked("foobar")};
        let _ = instantiate(deps.as_mut(), env.clone(), alice.clone(), msg).unwrap();

       
        let reason = "Going on vacation".to_string();
        let msg = Msg::RequestLeave { emp_id: 3, reason : "outside".to_string(), leave_days: 3};
        let res = execute(deps.as_mut(), env.clone(), alice.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        
        let query_msg = QueryMsg::GetAnnouncements { feedback: "None".to_owned(), owner: alice.sender.to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let state: Vec<LeaveRequest> = from_binary(&res).unwrap();
        assert_eq!(1, state.len());
        // assert_eq!(alice, state[0].employee);
        assert_eq!("Going on vacation", state[0].reason);
        assert_eq!("pending", state[0].approved);
        assert_eq!("", state[0].feedback);
    }
    #[test]
    fn test_approve_leave() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let alice = mock_info("alice", &[]);
        let bob = mock_info("bob", &[]);
        let msg = InstantiateMsg { employee: 5.to_owned(), reason: "not well".to_string(), approved: "yes".to_owned(), feedback: "okkk".to_owned(), owner: "kartik".to_owned(), contract_address: Addr::unchecked("foobar") };
        let _ = instantiate(deps.as_mut(), env.clone(), alice.clone(), msg).unwrap();

       
        let reason = "Going on vacation".to_string();
        //  let msg = Msg::RequestLeave { emp_id , reason , leave_days};
         let msg = Msg::RequestLeave { emp_id: 3, reason : "outside".to_string(), leave_days: 3};
        let _ = execute(deps.as_mut(), env.clone(), alice.clone(), msg).unwrap();

        
        let employee = "alice".to_string();
        let feedback = "Approved!".to_string();
        let msg = Msg::ApproveLeave { feedback, employee:5.to_owned() };
        let res = execute(deps.as_mut(), env.clone(), bob.clone(), msg).unwrap();
         assert_eq!(0, res.messages.len());

        
        let query_msg = QueryMsg::GetAnnouncements { feedback: "None".to_string(), owner: alice.sender.to_string() };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let state: Vec<LeaveRequest> = from_binary(&res).unwrap();
        assert_eq!(1, state.len());
        // assert_eq!("alice", state[0].employee);
        assert_eq!("Going on vacation", state[0].reason);
        assert_eq!("approved", state[0].approved);
        assert_eq!("Approved!", state[0].feedback);
    }


}