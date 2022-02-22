extern crate bmp;

struct Nonogram {
    img: bmp::Image,
    solution: Vec<Vec<bool>>,
    width: u32,
    height: u32,
    column_hints: Vec<u32>,
    row_hints: Vec<u32>,
}

// This colour (white) will be used for transparent. Or nothing.
const TRANS: bmp::Pixel = bmp::Pixel {
    r: 0xFF,
    g: 0xFF,
    b: 0xFF,
};

impl Nonogram {
    fn new(img: bmp::Image) -> Self {
        let mut solution = Vec::new();

        // img gets moved into struct. Can't do this after. Besides, it's useful for loop.
        let width = img.get_width();
        let height = img.get_height();

        // Doing with y outside because of how bmps are built
        // Top left is 0, 0
        for y in 0..width {
            // Add a new vector for this row
            solution.push(Vec::<bool>::new());

            for x in 0..height {
                // Check that it isn't the colour designated as transparent.
                solution
                    .get_mut(y as usize)
                    .unwrap()
                    .push(img.get_pixel(x, y) != TRANS);
            }
        }

        Nonogram {
            img,
            solution,
            width,
            height,
            column_hints: vec![3],
            row_hints: vec![2],
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    // Open file from first arg
    let mut f = std::fs::File::open(args[1].to_string()).expect("File not found.");

    // Try to open as bmp
    let img: bmp::Image = bmp::from_reader(&mut f).unwrap();

    println!(
        "Read a BMP with dimensions {}x{}px",
        img.get_width(),
        img.get_height()
    );

    print_bmp_1bit(&img);

    let nono = Nonogram::new(img);

    println!("{:?}", nono.solution);

    todo!();
}

fn print_bmp_1bit(img: &bmp::Image) {
    // Prints the given bmp::Image as a simple ascii representation
    let width = img.get_width();
    let height = img.get_height();

    for y in 0..width {
        for x in 0..height {
            let pixel = img.get_pixel(x, y);

            if (pixel.r & pixel.g & pixel.b) == 0xFF {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!("");
    }
}
