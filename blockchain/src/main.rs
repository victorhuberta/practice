#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate blockchain;

use std::sync::Mutex;
use rocket::State;
use rocket_contrib::Json;
use blockchain::ledger::DistributedLedger;
use blockchain::ledger::error::TransactionError;
use blockchain::ledger::example::stupid::{StupidLedger, StupidBlock, StupidTransaction};

type Ledger = Mutex<StupidLedger>;

#[get("/blocks")]
fn get_full_chain(ledger: State<Ledger>) -> Json<Vec<StupidBlock>> {
    Json(ledger.lock().unwrap().chain().to_vec())
}

#[post("/blocks")]
fn mine_block() -> &'static str {
    "Hello, world"
}

#[post("/transactions", format = "application/json", data = "<tx>")]
fn create_transaction(tx: Json<StupidTransaction>, ledger: State<Ledger>) -> Json<Result<usize, TransactionError>> {
    let mut ledger = ledger.lock().unwrap();
    Json(ledger.add_transaction(tx.0))
}

fn main() {
    rocket::ignite()
        .manage(Mutex::new(StupidLedger::new(vec![])))
        .mount("/", routes![
           mine_block,
           create_transaction,
           get_full_chain
        ])
        .launch();
}
