extern crate bmp;
use bmp::Image;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let mut f = std::fs::File::open(args[1].to_string()).unwrap();

    let f = bmp::from_reader(&mut f).unwrap();
    println!(
        "Read a BMP with dimensions {}x{}px",
        f.get_width(),
        f.get_height()
    );

    todo!();
}
