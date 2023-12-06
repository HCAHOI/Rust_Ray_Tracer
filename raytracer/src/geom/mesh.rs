use std::path::Path;

use crate::{
    hit::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
    },
    render::mat::Material,
    world::hittablelist::HittableList,
};

use crate::geom::{
    ray::Ray,
    triangle::Triangle,
    vec3::{Point3, Vec3},
};

pub struct Mesh {
    tris: HittableList,
}

impl Mesh {
    pub fn new<M: Material + Clone + 'static>(
        positions: Vec<Vec3>,
        indices: Vec<u32>,
        material: M,
    ) -> Mesh {
        let mut tris = HittableList::default();

        for i in 0..indices.len() / 3 {
            let vertices = [
                positions[indices[i * 3] as usize],
                positions[indices[i * 3 + 1] as usize],
                positions[indices[i * 3 + 2] as usize],
            ];
            tris.push(Triangle::new(vertices, material.clone()));
        }

        Mesh { tris }
    }

    pub fn load_obj<'a, P: AsRef<Path>, M: Material + Clone + 'static>(
        path: P,
        offset: Vec3,
        scale: f64,
        material: M,
    ) -> Result<Mesh, String> {
        let models = match tobj::load_obj(path.as_ref(), &tobj::OFFLINE_RENDERING_LOAD_OPTIONS) {
            Ok((models, _)) => {
                let m = &models[0];
                println!("Loading model {}", m.name);
                let mesh = &m.mesh;
                println!("{} has {} triangles", m.name, mesh.indices.len() / 3);

                let tri_positions = mesh
                    .positions
                    .chunks(3)
                    .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64) * scale + offset)
                    .collect();

                let tri_indices = &mesh.indices;

                Mesh::new(tri_positions, tri_indices.to_vec(), material)
            }
            Err(err) => return Err(format!("Failed to load obj file: {}", err)),
        };
        Ok(models)
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.tris.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.tris.bounding_box(t0, t1)
    }
}
