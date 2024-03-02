mod utils;
use utils::quake::handler;
#[warn(dead_code)]
fn main() {
  match handler(){
    Ok(lsb)=>{
      println!("{:#?}",lsb)
    }
    Err(e)=>{
      println!("{}",e)
  }
  }
  
}