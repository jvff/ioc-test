extern crate ioc_test;

use ioc_test::tests::*;

fn main() {
    match run_test() {
        Ok(_) => println!("Test Successful"),
        Err(error) => println!("Test FAILED: {}", error),
    }
}
