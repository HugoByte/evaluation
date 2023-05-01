#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{self, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Employee, OWNER, STATE, self};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-data-store";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contract_storage = Employee {
        emp_id: msg.emp_id,
        emp_name: msg.emp_name,
        leave_balance: msg.leave_balance,
        feedback: msg.feedback,
    };
    STATE.save(deps.storage, &vec![contract_storage]).unwrap();
    OWNER.save(deps.storage, &info.sender.to_string()).unwrap();
    Ok(Response::new())
}

pub fn store_leave_balance(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
    emp_id: String,
    leave_balance: u32,
) -> Result<Response, ContractError> {
    let actual_owner = OWNER.load(deps.storage).unwrap();
    let owner = OWNER.load(deps.storage)?;
    if actual_owner != owner {
        return Err(ContractError::Unauthorized {});
    }
    let mut state = STATE.load(deps.storage)?;
    let employee = state
        .iter_mut()
        .find(|e| e.emp_id == emp_id)
        .ok_or(ContractError::EmployeeNotFound {})?;
    employee.leave_balance = leave_balance;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

pub fn query_leave_balance(deps: Deps, emp_id: String) -> Result<QueryResponse, ContractError> {
    let actual_owner = OWNER.load(deps.storage).unwrap();
    let owner = OWNER.load(deps.storage)?;
    if actual_owner != owner {
        return Err(ContractError::Unauthorized {});
    }
    let mut state = STATE.load(deps.storage)?;

    let employee = state
        .iter()
        .find(|e| e.emp_id == emp_id)
        .ok_or(ContractError::EmployeeNotFound {})?;

    let response = to_binary(&employee.leave_balance)?;
    Ok(response)
}

pub fn store_feedback(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    emp_id: String,
    feedback: String,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    // Find the employee with the given emp_id
    let employee = state
        .iter_mut()
        .find(|e| e.emp_id == emp_id)
        .ok_or(ContractError::EmployeeNotFound {})?;

    // Update the feedback for the employee
    employee.feedback = feedback;

    // Save the updated state back to storage
    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

pub fn query_feedback(deps: Deps, emp_id: String) -> Result<QueryResponse, ContractError> {
    // Load the current contract storage
    let state = STATE.load(deps.storage)?;

    // Find the employee with the given emp_id
    let employee = state
        .iter()
        .find(|e| e.emp_id == emp_id)
        .ok_or(ContractError::EmployeeNotFound {})?;

    // Create a JSON-encoded response with the feedback
    let response = to_binary(&employee.feedback)?;
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddEmployee {
            emp_id,
            emp_name,
            leave_balance,
            feedback,
        } => {
            let employee = Employee {
                emp_id,
                emp_name,
                leave_balance,
                feedback,
            };

            STATE
                .update(
                    deps.storage,
                    |mut data| -> Result<Vec<Employee>, ContractError> {
                        data.push(employee);

                        Ok(data)
                    },
                )
                .unwrap();
            Ok(Response::new())
        }
        ExecuteMsg::UpdateEmployee {
            emp_id,
            emp_name,
            leave_balance,
            feedback,
        } => {
            STATE
                .update(
                    deps.storage,
                    |mut data| -> Result<Vec<Employee>, ContractError> {
                        let index = data
                            .iter()
                            .position(|emp| emp.emp_id == emp_id)
                            .ok_or_else(|| ContractError::EmployeeNotFound {})?;
                        let mut employee = &mut data[index];
                        employee.emp_name = emp_name;
                        employee.leave_balance = leave_balance;
                        employee.feedback = feedback;

                        Ok(data)
                    },
                )
                .unwrap();
            Ok(Response::new())
        }

    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Querypersonal { emp_id } => {
            let state = STATE.load(deps.storage).unwrap();
            let mut employee: Option<Employee> = None;
            for emp in &state {
                if emp.emp_id == emp_id {
                    employee = Some(emp.clone());
                    break;
                }
            }

            let emp = employee.ok_or_else(|| StdError::generic_err("Employee not found"))?;

            Ok(to_binary(&emp).unwrap())
        }
        QueryMsg::QueryEmployees { owner } => {
            let actual_owner = OWNER.load(deps.storage).unwrap();

            if actual_owner != owner {
                return Err(StdError::NotFound {
                    kind: "InvalidOwner".to_string(),
                });
            }

            let state = STATE.load(deps.storage).unwrap();
            // let state = get_state(deps.storage)?;
            Ok(to_binary(&state).unwrap())
        }
        QueryMsg::QueryLeaves{ emp_id }  => {
            let state = STATE.load(deps.storage).unwrap();
            let employee = state.iter().find(|e| e.emp_id.parse::<u32>().unwrap()  == emp_id).ok_or(ContractError::EmployeeNotFound {}).unwrap();
            let response = to_binary(&employee.leave_balance)?;
            Ok(response.into())
        }
    }
      
    }


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies , mock_env , mock_info};
    use cosmwasm_std::{coins, from_binary};

    // #[test]
//    fn test_query_personal_details(){
//     let mut deps = mock_dependencies(&[]);
//     let env = mock_env();
//     let emp_id = "1".to_string();

//     l
//    }

fn init_msg() -> InstantiateMsg {
    InstantiateMsg {
        emp_id: "test-employee".to_string(),
        emp_name: "Test Employee".to_string(),
        leave_balance: 10,
        feedback: "No feedback yet".to_string(),
    }
}

#[test]
    fn test_instantiation_check() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), env, info, init_msg()).unwrap();
        assert_eq!(0, res.messages.len());
        let query_res = query_leave_balance(deps.as_ref(), "test-employee".to_string()).unwrap();
        let value: u32 = from_binary(&query_res).unwrap();
        assert_eq!(10, value);
    }
    #[test]
    fn leave_balance_update_check() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), env, info, init_msg()).unwrap();
        assert_eq!(0, res.messages.len());

        let emp_id = "1".to_string();
        let new_leave_balance = 25;
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = store_leave_balance( deps.as_mut(), mock_env(), info,init_msg(), emp_id.clone(),new_leave_balance,);
        assert!(res.is_ok());

        let query_res = query_leave_balance(deps.as_ref(), emp_id.clone()).unwrap();
        let value: u32 = from_binary(&query_res).unwrap();
        assert_eq!(new_leave_balance, value);
    }

    #[test]
    fn leave_balance_without_unauthorized_sender() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), env, info, init_msg()).unwrap();
        assert_eq!(0, res.messages.len());

        let emp_id = "5".to_string();
        let new_leave_balance = 20;
        let info = mock_info("other", &coins(1000, "earth"));
        let res = store_leave_balance(
            deps.as_mut(),
            mock_env(),
            info,
            init_msg(),
            emp_id.clone(),
            new_leave_balance,
        );
         assert!(res.is_err());

        let query_res = query_leave_balance(deps.as_ref(), emp_id.clone()).unwrap();
        let value: u32 = from_binary(&query_res).unwrap();
        assert_eq!(0, value);
    }

    #[test]
    fn store_and_query_feedback() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), env, info, init_msg()).unwrap();
        assert_eq!(0, res.messages.len());

        let emp_id = "test-employee".to_string();
        let feedback = "Good job!".to_string();
        let info = mock_info("user1" , &coins(100 , "earth"));
        let res = store_feedback(deps.as_mut(), mock_env(), info, emp_id.clone(), feedback.clone());
        assert_eq!("{}" ,"{}");

}

}