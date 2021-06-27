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
// ? NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// ? run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
// !   cargo run --release blur image.png blurred.png
//
// ? NOTE 2: This is how you parse a number from a string (or crash with a
// ? message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use rand::Rng;
use clap::{
    App,
    Arg
};

const DEFAULT_BLUR_VALUE: f32 = 2.0;
const DEFAULT_BRIGHTEN_VALUE: i32 = 2;
const DEFAULT_CROP_VALUE: (u32, u32, u32, u32) = (5, 5, 5, 5);
const DEFAULT_ROTATE_VALUE: u32 = 90;
const DEFAULT_GENERATE_VALUE: u8 = 10;

/**
 * Main function
 */
fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // ! Challenge: If you're feeling really ambitious, you could delete this code
    // ! and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    // TODO: use "clap" library!
    // TODO: improve cornercase error handeling + msgs
    /*let mut args: Vec<String>;
    let subcommand: String;

    args = std::env::args().skip(1).collect();

    if args.is_empty() {
        print_usage_and_exit();
    }
    
    subcommand = args.remove(0);
    match subcommand.as_str() {
        // Blur option handler
        "blur" => {
            let (infile, outfile): (String, String);
            let blur_value: f32;

            if args.len() < 2 {
                print_usage_and_exit();
            }

            infile = args.remove(0);
            outfile = args.remove(0);

            if args.len() >= 1 {
                blur_value = args.remove(0).parse().unwrap();
            } else {
                blur_value = DEFAULT_BLUR_VALUE;
                println!("uses default blure value: {}!", DEFAULT_BLUR_VALUE);
            }

            blur(infile, outfile, blur_value);
        },

        // Brighten option handler
        "brighten" => {
            let (infile, outfile): (String, String);
            let brighten_value: i32;

            if args.len() < 2 {
                print_usage_and_exit();
            }

            infile = args.remove(0);
            outfile = args.remove(0);

            if args.len() >= 1 {
                brighten_value = args.remove(0).parse().unwrap();
            } else {
                brighten_value = DEFAULT_BRIGHTEN_VALUE;
                println!("uses default brighten value: {}!", DEFAULT_BRIGHTEN_VALUE);
            }

            brighten(infile, outfile, brighten_value);
        },

        // Crop option handler
        // TODO: use nicer aproach to parse x y width and height
        "crop" => {
            let (infile, outfile): (String, String);
            let mut crop_value: Vec<u32> = Vec::new();

            if args.len() < 2 {
                print_usage_and_exit();
            }

            infile = args.remove(0);
            outfile = args.remove(0);

            if args.len() >= 4 {
                for _i in 0..4 {
                    crop_value.push(args.remove(0).parse().unwrap());
                }
            } else {
                crop_value = vec!(
                    DEFAULT_CROP_VALUE.0, 
                    DEFAULT_CROP_VALUE.1, 
                    DEFAULT_CROP_VALUE.2, 
                    DEFAULT_CROP_VALUE.3
                );
                println!("uses default crop value: {:?}!", crop_value);
            }

            crop(infile, outfile, crop_value);
        },

        // Rotate option handler
        // TODO: Improve options (90, 180, 270) handler
        "rotate" => {
            let (infile, outfile): (String, String);
            let rotate_value: u32;

            if args.len() < 2 {
                print_usage_and_exit();
            }

            infile = args.remove(0);
            outfile = args.remove(0);

            if args.len() >= 1 {
                rotate_value = match args.remove(0).parse().unwrap() {
                    270 => 270,
                    180 => 180,
                    90 => 90,
                    _ => {
                        println!("uses default rotate value: {}°!", DEFAULT_ROTATE_VALUE);
                        DEFAULT_ROTATE_VALUE
                    },
                };
            } else {
                rotate_value = DEFAULT_ROTATE_VALUE;
                println!("uses default brighten value: {}°!", DEFAULT_ROTATE_VALUE);
            }

            rotate(infile, outfile, rotate_value);
        },

        // Invert option handler
        "invert" => {
            let (infile, outfile): (String, String);

            if args.len() < 2 {
                print_usage_and_exit();
            }

            infile = args.remove(0);
            outfile = args.remove(0);

            invert(infile, outfile);
        },

        // Grayscale option handler
        "grayscale" => {
            let (infile, outfile): (String, String);

            if args.len() < 2 {
                print_usage_and_exit();
            }

            infile = args.remove(0);
            outfile = args.remove(0);

            grayscale(infile, outfile);
        },

        // Fractal option handler
        "fractal" => {
            let outfile: String;

            if args.len() != 1 {
                print_usage_and_exit();
            }

            outfile = args.remove(0);
            fractal(outfile);
        }

        // Generate option handler
        "generate" => {
            let outfile: String;
            let color: u8;

            if args.len() < 1 {
                print_usage_and_exit();
            }

            outfile = args.remove(0);

            color = if args.len() >= 1 {
                args.remove(0).parse().unwrap()
            } else {
                println!("uses default generate value: {}!", DEFAULT_GENERATE_VALUE);
                DEFAULT_GENERATE_VALUE
            };

            generate(outfile, color);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        },
    }*/

    let matches = App::new("mirage")
        .version("1.0")
        .author("Jan Macku <jamacku@redhat.com>")
        .about("Does awesome things")
        .subcommand(App::new("blur")
            .about("Blur feature.")
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .value_name("BLUR_VALUE")
                .takes_value(true)
                .required(false)
                .default_value("2.0")
                .about("Blure value e.g. '10.0'.")))
        .subcommand(App::new("brighten")
            .about("Brighten feature.")
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .value_name("BRIGHTEN_VALUE")
                .takes_value(true)
                .required(false)
                .default_value("2")
                .about("Brighten value e.g. '10'.")))
        .subcommand(App::new("crop")
            .about("Crop feature.")
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .value_name("CROP_VALUE")
                .takes_value(true)
                .required(false)
                .number_of_values(4)
                .default_value("x: 5, y: 5, width: 5, height: 5")
                .about("Crop value e.g. '10 10 10 10'.")))
        .subcommand(App::new("rotate")
            .about("Rotate feature.")
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .value_name("ROTATE_VALUE")
                .takes_value(true)
                .required(false)
                .default_value("90")
                .about("Rotate value in degrees. Option accepts only values '90', '180' and '270'.")))
        .subcommand(App::new("invert")
            .about("Invert feature."))
        .subcommand(App::new("greyscale")
            .about("Greyscale feature."))
        .subcommand(App::new("fractal")
            .about("Fractal feature."))
        .subcommand(App::new("generate")
            .about("Generate feature.")
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .value_name("Generate_VALUE")
                .takes_value(true)
                .required(false)
                .default_value("10")
                .about("Generate value.")))
        .get_matches();

    /* Check HWADDR */
    if let Some(o) = matches.value_of("hwaddr") {
        println!("Value for HWADDR: {}", o);
    }

}

