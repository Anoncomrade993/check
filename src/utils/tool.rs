
use pixelate::algorithms::lsb::LSB::{encode,decode};
use image::GenericImageView;
use base64::prelude::*;
use serde_json::{Deserialize, Serialize,Value};
use std::io::{Read,BufReader};
use std::fs::File
use std::io::Error;

#[derive(Debug)]
struct Data{
  channel : String,
  pixels : Vec<u8>,
  message: String,
}
impl Data{
  fn new(channel:&str,pixels:Vec<u8>, message:&str)-> Self{
    Self{
      channel: channel.as_string(),
      pixels,
      message: message.as_string(),
    }
  }
}



fn read_image(path:&str) -> Result<Vec<u8>,Error>{
  let _img = image::open(path)?;
     _img.pixels()
}

fn read_json(path:&str) -> Result<Value,Error>{
  let file = File::open(path)?;
  let store = String::new();
  let mut buffer = BufReader::new(file)?;
  buffer.read_as_string(&mut store);
  let val : Value = serde_json::from_str(&buffer);
  Ok(val)
}
fn base_encode(data:&str){
   BASE_STANDARD.encode(data.as_bytes());
}
fn base_decode(data:&str){
   BASE_STANDARD.decode(data.as_bytes());
}
/// Approach to convert a string to binary
 fn binary(text: String) -> String {
    text.chars().map(|c| format!("{:08b}", c as u8)).collect()
}

/// Binary back to string
 fn unbinary(bin: String) -> String {
    bin.chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|c| {
            let s: String = c.iter().collect();
            let byte: u8 = u8::from_str_radix(&s, 2).unwrap();
            byte as char
        })
        .collect()
}
fn data_encode(strng:&str) -> String {
  let bin = binary(strng);
  let base = base_encode(bin.as_bytes());
    base.as_str()
}
pub fn handler(path : &str) -> Result<Vec<u8>,&'static str>{
  let jf = read_json(path).ok()?;
   
   let message = &jf["message"];
   
   let f_path= &jf["path"];
   
   let b_msg = base_encode(message);
   let mut channel = &jf["channel"];
   let mut pixels = read_image(f_path);
   let d = Data::new(&channel,&pixels,&b_msg);
   println("{:?}",d);
  let lsb_enc =  encode(&mut pixels,b_msg,&mut channel);
       lsb_enc
}