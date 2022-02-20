use std::fs::File;
use std::io::{Lines, BufReader, BufRead};
use std::str::SplitWhitespace;
use crate::shapes::Vec3D;

pub struct ObjParser {
    pub filename: String
}

impl ObjParser {

    pub fn extract_obj(&self) {
        let file = File::open(&self.filename).unwrap();
        let reader = BufReader::new(file);

        let mut vert_indices : Vec<Vec<i32>> = Vec::new();
        let mut vertices: Vec<Vec3D> = Vec::new();

        let mut unwrapped_line;

        for (index, line) in reader.lines().enumerate(){
            unwrapped_line = line.unwrap();
            let mut line = unwrapped_line.split_whitespace();

            match line.next() {
                Some("v") => {
                    let vec: Vec<f32> = line.map(|x| x.parse().unwrap()).collect();
                    vertices.push(Vec3D::from_vec(vec));
                },
                Some("f") => {
                    let indices: Vec<i32> = line.map(|x| x.split('/').next().unwrap().parse().unwrap()).collect();
                    vert_indices.push(indices);
                },
                _ => {}
            }
        }
    }
}