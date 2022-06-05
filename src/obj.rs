use std::fs::File;
use crate::shapes::{Triangle, Mesh};
use std::io::{Lines, BufReader, BufRead};
use std::str::SplitWhitespace;
use nalgebra::{Vector3};

pub struct ObjParser {
    pub filename: String
}

impl ObjParser {

    pub fn extract_obj(&self) -> Mesh{
        let file = File::open(&self.filename).unwrap();
        let reader = BufReader::new(file);

        let mut vert_indices : Vec<Vec<i32>> = Vec::new();
        let mut mesh: Mesh = Mesh::new();

        let mut unwrapped_line;

        for (index, line) in reader.lines().enumerate(){
            unwrapped_line = line.unwrap();
            let mut line = unwrapped_line.split_whitespace();

            match line.next() {
                Some("v") => {
                    let vec: Vec<f64> = line.map(|x| x.parse().unwrap()).collect();
                    mesh.vertices.push(Vector3::new(vec[0], vec[1], vec[2]));
                },
                Some("f") => {
                    let indices: Vec<usize> = line.map(|x| x.split('/').next().unwrap().parse().unwrap()).collect();
                    mesh.triangles.push(Triangle{
                        a: mesh.vertices[indices[0]-1],
                        b: mesh.vertices[indices[1]-1],
                        c: mesh.vertices[indices[2]-1],
                    });
                },
                _ => {}
            }
        }

        mesh
    }
}