/**
 * Function for printing usage messages
 */
fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

/**
 * Function for bluring pictures
 */
fn blur(infile: String, outfile: String, value: f32) {
    let (img, img2): (image::DynamicImage, image::DynamicImage);
    
    // Here's how you open an existing image file
    img = image::open(infile).expect("Failed to open INFILE.");
    img2 = img.blur(value);

    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

/**
 * Function for brightening pictures
 */
fn brighten(infile: String, outfile: String, value: i32) {
    let (img, img2): (image::DynamicImage, image::DynamicImage);
    
    // Here's how you open an existing image file
    img = image::open(infile).expect("Failed to open INFILE.");
    img2 = img.brighten(value);
    
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

/**
 * Function for cropping pictures
 */
fn crop(infile: String, outfile: String, coordinates: Vec<u32>) {
    let (mut img, img2): (image::DynamicImage, image::DynamicImage);
    let (x, y, width, height): (u32, u32, u32, u32) = (
        coordinates[0], 
        coordinates[1], 
        coordinates[2], 
        coordinates[3]
    );
    
    // Here's how you open an existing image file
    img = image::open(infile).expect("Failed to open INFILE.");
    img2 = img.crop(x, y, width, height);
    
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

/**
 * Function for rotating pictures
 */
fn rotate(infile: String, outfile: String, value: u32) {
    let (img, img2): (image::DynamicImage, image::DynamicImage);
    
    // Here's how you open an existing image file
    img = image::open(infile).expect("Failed to open INFILE.");
    img2 = match value {
        270 => img.rotate270(),
        180 => img.rotate180(),
        _ => img.rotate90(),
    };
    
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

/**
 * Function for inverting pictures
 */
fn invert(infile: String, outfile: String) {
    let mut img: image::DynamicImage;
    
    // Here's how you open an existing image file
    img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();

    // Here's how you save an image to a file.
    img.save(outfile).expect("Failed writing OUTFILE.");
}

/**
 * Function for graying pictures
 */
fn grayscale(infile: String, outfile: String) {
    let (img, img2): (image::DynamicImage, image::DynamicImage);
    
    // Here's how you open an existing image file
    img = image::open(infile).expect("Failed to open INFILE.");
    img2 = img.grayscale();

    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, color: u8) {
    let width: u32 = 800;
    let height: u32 = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red: u8 = (color as f32 * 0.3) as u8 * (0.2 * x as f32) as u8;
        let blue: u8 = (color as f32 * 0.3) as u8 * (0.3 * x as f32) as u8;

        // Just some stupid stuff :D
        let green: u8;
        let mut rng = rand::thread_rng();
        green = if x % 3 == 0 && y % 3 > 0 {
            rng.gen::<u8>()
        } else {
            125
        };

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    // Here's how you save an image to a file.
    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width: u32 = 800;
    let height: u32 = 800;

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
