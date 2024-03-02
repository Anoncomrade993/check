mod utils;
use utils::tool::handler;
use std::io::stdin;

fn main() {
  let mut box_ = String::new();
  match stdin().read_line(&mut box_) {
    Ok(b) => {
      handler(&box_).unwrap();
    },
    Err(_)=>{
      println!("Error taking input")
    
    }
  }
}