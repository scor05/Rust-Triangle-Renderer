use obj::{Obj, ObjError};
use raylib::prelude::Vector3;
use std::collections::HashSet;
use std::path::Path;

pub struct Model {
    pub vertices: Vec<Vector3>,
    pub edges: HashSet<(usize, usize)>,
    pub center: Vector3,
}

impl Model {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ObjError> {
        let obj = Obj::load(path)?;
        Ok(Self::from_obj(obj))
    }

    pub fn from_obj(obj: Obj) -> Self {
        let vertices: Vec<Vector3> = obj
            .data
            .position
            .iter()
            .map(|pos| Vector3::new(pos[0], pos[1], pos[2]))
            .collect();

        assert!(!vertices.is_empty(), "OBJ has no vertices.");

        let center = Self::get_center(&vertices);
        let edges = Self::get_edges(&obj);

        Self {
            vertices,
            edges,
            center,
        }
    }

    pub fn get_center(vertices: &Vec<Vector3>) -> Vector3 {
        let mut maximum = vertices[0];
        let mut minimum = vertices[0];

        for v in &vertices[1..] {
            maximum.x = maximum.x.max(v.x);
            maximum.y = maximum.y.max(v.y);
            maximum.z = maximum.z.max(v.z);

            minimum.x = minimum.x.min(v.x);
            minimum.y = minimum.y.min(v.y);
            minimum.z = minimum.z.min(v.z);
        }

        (maximum + minimum) * 0.5
    }

    pub fn get_edges(obj: &Obj) -> HashSet<(usize, usize)> {
        let mut edges = HashSet::new();

        for object in &obj.data.objects {
            for group in &object.groups {
                for polygon in &group.polys {
                    let indices = &polygon.0;

                    if indices.len() < 2 {
                        continue;
                    }

                    for index in 0..indices.len() {
                        let first = indices[index].0;
                        let second = indices[(index + 1) % indices.len()].0;

                        // guardar índice más pequeño
                        let edge = (first.min(second), first.max(second));

                        edges.insert(edge);
                    }
                }
            }
        }

        edges
    }
}
