use std::fs;
use clap::Parser;
use image::open;

fn main() {
    let args = Args::parse();

    let ascii_chars = ["." ,"," ,":" ,";" ,"+" ,"*" ,"?" ,"%" ,"$" ,"#" ,"@"];
    let img = match open(args.input_path) {
        Ok(image) => image,
        Err(error) => {
            println!("{:}", error);
            return;
        },
    };
    let luma_alpha = img.into_luma_alpha8();
    if luma_alpha.len() <= 0 {
        println!("Unable to load image alpha channel data");
        return;
    }
    let mut current_y = 0;
    let mut output = String::new();

    for data in luma_alpha.enumerate_pixels() {
        let y = data.1;
        let lumaa = data.2;
        let alpha = lumaa.0[0] as f32 / lumaa.0[1] as f32;  
        let index: usize = (alpha * (ascii_chars.len() as f32 - 1.0)) as usize;
        output.push_str(ascii_chars[index]);
        if current_y != y {
            output.push_str("\n");
            current_y = y;
        }
    }
    fs::write(args.output_name + ".txt", output).expect("Unable to write file");
}

#[derive(Parser)]
struct Args {
    input_path: String,
    output_name: String,
}
