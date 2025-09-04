use std::{fs, io::Read, path::Path, time::{self, Duration, Instant}};

fn main() {
    let start = Instant::now();
    let path = Path::new("./resources/measurements.txt");
    let mut f = fs::File::open(path).unwrap();
    let mut buf = vec![];

    let size = f.read_to_end(&mut buf).unwrap();
    println!("size of buffer: {size}");
    println!("time taken to read: {}", Instant::now().duration_since(start).as_nanos());

}
