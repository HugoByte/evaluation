use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;


use crate::state::{ LeaveRequest , leaveBalance } ;


#[cw_serde]

pub struct InstantiateMsg {
  pub employee: String,
  pub reason: String,
  pub approved: String,
  pub feedback: String,
  pub owner:String,
  pub contract_address : Addr
}
// pub enum Msg {
  
// }

#[cw_serde]

pub enum Msg {
  RequestLeave { emp_id: u32 , reason: String , leave_days: u32},
  ApproveLeave { employee: u32, feedback: String },
  RejectLeave { employee: u32, feedback: String },
  PostAnnouncement { announcement: String },

}



#[cw_serde]

#[derive(QueryResponses)]

pub enum QueryMsg {
     #[returns(Vec<LeaveRequest>)]
    GetAnnouncements{ feedback : String , owner: String},

}

// #[derive(Serialize, Deserialize , Clone , Debug , PartialEq)]
// pub enum HandleMsg {
//     CreateLeaveRequest { reason: String },
//     ApproveLeaveRequest { requester: String, feedback: String },
//     RejectLeave {requester: String , feedback: String},
//     PostAnnouncement { message: String },
// }


