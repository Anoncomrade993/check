
use pixelate::algorithms::lsb::LSB;
use image::{GenericImageView,Pixel};
use base64::Engine;
use serde_json::Value;
use std::io::BufReader;
use std::fs::File;
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
      channel: channel.to_string(),
      pixels,
      message: message.to_string(),
    }
  }
}



fn read_image(path:&str) -> Result<Vec<u8>,image::flat::Error>{
  let _img = image::open(path).unwrap();
     let pixel_data: Vec<u8> = _img.pixels().flat_map(|(_, _, pixel)| {
        let rgba = pixel.to_rgba();
        vec![rgba[0], rgba[1], rgba[2], rgba[3]]
    }).collect();
  Ok(pixel_data)
}

fn read_json(path:&str) -> Result<Value,Error>{
  let file = File::open(path)?;
  let store = String::new();
  let mut buffer = BufReader::new(file);
  buffer(&mut store);
  let val : Data = serde_json::from_str(&buffer);
  Ok(val)
}
fn base_encode(data:&str){
   BASE_STANDARD.encode(data.as_bytes());
}
fn base_decode(data:&str){
   Engine::decode(data.to_string())
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
  let bin = binary(strng.to_string);
  let base = base_encode(&bin);
    String::from_utf8(base).unwrap()
}
pub fn handler(path : &str) -> Result<Vec<u8>,&'static str>{
  let jf = read_json(path).expect("error");
   
   let message = &jf["message"];
   
   let f_path= &jf["path"];
   
   let b_msg = data_encode(message.as_string());;
   let mut channel = &jf["channel"];
   let mut pixels = read_image(f_path).ok(b"error");
   let d = Data::new(&channel,&pixels,&b_msg);
   println!(" {:?}",d);
  let lsb_enc =  LSB::encode(&mut pixels,b_msg,&mut channel);
       lsb_enc
}