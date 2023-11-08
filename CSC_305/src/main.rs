mod greetings;
use greetings::spanish;
use greetings::french;
extern crate hello_world_lib;

fn main() {
    println!("English - Hello, world!");
    println!("French - {}", french::default_greeting());
    println!("Spanish - {}", spanish::default_greeting());
    println!("{}",hello_world_lib::greeting_from_lib());
    
}  
