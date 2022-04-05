use std::fs::File;
use crate::shapes::{Vec3D, Triangle, Mesh};
use std::io::{Lines, BufReader, BufRead};
use std::str::SplitWhitespace;

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
                    let vec: Vec<f32> = line.map(|x| x.parse().unwrap()).collect();
                    mesh.vertices.push(Vec3D::from_vec(vec));
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
