pub mod fileio {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn read_file(filename: String) -> Vec<String> {
        let mut vec = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(num) = line {
                    vec.push(num);
                }
            }
        }
        vec
    }
    pub fn read_file_int(filename: String) -> Vec<i32> {
        let mut vec = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(num) = line {
                    vec.push(num.parse().expect("Expected an integer"));
                }
            }
        }
        vec
    }

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
