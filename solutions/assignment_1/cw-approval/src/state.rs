use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Serialize , Deserialize};

// use crate::msg::Announcement;

#[derive(Debug , Serialize , Deserialize , Clone , PartialEq , Eq, JsonSchema)]


pub struct LeaveRequest {
    pub employee: String,
    pub reason: String,
    pub approved: String,
    pub feedback: String,
    
}

#[derive(Serialize , Deserialize)]
pub struct Announcement {
    admin: String,
    message: String,

}


// pub const OWNER: Item<String> = Item::new("ownner");

// pub const STATE: Item<Vec<Employee>> = Item::new("STATE");
#[derive(Serialize , Deserialize)]
pub struct State {
     announcements:  Vec<Announcement>,
     leave_requests: Vec<LeaveRequest>,
     
}


// #[derive(Serialize , Deserialize)]
// pub struct QueryResponses {
//     // leave_request: Vec<leaveRequest>,
//     // 
//     GetAnnouncements: 

// }

pub const OWNER: Item<String> = Item::new("Owner");


pub const STATE : Item<Vec<LeaveRequest>> = Item::new("STATE");

