use cosmwasm_schema::write_api;

use cw_approval::msg::{Msg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Msg,
        query: QueryMsg,
    }
}
