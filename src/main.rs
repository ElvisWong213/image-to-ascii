use clap::{ArgAction, Parser};
use image::{open, DynamicImage};
use std::{fs, path::PathBuf};

fn main() {
    let args = Args::parse();

    let ascii_chars = [".", ",", ":", ";", "+", "*", "?", "%", "$", "#", "@"];

    // open image
    let img = match open(&args.input_path) {
        Ok(image) => {
            // resize image
            let resized_image: DynamicImage = match args.width {
                Some(new_width) => {
                    let origin_width = image.width();
                    let origin_height = image.height();
                    let ratio = new_width as f32 / origin_width as f32;
                    let new_height = (origin_height as f32 * ratio) as u32;
                    image.resize(new_width, new_height, image::imageops::FilterType::Triangle)
                }
                None => image,
            };
            resized_image
        }
        Err(error) => {
            println!("{:}", error);
            return;
        }
    };

    // get image alpha channel
    let luma_alpha = img.into_luma_alpha8();

    // convert image to ascii chars
    let mut current_y = 0;
    let mut output = String::new();

    for data in luma_alpha.enumerate_pixels() {
        let y = data.1;
        let lumaa = data.2;
        let alpha = lumaa.0[0] as f32 / lumaa.0[1] as f32;
        let index: usize = (alpha * (ascii_chars.len() as f32 - 1.0)) as usize;
        output.push_str(ascii_chars[index]);
        if current_y != y {
            output.push('\n');
            current_y = y;
        }
    }

    if save_file(&args, &output) {
        println!("File saved");
    }

    // print output
    if let Some(stop_print) = args.stop_print {
        if stop_print {
            return;
        }
    }
    print!("{:}", &output);
}

fn save_file(args: &Args, output: &String) -> bool {
    if let Some(save) = args.save {
        if !save {
            return false;
        }
    }
    let output_name = match &args.output_name {
        Some(name) => name.to_owned() + ".txt",
        None => {
            let mut origin_input_path = args.input_path.to_owned();
            if !origin_input_path.set_extension("txt") {
                panic!("Unable to change the wxtension");
            }
            origin_input_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }
    };
    fs::write(output_name, output).expect("Unable to write file");
    true
}

#[derive(Parser, Debug)]
struct Args {
    /// Input image path
    #[clap(short)]
    input_path: PathBuf,

    /// Output file name (Optional)
    #[clap(short)]
    output_name: Option<String>,

    /// Output width (Optional)
    #[clap(short)]
    width: Option<u32>,

    /// Save file (Optional)
    #[clap(short, action=ArgAction::SetTrue)]
    save: Option<bool>,

    /// Stop print out result to terminal (Optional)
    #[clap(long, action=ArgAction::SetTrue)]
    stop_print: Option<bool>,
}
