use std::fs::File;
use crate::shapes::{Triangle, Mesh};
use std::io::{Lines, BufReader, BufRead};
use std::str::SplitWhitespace;
use vek::Vec3;

pub fn extract_obj(filename: String) -> Mesh {
    // accepts triangulated mesh with normals and texture coordinates
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut mesh: Mesh = Mesh::new();
    let mut unwrapped_line;

    for (index, line) in reader.lines().enumerate() {
        unwrapped_line = line.unwrap();
        let mut line = unwrapped_line.split_whitespace();

        match line.next() {
            Some("v") => {parse_vertex(line, &mut mesh)}
            Some("f") => {parse_faces(line, &mut mesh)}
            _ => {}
        }
    }

    mesh
}

fn parse_vertex(line: SplitWhitespace, mesh: &mut Mesh){
    let vec: Vec<f64> = line.map(|x| x.parse().unwrap()).collect();
    mesh.vertices.push(Vec3::new(vec[0], vec[1], vec[2]));
}

fn parse_faces(line: SplitWhitespace, mesh: &mut Mesh){
    let f_indices: Vec<Vec<&str>> = line.map(|x| x.split('/').collect::<Vec<&str>>()).collect();

    mesh.triangles.push(Triangle {
        a: f_indices[0][0].parse().unwrap(),
        b: f_indices[1][0].parse().unwrap(),
        c: f_indices[2][0].parse().unwrap(),
    });
}