extern crate bmp;

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

    todo!();
}
