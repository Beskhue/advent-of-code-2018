use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn lines_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = try!(File::open(filename));
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}
