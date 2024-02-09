use std::{
    fs::{read, File}, io::{BufWriter, Write}, path::Path};
use png::{Decoder, Encoder};
mod bitutil;
use bitutil::BitUtils;
use clap::Parser;

macro_rules! info {
    ($($arg:tt)*) => {{
        println!("[INFO] {}", format_args!($($arg)*));
    }};
}
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("[SUCCESS] {}", format_args!($($arg)*));
    }};
}
macro_rules! error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] {}", format_args!($($arg)*));
    }};
}

// function to check if the file associated with the 
// secret message passed in ends in .txt
fn is_txt(path: &str) -> Result<String, String> {
    if !path.ends_with(".txt") {
        return Err(format!("{} is not a .txt file. CLI only works with text files", path));
    }
    Ok(path.to_string())
}

// function to check if the file associated with the 
// secret message passed in ends in .txt
fn is_png(path: &str) -> Result<String, String> {
    if !path.ends_with(".png") {
        return Err(format!("{} is not a .txt file. CLI only works with PNG images", path));
    }
    Ok(path.to_string())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If the program should read or hide a message
    #[clap(short, long, value_parser(["read", "write"]))]
    mode: String,

    /// The path to the image to use [must be .png]
    #[clap(short, long, value_parser(clap::builder::ValueParser::new(is_png)))]
    input_image: String,

    /// The path to output the image [must be in "write" mode, must be .png]
    #[clap(short, long, value_parser(clap::builder::ValueParser::new(is_png)))]
    output_image: Option<String>,

    /// The path to the file with the secret message [must be .txt]
    #[clap(short('f'), long, value_parser(clap::builder::ValueParser::new(is_txt)))]
    message_file: String,
}


fn encode(src: &str, msg_src: &str, dest: &str) {
    info!("Writing the message to {}", dest);

    // Load file contents as bits
    let message_bytes = read(msg_src).unwrap();
    let message_bits = BitUtils::make_bits(message_bytes);

    // get message size (stored as a 4 byte value)
    let message_size = BitUtils::byte_u32_to_bit(message_bits.len() as u32);

    // add size to the message
    let mut complete_message = Vec::new();
    complete_message.extend_from_slice(&message_size); // adds the message size
    complete_message.extend_from_slice(&message_bits); // adds the message bits

    // open the image

    // decodes the image where we are going to hide our data
    let decoder = Decoder::new(File::open(src).unwrap());

    // get the reader as a muutable to perform operations on it
    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    // check the capacity of the image
    if complete_message.len() > reader.output_buffer_size() {
        error!("Image is too small: message size is {} and image allows for {}",
            complete_message.len(),
            reader.output_buffer_size());
        return;
    } else {
        let percent_used = (complete_message.len() as f64 / reader.output_buffer_size() as f64) * 100.0;
        info!("Image is large enough: message size is {} bytes  and image allows for {} bytes. Percent used: ~{}%",
            complete_message.len(),
            reader.output_buffer_size(),
            percent_used.round());
    }

    // encode the message into the image using LSB
    info!("Encoding the message into the image");
    let mut data = vec![0; reader.output_buffer_size()]; // create a data vector
    reader.next_frame(&mut data).unwrap(); // get the image data

    let info = reader.info(); // get the image metadata
    let mut i = 0; // tracker for the current bit

    // iterate over the bits of the message
    for bit in complete_message.iter() {
        if *bit == 1 && data[i] % 2 == 0 {
            // Check if the current bit of the message is equal 
            // to 1 and check if the LSB of the image data is 0
            // If both are true, then we need to flip the data to 1 (+1)
            data[i] += 1;
        } else if *bit == 0 && data[i] % 2 == 1 {
            // Check if the current bit of the message is equal
            // to 0 and check if the LSB of the image data is 1
            // If both are true, then we need to flip the data to 0 (-1)
            data[i] -= 1;
        }
        i += 1; // increment the tracker
    }

    // save the image

    // creates a destination file
    let encoded_img = File::create(dest).unwrap();

    // creates an encoder for the image
    let mut image_encoder = Encoder::new(BufWriter::new(encoded_img), info.width, info.height);

    // sets the color type of the image
    image_encoder.set_color(info.color_type);
    image_encoder.set_depth(info.bit_depth);

    image_encoder
        .write_header()
        .unwrap()
        .write_image_data(&data)
        .unwrap();

    // print success message
    success!("Image encoded successfully");
}

fn decode(src: &str, dest: &str) {
    info!("Reading the message from {}", src);
    // decodes the data from the image with the message
    let decoder = Decoder::new(File::open(src).unwrap());
    let mut binding = decoder.read_info();

    // get the reader as a muutable to perform operations on it
    let reader = binding.as_mut().unwrap();

    // read the data in the reader to the data vector (we will recive bytes)
    let mut data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut data).unwrap();

    // split the first 32 bits to check the message length,
    // then split the rest of the data to get the message bits
    let (message_len, image_data) = data.split_at(32);

    // transform the message length bytes to its decimal value,
    // signifing the number of bits in the message
    let message_len = BitUtils::byte_u32_to_decimal(BitUtils::read_lsb(message_len.to_vec()));

    // get the bytes that have the LSB and dicard the rest
    let (bytes_message, _): (&[u8], &[u8]) = image_data.split_at(message_len as usize);

    // get the bit of the message by reading the LSBV of every byte
    let message_bits = BitUtils::read_lsb(bytes_message.to_vec());

    // get the bytes of the message
    let message_retrived = BitUtils::bits_to_bytes(message_bits);

    info!("Message retrived successfully. Writing to {}", dest);
    // create and save the file with the message
    let mut output_file = File::create(Path::new(dest)).unwrap();

    output_file.write_all(&message_retrived).unwrap();
    success!("File written successfully");
}

fn main() {
    let args = Args::parse();

    match args.mode.as_str() {
        "read" => {
            decode(&args.input_image, &args.message_file);
                
        },
        "write" => {
            match args.output_image {
                Some(output_image) => {
                    encode(&args.input_image, &args.message_file, output_image.as_str());
                },
                None => {
                    eprintln!("Please provide a image to write to")
                }
            }
        },
        _ => {
            panic!("Option is invalid. Please use --help to see valid flags")
        }
    }
}