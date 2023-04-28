
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq, Eq,JsonSchema)]
pub struct Employee{
    pub emp_id : String,
    pub emp_name: String,
    pub leave_balance: u32,
    pub feedback: String,
    
}

pub const OWNER: Item<String> = Item::new("Owner");


pub const STATE : Item<Vec<Employee>> = Item::new("STATE");


