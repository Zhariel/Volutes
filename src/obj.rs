use std::fs::File;
use std::io::{Lines, BufReader, BufRead};
use std::str::SplitWhitespace;

pub struct ObjParser {
    filename: String
}

impl ObjParser {

    fn read_lines(&self) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate(){
            let line = line.unwrap().split_whitespace();

            self.parse_line(line)
        }
    }

    fn parse_line(&self, line: SplitWhitespace){

    }
}