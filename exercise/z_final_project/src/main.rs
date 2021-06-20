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

const DEFAULT_BLUR_VALUE: f32 = 2.0;
const DEFAULT_BRIGHTEN_VALUE: i32 = 2;
const DEFAULT_CROP_VALUE: (u32, u32, u32, u32) = (5, 5, 5, 5);
const DEFAULT_ROTATE_VALUE: u32 = 90;

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
    let mut args: Vec<String>;
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
                for i in 0..4 {
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

        // **OPTION**
        // Invert -- see the invert() function below
        "intert" => {
            // invert();
        },

        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" => {
            // grayscale();
        },

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
        "generate" => {
            // generate();
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        },
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


fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments and converts the image in-place, so
    // you will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // ! Challenge: parse some color data from the command-line, pass it through
    // ! to this function to use for the solid color.

    // ! Challenge 2: Generate something more interesting!

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
