use image::GenericImageView;
use image::Pixel;
use pixelate::algorithms::lsb::LSB;
#[derive(Debug)]
struct Data<'a>{
    channel: u8,
    pixels: Vec<u8>,
    message: &'a str,
}
impl<'a> Data<'a> {
    fn new(channel: u8, pixels: Vec<u8>, message: &'a str) -> Data<'a> {
        Data { channel, pixels, message }
    }
}

fn read_image(path: &str) -> Result<Vec<u8>, image::ImageError> {
    let img = image::open(path)?;
    let pixel_data: Vec<u8> = img.pixels().flat_map(|(_, _, pixel)| {
        let rgba = pixel.to_rgba();
        vec![rgba[0], rgba[1], rgba[2], rgba[3]]
    }).collect();
    Ok(pixel_data)
}



fn binary(text: String) -> String {
    text.chars().map(|c| format!("{:08b}", c as u8)).collect::<Vec<String>>().join("")
}

/**fn unbinary(bin: String) -> String {
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
**/
fn data_encode(strng: &str) -> String {
    let bin = binary(strng.to_string());
   bin // base_encode(&bin)
}

pub fn handler() -> Result<Vec<u8>, &'static str> {
   let data = "hello ness";
    let f_path = "./src/assets/img.jpg";
    let mut channel = 2u8;
   // let pixels = read_image(f_path).map_err(|_| "error reading image")?;
    //let message = &jf.message;
    let message = data_encode(data);
    let mut pixels = read_image(f_path).map_err(|_| "error reading image")?;

    // Assuming LSB::encode modifies the pixels and returns a Result<(), ErrorType>
     let lsb_enc = LSB::encode(&mut pixels, &message,&mut channel).unwrap();

    let d = Data::new(channel, pixels, &message);
    println!("{:?}", d);

    Ok(lsb_enc)
}
