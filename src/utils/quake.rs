use base64::{
    engine::{general_purpose::STANDARD as st},
    Engine as _,
};
use image::GenericImageView;
use image::Pixel;
use pixelate::algorithms::lsb::LSB;


#[derive(Debug)]
struct Data<'a> {
    channel: u8,
    message: &'a str,
}
impl<'a> Data<'a> {
    fn new(channel: u8, message: &'a str) -> Data<'a> {
        Data {
            channel,
          
            message,
        }
    }
}

fn read_image(path: &str) -> Result<Vec<u8>, image::ImageError> {
    let img = image::open(path)?;
    let pixel_data: Vec<u8> = img
        .pixels()
        .flat_map(|(_, _, pixel)| {
            let rgba = pixel.to_rgba();
            vec![rgba[0], rgba[1], rgba[2], rgba[3]]
        })
        .collect();
    Ok(pixel_data)
}


//fn handler() -> Result<Vec<u8>, Error
//use serde_json::from_reader;


fn base_encode(s: &str) -> String {
    st.encode(s)
}
/**
fn base_decode(s: String) -> String {
    st.decode(s) as String
}**/

fn binary(text: String) -> String {
    text.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join("")
}

fn unbinary(bin: String) -> String {
    bin.chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| {
            let s: String = chunk.iter().collect();
            let byte: u8 = u8::from_str_radix(&s, 2).unwrap();
            byte as char
        })
        .collect()
}

fn data_encode(strng: &str) -> String {
    let base = base_encode(strng);
    binary(base.to_string())
}

/***fn data_decode(strng: &str) -> String {
    let bin = unbinary(strng.to_string());
    base_encode(bin.as_str())
}**/

pub fn handler() -> Result<(), &'static str> {
  let msg = "i love ness";
    let f_path = "./src/assets/img.jpg";
    let mut channel = 2u8;
    let message = data_encode(msg);
    let mut pixels = read_image(f_path).map_err(|_| "error reading image")?;

    let lsb_enc = LSB::encode(&mut pixels, &message, &mut channel).unwrap();

    let d = Data::new(channel, &message);
    println!("{:?}", d);

    Ok(())
}
