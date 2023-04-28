use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Employee;

#[cw_serde]
pub struct InstantiateMsg {
    pub emp_id: String,
    pub emp_name: String,
    pub leave_balance: u32,
    pub feedback: String,

}

#[cw_serde]
pub enum ExecuteMsg {

AddEmployee {emp_id : String , emp_name: String , leave_balance: u32 , feedback: String },
UpdateEmployee{emp_id: String , emp_name: String , leave_balance: u32 , feedback: String },

}

#[cw_serde]

#[derive(QueryResponses)]

pub enum QueryMsg {
    #[returns(Employee)]
        Querypersonal { emp_id : String },

        #[returns(Vec<Employee>)]
        QueryEmployees { owner:String },

        
    }
    

