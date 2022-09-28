use std::fs::File;
use crate::shapes::{Triangle, Mesh};
use std::io::{Lines, BufReader, BufRead};
use std::str::SplitWhitespace;
use vek::Vec3;

pub fn extract_obj(filename: String) -> Mesh {
    // accepts triangulated mesh with normals and texture coordinates
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vert_indices: Vec<Vec<i32>> = Vec::new();
    let mut mesh: Mesh = Mesh::new();

    let mut unwrapped_line;

    for (index, line) in reader.lines().enumerate() {
        unwrapped_line = line.unwrap();
        let mut line = unwrapped_line.split_whitespace();

        match line.next() {
            Some("v") => {
                let vec: Vec<f64> = line.map(|x| x.parse().unwrap()).collect();
                mesh.vertices.push(Vec3::new(vec[0], vec[1], vec[2]));
            }
            Some("f") => {
                // let indices: Vec<usize> = line.map(|x| x.split('/').next().unwrap().parse().unwrap()).collect();
                // mesh.triangles.push(Triangle {
                //     a: mesh.vertices[indices[0] - 1],
                //     b: mesh.vertices[indices[1] - 1],
                //     c: mesh.vertices[indices[2] - 1],
                // }

                // i32::from_str(&"0").unwrap_or(0);
                let f_indices: Vec<Vec<&str>> = line.map(|x| x.split('/').collect::<Vec<&str>>()).collect();
                let triangle = Triangle{
                    a: f_indices[1][0] as usize,
                    b: f_indices[1][0] as usize,
                    c: f_indices[2][0] as usize,
                };
                // f_indices.into_iter().map(|x: Vec<&str>| x[0]).collect();
                print!(" {:?},", f_indices);

                // f_indices.clone().into_iter().for_each(|i| print!(" {:?},", i));
                // let a = line.split(|c: char| c.is_whitespace() || c == '/').collect::<Vec<&str>>();

                // f_indices.into_iter().for_each(|x| {
                //
                // });
                // let v_indices: Vec<usize> = f_indices.into_iter().split('/').next().unwrap().
                // v_indices.into_iter().for_each(|i| print!(" {},", i));
                // for i in 0..indices.len(){
                //     let vert = indices[i].split('/').next().unwrap();
                //     let a = 0;
                // }

                // mesh.triangles.push(Triangle {
                //     a: indices[0],
                //     b: indices[1],
                //     c: indices[2]
                // });
                println!();
            }
            _ => {}
        }
    }

    mesh
}
