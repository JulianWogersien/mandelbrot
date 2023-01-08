pub mod io {
    use std::{path::Path, fs::File, io::{self, BufRead}};

    #[allow(dead_code)]
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}