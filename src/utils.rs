use nalgebra::{Isometry3, Matrix3xX, Vector3};

use super::types::*;
use std::error::Error;

pub fn file_extension(path: &str) -> Result<&str, Box<dyn Error>> {
    Ok(path.split('.').last().ok_or("Invalid file extension")?)
}

// TODO: https://github.com/rdeits/MeshCat.jl/blob/master/src/mesh_files.jl
pub fn load_mesh(path: &str) -> Result<GeometryType, Box<dyn Error>> {
    Ok(GeometryType::Mesh {
        format: file_extension(path)?.to_string(),
        data: std::fs::read_to_string(path)?,
    })
}

pub fn scene_text(texture: TextureType) -> LumpedObject {
    LumpedObject::builder()
        .texture(Texture::new(texture))
        .geometries(vec![Geometry::new(GeometryType::Plane {
            width: 10.0,
            height: 10.0,
            width_segments: 1,
            height_segments: 1,
        })])
        .material(
            Material::builder()
                .material_type(MaterialType::MeshPhong)
                .transparent(true)
                .build(),
        )
        .build()
}

pub fn triad(pose: Isometry3<f64>) -> LumpedObject {
    let scale = 0.5;
    let points = Matrix3xX::<f64>::from_columns(&[
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(scale, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, scale, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, scale),
    ]);
    let colors = Matrix3xX::<f64>::from_columns(&[
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(1.0, 0.6, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.6, 1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 0.6, 1.0),
    ]);
    LumpedObject::builder()
        .geometries(vec![Geometry::new(GeometryType::Buffer {
            data: Box::new(BufferGeometryData {
                attributes: BufferGeometryAttributes {
                    position: BufferGeometryAttribute {
                        item_size: 3,
                        array: points,
                        attribute_type: "Float32Array".to_string(),
                        normalized: false,
                    },
                    color: BufferGeometryAttribute {
                        item_size: 3,
                        array: colors,
                        attribute_type: "Float32Array".to_string(),
                        normalized: false,
                    },
                    normal: None,
                    uv: None,
                },
            }),
        })])
        .material(
            Material::builder()
                .vertex_colors(true)
                .material_type(MaterialType::LineBasic)
                .build(),
        )
        .object(Object::new(pose, ObjectType::LineSegments))
        .build()
}
