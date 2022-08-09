use dotenv::dotenv;
use std::env;

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    candidate_id: i32,
    first_name: String,
    last_name: String,
}

fn main() {
    dotenv().ok();

    let url = env::var("DB_URL");
    let value = url.as_deref().expect("DB URL NOT FOUND");

    let pool = Pool::new(value).expect("Fail to create pool");

    let mut conn = pool.get_conn().expect("fail to get connection");

    let selected_payments = conn.query_map(
        "SELECT candidate_id, first_name, last_name from candidate LIMIT 2",
        |(candidate_id, first_name, last_name)| {
            Candidate { candidate_id, first_name, last_name }
        }
    ).expect("fail to query");

    println!("{:?}",selected_payments);

    println!("{}",selected_payments[0].first_name);
}