#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate uuid;
extern crate blockchain;

use std::sync::Mutex;
use std::error::Error;

use rocket::State;
use rocket_contrib::{Json, Value};

use uuid::Uuid;
use blockchain::ledger::DistributedLedger;
use blockchain::ledger::util::Timestamp;
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
        return Json(json!({ "status": "error", "reason": e.description() }));
    }

    let timestamp = Timestamp::current_nanos();
    if let Err(e) = ledger.new_block(timestamp, proof) {
        return Json(json!({ "status": "error", "reason": e.description() }));
    }

    Json(json!(ledger.last_block().expect("Get last block; but there is none!")))
}

#[post("/transactions", format = "application/json", data = "<tx>")]
fn create_transaction(tx: Json<StupidTransaction>, ledger: State<Ledger>) -> Json<Value> {
    let mut ledger = ledger.lock().unwrap();
    let json = match ledger.add_transaction(tx.0) {
        Ok(index) => json!({ "index": index }),
        Err(e) => json!({ "status": "error", "reason": e.description() })
    };
    Json(json)
}

#[post("/peers", format = "application/json", data = "<peers>")]
fn register_peers(peers: Json<Vec<String>>, ledger: State<Ledger>) -> Json<Vec<String>> {
    let mut ledger = ledger.lock().unwrap();
    for peer in peers.0 {
        ledger.register_peer(peer);
    }

    Json(ledger.peers.to_vec())
}

#[post("/peers/consensus")]
fn consensus(ledger: State<Ledger>) -> Json<Value> {
    let mut ledger = ledger.lock().unwrap();

    let is_replaced = match ledger.resolve_conflicts() {
        Ok(is_replaced) => is_replaced,
        Err(e) => {
            return Json(json!({ "status": "error", "reason": e.description() }));
        }
    };
    Json(json!({ "is_replaced": is_replaced, "chain": ledger.chain.to_vec() }))
}

fn main() {
    let node_id = Uuid::new_v4();

    rocket::ignite()
        .manage(Mutex::new(StupidLedger::new(vec![])))
        .manage(node_id.simple().to_string())
        .mount("/", routes![
            get_full_chain,
            mine_block,
            create_transaction,
            register_peers,
            consensus
        ])
        .launch();
}
