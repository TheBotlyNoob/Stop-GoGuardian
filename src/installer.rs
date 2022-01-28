use std::env::var;

fn main() {
    let username = var("UserName").unwrap();
}

