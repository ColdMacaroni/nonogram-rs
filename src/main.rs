extern crate bmp;

#[derive(Debug)]
struct Nonogram {
    img: bmp::Image,
    solution: Vec<Vec<bool>>,
    width: u32,
    height: u32,
    column_hints: Vec<Vec<u32>>,
    row_hints: Vec<Vec<u32>>,
}

// This colour (white) will be used for transparent. Or nothing.
const TRANS: bmp::Pixel = bmp::Pixel {
    r: 0xFF,
    g: 0xFF,
    b: 0xFF,
};

impl Nonogram {
    fn new(img: bmp::Image) -> Self {
        let mut column_hints = Vec::new();
        let mut row_hints = Vec::new();
        let mut solution = Vec::new();

        // img gets moved into struct. Can't do this after. Besides, it's useful for loop.
        let width = img.get_width();
        let height = img.get_height();

        // Column hints needs a vector for each column. Can't be added on each iteration of the loop like with row hints
        // 0s will have to be filtered out later
        for _ in 0..width {
            column_hints.push(vec![0 as u32]);
        }

        // I'm trying to keep most of the logic in this loop
        // Doing with y outside because of how bmps are built
        // Top left is 0, 0
        for y in 0..width {
            // Hints count how many contiguous blocks there are.
            let mut row_hint_counter: u32 = 0;

            // Add a new vector for this row
            row_hints.push(Vec::new());
            solution.push(Vec::<bool>::new());

            for x in 0..height {
                // Store so that we dont need to do all the unwrapping and getting to access it in the if
                let row: &mut Vec<bool> = solution.get_mut(y as usize).unwrap();

                // Check that it isn't the colour designated as transparent.
                if img.get_pixel(x, y) != TRANS {
                    row.push(true);

                    // Update row hint
                    row_hint_counter += 1;

                    // Update column hint
                    *column_hints
                        .get_mut(x as usize)
                        .unwrap()
                        .last_mut()
                        .unwrap() += 1;
                } else {
                    row.push(false);

                    // Update with the new hint only if there is new stuff.
                    // This avoids pushing 0s into the row array.
                    if row_hint_counter > 0 {
                        row_hints
                            .get_mut(y as usize)
                            .unwrap()
                            .push(row_hint_counter);
                        row_hint_counter = 0;
                    }

                    // Add a new thing to be used as hint for the columns
                    column_hints.get_mut(x as usize).unwrap().push(0);
                }
            }

            // Check this outside of the loop in case there's a leftover value
            if row_hint_counter > 0 {
                row_hints
                    .get_mut(y as usize)
                    .unwrap()
                    .push(row_hint_counter);
            }
        }

        // Delete superflous 0s.
        for v in column_hints.iter_mut() {
            v.retain(|&x| x != 0);
        }

        Nonogram {
            img,
            solution,
            width,
            height,
            column_hints,
            row_hints,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Open file from first arg
    let mut f = std::fs::File::open(args[1].to_string()).expect("File not found.");

    // Try to open as bmp
    let img: bmp::Image = bmp::from_reader(&mut f).unwrap();

    // Convert
    let nono = Nonogram::new(img);
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
                print!("???");
            }
        }
        println!("");
    }
}
