// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use std::thread;
use::clap::{Arg, App, SubCommand};
use image::{DynamicImage};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/

    let args = App::new("imager")
        .arg(
            Arg::with_name("input")
            .short("i")
            .help("Input files.")
            .multiple(true)
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("output")
            .short("o")
            .help("Output files.")
            .multiple(true)
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("blur")
            .short("b")
            .long("blur")
            .help("Blur image by some amount. [float]")
            .takes_value(true)
            .number_of_values(1)
            .required(false)
        )
        .arg(
            Arg::with_name("brighten")
            .short("a")
            .long("brighten")
            .help("Brighten image by some amount. [integer]")
            .takes_value(true)
            .number_of_values(1)
            .required(false)
        )
        .arg(
            Arg::with_name("crop")
            .short("c")
            .long("crop")
            .help("Crop image. Takes dimensions: x, y, w, and h. [uinteger]")
            .takes_value(true)
            .number_of_values(4)
            .required(false)
        )
        .arg(
            Arg::with_name("rotate")
            .short("r")
            .long("rotate")
            .help("Rotate image.")
            .takes_value(true)
            .number_of_values(1)
            .possible_values(&["90", "180", "270"])
            .required(false)
        )
        .arg(
            Arg::with_name("invert")
            .short("v")
            .long("invert")
            .help("Invert image.")
            .required(false)
        )
        .arg(
            Arg::with_name("grayscale")
            .short("g")
            .long("grayscale")
            .help("Grayscale image.")
            .required(false)
        )
        .arg(
            Arg::with_name("fractal")
            .short("f")
            .long("fractal")
            .help("Make fractal.")
            .required(false)
        )
        .arg(
            Arg::with_name("generate")
            .short("n")
            .long("gen")
            .help("Generate image.")
            .required(false)
        )
        .get_matches_safe()
        .unwrap_or_else(|e| e.exit() );
    
    let inputs: Vec<&str> = args.values_of("input").unwrap().collect();
    let outputs: Vec<&str> = args.values_of("output").unwrap().collect();

    if inputs.len() != outputs.len() {
        println!("Inputs must be same length as outputs.");
        std::process::exit(-1);
    }
    
    for (input_file, output_file) in inputs.iter().zip(outputs) {
        let mut img = image::open(input_file).expect("Failed to open INFILE.");
        // blur image
        
        if args.is_present("blur") {
            let blur_amt: f32 = args.value_of("blur")
                .unwrap()
                .parse::<f32>()
                .expect("Unable to convert blur amount to float.");
            img = blur(img, blur_amt);
        };
        if args.is_present("brighten") {
            let brighten_amt: i32 = args.value_of("brighten")
                .unwrap()
                .parse::<i32>()
                .expect("Unable to convert brighten amount to integer.");
            img = brighten(img, brighten_amt);
        };
        if args.is_present("crop") {
            let crop_dims: Vec<u32> = args.values_of("crop")
                .unwrap()
                .map(|dim| dim.parse().expect("Unable to convert to u32."))
                .collect();
            img = crop(img, crop_dims);
        };
        if args.is_present("rotate") {
            let rot_angle: i32 = args.value_of("rotate")
                .unwrap()
                .parse::<i32>()
                .expect("Unable to coerce rotation angle to integer.");
            img = rotate(img, rot_angle);
        };

        img.save(output_file).expect("Unable to save file.");
        println!("Finished processing {} into {}...", input_file, output_file);    
    }
}

fn blur(mut in_img: DynamicImage, blur_amt: f32) -> DynamicImage {
    in_img.blur(blur_amt)
}

fn brighten(mut in_img: DynamicImage, brighten_amt: i32) -> DynamicImage {
    in_img.brighten(brighten_amt)
}

fn crop(mut in_img: DynamicImage, crop_dims: Vec<u32>) -> DynamicImage {
    let (x, y, width, height) = (crop_dims[0], crop_dims[1], crop_dims[2], crop_dims[3]);
    in_img.crop(x, y, width, height)
}

fn rotate(mut in_img: DynamicImage, rotation: i32) -> DynamicImage {
    let rot_img = match rotation {
        90 => in_img.rotate90(),
        180 => in_img.rotate180(),
        270 => in_img.rotate270(),
        _ => in_img
    };
    rot_img
}

fn invert(mut in_img: DynamicImage) -> DynamicImage {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
    in_img.invert();
    in_img
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
