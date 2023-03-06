use super::types::*;
use std::error::Error;

pub fn file_extension(path: &str) -> Result<&str, Box<dyn Error>> {
    Ok(path.split('.').last().ok_or("Invalid file extension")?)
}

pub fn load_mesh(path: &str) -> Result<GeometryType, Box<dyn Error>> {
    Ok(GeometryType::Mesh {
        format: file_extension(path)?.to_string(),
        data: std::fs::read_to_string(path)?,
    })
}

pub fn scene_text(texture: TextureType) -> LumpedObject {
    LumpedObject::builder()
        .texture(Texture::new(texture))
        .geometry(Geometry::new(GeometryType::Plane {
            width: 10.0,
            height: 10.0,
            width_segments: 1,
            height_segments: 1,
        }))
        .material(
            Material::builder()
                .material_type(MaterialType::MeshPhong)
                .transparent(true)
                .build(),
        )
        .build()
}
