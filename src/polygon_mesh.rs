use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;

pub fn generate_triangles_mesh(
    vertices: &[Point3],
    indices: &[usize],
    material: Arc<dyn Material>,
) -> HittableList {
    let mut triangles = HittableList::new();

    for chunk in indices.chunks_exact(3) {
        let v0 = vertices[chunk[0]];
        let v1 = vertices[chunk[1]];
        let v2 = vertices[chunk[2]];

        triangles.add(Arc::new(Triangle::new(v0, v1, v2, material.clone())));
    }
    triangles
}
pub fn add_polygon(
    vertices: &[Point3],
    indices: &[usize],
    material: Arc<dyn Material>,
    triangles: &mut HittableList,
) {
    if indices.len() < 3 {
        return;
    }
    let v0 = vertices[indices[0]];
    for i in 1..(indices.len() - 1) {
        let v1 = vertices[indices[i]];
        let v2 = vertices[indices[i + 1]];

        triangles.add(Arc::new(Triangle::new(v0, v1, v2, material.clone())));
    }
}
