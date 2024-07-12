#![allow(dead_code, unused_variables, unused_imports, private_interfaces)]

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Deserialize)]
struct StockStruct {

}


#[derive(Deserialize)]
struct OptionStruct {

}


#[derive(Deserialize)]
#[serde(tag = "quoteType")]
pub enum FinResult {
    STOCK(StockStruct),
    OPTION(OptionStruct),
}


impl FinResult {
    pub fn new(data : &str) -> Result<Vec<Self>, String> {
        let json_obj: Value = serde_json::from_str(data).unwrap();
        let json_obj = json_obj.get("quoteResponse").unwrap();
        
        let pot_error = json_obj.get("error");

        if pot_error.is_some() {
            return Err(pot_error.unwrap().to_string());
        }

        let result = json_obj.get("result").unwrap().as_array().unwrap();

        let ret_obj = Vec::<Self>::new();

        for obj in result {
            
        }


        Ok(vec!(FinResult::STOCK(StockStruct {})))
    }

}

