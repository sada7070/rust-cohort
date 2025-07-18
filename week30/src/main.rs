// Serde

use serde::{ Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let s = String::from("{\"username\": \"sada\", \"password\": \"123123\"}");

    let u: Result<User, serde_json::Error> = serde_json::from_str(&s);

    println!("{:?}", u.unwrap());
}
