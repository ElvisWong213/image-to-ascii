use std::fs;

use image::open;

fn main() {
    let ascii_chars = ["." ,"," ,":" ,";" ,"+" ,"*" ,"?" ,"%" ,"$" ,"#" ,"@"];
    let img = open("test.jpg").unwrap();
    let luma_alpha = img.into_luma_alpha8();
    let mut now_y = 0;
    let mut output = String::new();

    for data in luma_alpha.enumerate_pixels() {
        // let x = data.0;
        let y = data.1;
        let lumaa = data.2;
        let alpha = lumaa.0[0] as f32 / lumaa.0[1] as f32;  
        let index: usize = (alpha * (ascii_chars.len() as f32 - 1.0)) as usize;
        // println!("{:}, {:}, {:?}", x, y, alpha);
        // print!("{:}", ascii_chars[index]);
        output.push_str(ascii_chars[index]);
        if now_y != y {
            // print!("\n");
            output.push_str("\n");
            now_y = y;
        }
    }
    fs::write("test.txt", output).expect("Unable to write file");
}
