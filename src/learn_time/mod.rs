use std::time::Instant;
use std::time::SystemTime;

pub fn initialization()
{

    println!("Hello world  time {:?}", Instant::now());
    println!("Hello world  time {:?}", SystemTime::now());
}