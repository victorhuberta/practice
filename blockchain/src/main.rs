#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate uuid;
extern crate blockchain;

use std::sync::Mutex;
use rocket::State;
use rocket_contrib::{Json, Value};
use uuid::Uuid;
use blockchain::ledger::{DistributedLedger, Timestamp};
use blockchain::ledger::example::stupid::{StupidLedger, StupidBlock, StupidTransaction};

type Ledger = Mutex<StupidLedger>;

#[get("/blocks")]
fn get_full_chain(ledger: State<Ledger>) -> Json<Vec<StupidBlock>> {
    Json(ledger.lock().unwrap().chain.to_vec())
}

#[post("/blocks")]
fn mine_block(node_id: State<String>, ledger: State<Ledger>) -> Json<Value> {
    let mut ledger = ledger.lock().unwrap();
    let last_proof = if let Some(last_block) = ledger.last_block() {
        last_block.proof
    } else {
        0
    };
    let proof = ledger.find_proof(last_proof);

    let tx = StupidTransaction::new(String::from("0"), node_id.clone(), 1);
    if let Err(e) = ledger.add_transaction(tx) {
        return Json(json!({ "status": "error", "reason": e }));
    }

    let timestamp = Timestamp::current_nanos();
    if let Err(e) = ledger.new_block(timestamp, proof) {
        return Json(json!({ "status": "error", "reason": e }));
    }

    Json(json!(ledger.last_block().expect("Get last block; but there is none!")))
}

#[post("/transactions", format = "application/json", data = "<tx>")]
fn create_transaction(tx: Json<StupidTransaction>, ledger: State<Ledger>) -> Json<Value> {
    let mut ledger = ledger.lock().unwrap();
    let json = match ledger.add_transaction(tx.0) {
        Ok(index) => json!({ "index": index }),
        Err(e) => json!({ "error": e })
    };
    Json(json)
}

fn main() {
    let node_id = Uuid::new_v4();

    rocket::ignite()
        .manage(Mutex::new(StupidLedger::new(vec![])))
        .manage(node_id.simple().to_string())
        .mount("/", routes![
           mine_block,
           create_transaction,
           get_full_chain
        ])
        .launch();
}
