use std::error::Error;

use base64::{engine::general_purpose, Engine as _};
use log::info;
use nalgebra::{Isometry3, Matrix3xX, Matrix4, Translation3, UnitQuaternion, Vector3, Vector4};
use serde::ser::{SerializeSeq, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "type")]
    pub metadata_type: String,
    pub version: f64,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            metadata_type: "Object".to_string(),
            version: 4.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BufferGeometryAttribute {
    pub item_size: usize,
    pub attribute_type: String,
    // TODO: ext type?
    pub array: Matrix3xX<f64>,
    pub normalized: bool,
}
impl Serialize for BufferGeometryAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BufferGeometryAttribute", 4)?;
        state.serialize_field("itemSize", &self.item_size)?;
        state.serialize_field("type", &self.attribute_type)?;
        // Using nalgebra's serialization will save it as [.., number of rows, number of columns]
        // which is not what we want
        state.serialize_field("array", &self.array.as_slice())?;
        state.serialize_field("normalized", &self.normalized)?;
        state.end()
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct BufferGeometryAttributes {
    pub position: BufferGeometryAttribute,
    pub color: BufferGeometryAttribute,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<BufferGeometryAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv: Option<BufferGeometryAttribute>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BufferGeometryData {
    pub attributes: BufferGeometryAttributes,
}

// https://threejs.org/docs/#api/en/geometries/
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum GeometryType {
    // https://threejs.org/docs/#api/en/core/BufferGeometry
    #[serde(rename = "BufferGeometry")]
    Buffer { data: Box<BufferGeometryData> },
    #[serde(rename = "_meshfile_geometry")]
    Mesh { format: String, data: String },
    #[serde(rename = "BoxGeometry")]
    Box { width: f64, height: f64, depth: f64 },
    // TODO: Unsupported by meshcat
    // #[serde(rename = "CapsuleGeometry")]
    // Capsule {
    //     radius: f64,
    //     length: f64,
    //     #[serde(rename = "radialSegments")]
    //     radial_segments: u32,
    //     #[serde(rename = "capSegments")]
    //     cap_segments: u32,
    // },
    #[serde(rename = "CircleGeometry")]
    Circle {
        radius: f64,
        segments: u32,
        #[serde(rename = "thetaStart")]
        theta_start: f64,
        #[serde(rename = "thetaLength")]
        theta_length: f64,
    },
    #[serde(rename = "ConeGeometry")]
    Cone {
        radius: f64,
        height: f64,
        #[serde(rename = "radialSegments")]
        radial_segments: u32,
        #[serde(rename = "heightSegments")]
        height_segments: u32,
        #[serde(rename = "thetaStart")]
        theta_start: f64,
        #[serde(rename = "thetaLength")]
        theta_length: f64,
    },
    #[serde(rename = "CylinderGeometry")]
    Cylinder {
        #[serde(rename = "radiusTop")]
        radius_top: f64,
        #[serde(rename = "radiusBottom")]
        radius_bottom: f64,
        height: f64,
        #[serde(rename = "radialSegments")]
        radial_segments: u32,
        #[serde(rename = "heightSegments")]
        height_segments: u32,
        #[serde(rename = "thetaStart")]
        theta_start: f64,
        #[serde(rename = "thetaLength")]
        theta_length: f64,
    },
    #[serde(rename = "DodecahedronGeometry")]
    Dodecahedron { radius: f64, detail: u32 },
    #[serde(rename = "IcosahedronGeometry")]
    Icosahedron { radius: f64, detail: u32 },
    #[serde(rename = "OctahedronGeometry")]
    Octahedron { radius: f64, detail: u32 },
    #[serde(rename = "PlaneGeometry")]
    Plane {
        width: f64,
        height: f64,
        #[serde(rename = "widthSegments")]
        width_segments: u32,
        #[serde(rename = "heightSegments")]
        height_segments: u32,
    },
    #[serde(rename = "RingGeometry")]
    Ring {
        #[serde(rename = "innerRadius")]
        inner_radius: f64,
        #[serde(rename = "outerRadius")]
        outer_radius: f64,
        #[serde(rename = "thetaSegments")]
        theta_segments: u32,
        #[serde(rename = "phiSegments")]
        phi_segments: u32,
        #[serde(rename = "thetaStart")]
        theta_start: f64,
        #[serde(rename = "thetaLength")]
        theta_length: f64,
    },
    #[serde(rename = "SphereGeometry")]
    Sphere {
        radius: f64,
        #[serde(rename = "widthSegments")]
        width_segments: u32,
        #[serde(rename = "heightSegments")]
        height_segments: u32,
    },
    #[serde(rename = "TetrahedronGeometry")]
    Tetrahedron { radius: f64, detail: u32 },
    #[serde(rename = "TorusGeometry")]
    Torus {
        radius: f64,
        tube: f64,
        #[serde(rename = "radialSegments")]
        radial_segments: u32,
        #[serde(rename = "tubularSegments")]
        tubular_segments: u32,
    },
}

// properties??
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MaterialType {
    #[serde(rename = "MeshBasicMaterial")]
    MeshBasic,
    #[serde(rename = "MeshPhongMaterial")]
    MeshPhong,
    #[serde(rename = "MeshLambertMaterial")]
    MeshLambert,
    #[serde(rename = "MeshToonMaterial")]
    MeshToon,
    #[serde(rename = "LineBasicMaterial")]
    LineBasic,
    #[serde(rename = "PointsMaterial")]
    Points { size: f64 },
}

// https://threejs.org/docs/index.html#api/en/materials/Material
#[derive(Clone, Debug, TypedBuilder, Serialize, Deserialize)]
pub struct Material {
    #[builder(default = Uuid::new_v4(), setter(skip))]
    pub uuid: Uuid,
    #[builder(default = MaterialType::MeshPhong)]
    #[serde(flatten)]
    pub material_type: MaterialType,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linewidth: Option<f64>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflectivity: Option<f64>,
    #[builder(default = Some(2), setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<u16>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vertexColors")]
    pub vertex_colors: Option<bool>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wireframe: Option<bool>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wireframeLineWidth")]
    pub wireframe_line_width: Option<f64>,
    #[builder(default, setter(skip))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Uuid>,
}

impl Default for Material {
    fn default() -> Self {
        Material::builder()
            .material_type(MaterialType::MeshPhong)
            .build()
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextureType {
    Text {
        #[serde(rename = "type")]
        text_type: String,
        text: String,
        font_size: u32,
        font_face: String,
    },
    Image {
        image: Option<Uuid>,
        repeat: [u32; 2],
        wrap: [u32; 2],
    },
}

impl TextureType {
    pub fn new_text(text: &str, font_size: u32, font_face: &str) -> Self {
        TextureType::Text {
            text_type: "_text".to_string(),
            text: text.to_string(),
            font_size,
            font_face: font_face.to_string(),
        }
    }

    pub fn new_image() -> Self {
        TextureType::Image {
            image: None,
            repeat: [1, 1],
            wrap: [1001, 1001],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Texture {
    pub uuid: Uuid,
    #[serde(flatten)]
    pub texture_type: TextureType,
}

impl Texture {
    pub fn new(texture_type: TextureType) -> Self {
        Texture {
            uuid: Uuid::new_v4(),
            texture_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    // #[builder(default = Uuid::new_v4(), setter(skip))]
    pub uuid: Uuid,
    pub url: String,
}

impl Image {
    pub fn new(url: &str) -> Self {
        let mut buf = String::new();
        match crate::utils::file_extension(url) {
            Ok("png") => {
                buf.push_str("data:image/png;base64,");
                general_purpose::STANDARD.encode_string(
                    std::fs::read(url)
                        .unwrap_or_else(|err| panic!("Unable to load file '{}': {}", url, err)),
                    &mut buf,
                );
            }
            _ => panic!("Unsupported image type"),
        }
        Image {
            uuid: Uuid::new_v4(),
            url: buf,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ObjectType {
    Mesh,
    Points,
    LineSegments,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    pub uuid: Uuid,
    // Both will be set by the build function of LumpedObject
    pub material: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Uuid>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Box<Object>>,
    // TODO: Change to Isometry3<f64> and handle to homogeneous matrix in the serializer
    pub matrix: Matrix4<f64>,
    #[serde(flatten)]
    pub object_type: ObjectType,
}

impl Default for Object {
    fn default() -> Self {
        Self::new(Isometry3::identity(), ObjectType::Mesh)
    }
}

impl Object {
    pub fn new(origin: Isometry3<f64>, object_type: ObjectType) -> Self {
        Object {
            uuid: Uuid::new_v4(),
            material: None,
            geometry: None,
            children: Vec::new(),
            matrix: origin.to_homogeneous(),
            object_type,
        }
    }
}

fn to_one_element_array<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut seq = serializer.serialize_seq(Some(1))?;
    seq.serialize_element(value)?;
    seq.end()
}

// textures, images, materials should be a Vec<_>,
// but I don't see a use case for it yet, so to simplify the code it's just an element (Drake's meshcat interface does the same)
// https://github.com/mrdoob/three.js/wiki/JSON-Object-Scene-format-4
#[derive(Clone, Debug, TypedBuilder, Serialize)]
#[builder(build_method(vis="", name=__build))]
pub struct LumpedObject {
    #[builder(default)]
    pub metadata: Metadata,
    #[builder(default, setter(strip_option))]
    #[serde(
        rename = "textures",
        serialize_with = "to_one_element_array",
        skip_serializing_if = "Option::is_none"
    )]
    pub texture: Option<Texture>,
    #[builder(default, setter(strip_option))]
    #[serde(
        rename = "images",
        serialize_with = "to_one_element_array",
        skip_serializing_if = "Option::is_none"
    )]
    pub image: Option<Image>,
    #[builder(default)]
    pub geometries: Vec<Geometry>,
    #[builder(default)]
    #[serde(rename = "materials", serialize_with = "to_one_element_array")]
    pub material: Material,
    #[builder(default)]
    pub object: Object,
}

// https://github.com/idanarye/rust-typed-builder/blob/master/examples/complicate_build.rs
#[allow(non_camel_case_types)]
impl<
        __metadata: typed_builder::Optional<Metadata>,
        __texture: typed_builder::Optional<Option<Texture>>,
        __image: typed_builder::Optional<Option<Image>>,
        __material: typed_builder::Optional<Material>,
        __object: typed_builder::Optional<Object>,
    >
    LumpedObjectBuilder<(
        __metadata,
        __texture,
        __image,
        (Vec<Geometry>,),
        __material,
        __object,
    )>
{
    #[allow(clippy::default_trait_access)]
    pub fn build(self) -> LumpedObject {
        let mut lumped_object = self.__build();
        // Setting the uuid for an image texture
        if let (Some(image), Some(texture)) = (&lumped_object.image, &mut lumped_object.texture) {
            if let TextureType::Image {
                image: image_uuid, ..
            } = &mut texture.texture_type
            {
                *image_uuid = Some(image.uuid);
            }
        }
        // Setting the uuid for the material
        if let Some(texture) = &lumped_object.texture {
            lumped_object.material.map = Some(texture.uuid);
        }
        // Setting the uuid for the object
        lumped_object.object.material = Some(lumped_object.material.uuid);
        // Meshcat cylinders have their long axis in y.
        lumped_object.object.children = lumped_object
            .geometries
            .iter()
            .map(|geometry| {
                let mut object_pose = geometry.origin;
                if let GeometryType::Cylinder { .. } = &geometry.geometry {
                    object_pose *= Isometry3::from_parts(
                        Translation3::new(0.0, 0.0, 0.0),
                        UnitQuaternion::from_euler_angles(std::f64::consts::FRAC_PI_2, 0.0, 0.0),
                    );
                }
                Box::new(Object {
                    uuid: Uuid::new_v4(),
                    material: Some(lumped_object.material.uuid),
                    geometry: Some(geometry.uuid),
                    children: Vec::new(),
                    matrix: object_pose.to_homogeneous(),
                    object_type: lumped_object.object.object_type.clone(),
                })
            })
            .collect();
        LumpedObject {
            metadata: lumped_object.metadata,
            texture: lumped_object.texture,
            image: lumped_object.image,
            geometries: lumped_object.geometries,
            material: lumped_object.material,
            object: lumped_object.object,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTransformData {
    matrix: Matrix4<f64>,
    path: String,
    #[serde(rename = "type")]
    request_type: String,
}

impl SetTransformData {
    pub fn new(matrix: Isometry3<f64>, path: &str) -> Self {
        SetTransformData {
            matrix: matrix.to_homogeneous(),
            path: path.to_string(),
            request_type: "set_transform".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SetObjectData {
    pub object: LumpedObject,
    pub path: String,
    #[serde(rename = "type")]
    pub request_type: String,
}

// TODO: LumpedCameraData and SetCameraData
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteData {
    pub path: String,
    #[serde(rename = "type")]
    pub request_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyType {
    Visible(bool),
    Position(Vector3<f64>),
    Quaternion(Vector4<f64>),
    Scale(Vector3<f64>),
    Color(Vector4<f64>),
    Opacity(f64),
    ModulatedOpacity(f64),
    TopColor(Vector3<f64>),
    BottomColor(Vector3<f64>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPropertyData {
    path: String,
    #[serde(rename = "type")]
    request_type: String,
    property: String,
    value: PropertyType,
}

impl SetPropertyData {
    pub fn new(path: &str, property: PropertyType) -> Self {
        SetPropertyData {
            path: path.to_string(),
            request_type: "set_property".to_string(),
            property: match property {
                PropertyType::Visible(_) => String::from("visible"),
                PropertyType::Position(_) => String::from("position"),
                PropertyType::Quaternion(_) => String::from("quaternion"),
                PropertyType::Scale(_) => String::from("scale"),
                PropertyType::Color(_) => String::from("color"),
                PropertyType::Opacity(_) => String::from("opacity"),
                PropertyType::ModulatedOpacity(_) => String::from("modulated_opacity"),
                PropertyType::TopColor(_) => String::from("top_color"),
                PropertyType::BottomColor(_) => String::from("bottom_color"),
            },
            value: property,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Geometry {
    pub uuid: Uuid,
    #[serde(flatten)]
    pub geometry: GeometryType,
    // This is used for multi-geometry objects, when creating the children of the object (Type
    // Object)
    #[serde(skip)]
    pub origin: Isometry3<f64>,
}

impl Geometry {
    pub fn new(geometry: GeometryType) -> Self {
        Self::new_with_origin(geometry, Isometry3::identity())
    }

    pub fn new_with_origin(geometry: GeometryType, origin: Isometry3<f64>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            geometry,
            origin,
        }
    }
}

impl From<&urdf_rs::Visual> for Geometry {
    fn from(visual: &urdf_rs::Visual) -> Self {
        Geometry::new_with_origin(
            GeometryType::from(&visual.geometry),
            Isometry3::from_parts(
                Translation3::new(
                    visual.origin.xyz[0],
                    visual.origin.xyz[1],
                    visual.origin.xyz[2],
                ),
                UnitQuaternion::from_euler_angles(
                    visual.origin.rpy[0],
                    visual.origin.rpy[1],
                    visual.origin.rpy[2],
                ),
            ),
        )
    }
}

impl From<&urdf_rs::Collision> for Geometry {
    fn from(collision: &urdf_rs::Collision) -> Self {
        Geometry::new_with_origin(
            GeometryType::from(&collision.geometry),
            Isometry3::from_parts(
                Translation3::new(
                    collision.origin.xyz[0],
                    collision.origin.xyz[1],
                    collision.origin.xyz[2],
                ),
                UnitQuaternion::from_euler_angles(
                    collision.origin.rpy[0],
                    collision.origin.rpy[1],
                    collision.origin.rpy[2],
                ),
            ),
        )
    }
}

impl From<&urdf_rs::Geometry> for GeometryType {
    fn from(geometry: &urdf_rs::Geometry) -> Self {
        match geometry {
            urdf_rs::Geometry::Box { size } => GeometryType::Box {
                width: size[0],
                height: size[1],
                depth: size[2],
            },
            urdf_rs::Geometry::Cylinder { radius, length } => GeometryType::Cylinder {
                radius_top: *radius,
                radius_bottom: *radius,
                height: *length,
                radial_segments: 32,
                height_segments: 1,
                theta_start: 0.0,
                theta_length: 2.0 * std::f64::consts::PI,
            },
            urdf_rs::Geometry::Capsule { .. } => {
                panic!("Capsule geometry is not supported by Meshcat.")
            }
            urdf_rs::Geometry::Sphere { radius } => GeometryType::Sphere {
                radius: *radius,
                width_segments: 32,
                height_segments: 16,
            },
            urdf_rs::Geometry::Mesh { filename, .. } => {
                crate::utils::load_mesh(filename).expect("Failed to load mesh")
            }
        }
    }
}

pub struct Meshcat {
    socket: zmq::Socket,
}

impl Meshcat {
    pub fn new(endpoint: &str) -> Self {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REQ).unwrap();
        socket.connect(endpoint).unwrap_or_else(|err| {
            panic!(
                "Failed to connect to Meshcat server '{}': {}.",
                endpoint, err
            )
        });
        Self { socket }
    }

    pub fn set_object(&self, path: &str, object: LumpedObject) -> Result<(), Box<dyn Error>> {
        let data = SetObjectData {
            object,
            path: path.to_string(),
            request_type: "set_object".to_string(),
        };
        let buf = rmp_serde::encode::to_vec_named(&data)?;
        self.socket.send_multipart(
            [data.request_type.as_bytes(), data.path.as_bytes(), &buf],
            0,
        )?;
        let message = self.socket.recv_string(0)?;
        info!("Received reply {} {}", 0, message.unwrap());
        Ok(())
    }

    pub fn set_transform(&self, path: &str, matrix: Isometry3<f64>) -> Result<(), Box<dyn Error>> {
        let data = SetTransformData::new(matrix, path);
        let buf = rmp_serde::encode::to_vec_named(&data)?;
        self.socket.send_multipart(
            [data.request_type.as_bytes(), data.path.as_bytes(), &buf],
            0,
        )?;
        let message = self.socket.recv_string(0)?;
        info!("Received reply {} {}", 0, message.unwrap());
        Ok(())
    }

    pub fn set_property(&self, path: &str, property: PropertyType) -> Result<(), Box<dyn Error>> {
        let data = SetPropertyData::new(path, property);
        let buf = rmp_serde::encode::to_vec_named(&data)?;
        self.socket.send_multipart(
            [data.request_type.as_bytes(), data.path.as_bytes(), &buf],
            0,
        )?;
        let message = self.socket.recv_string(0)?;
        info!("Received reply {} {}", 0, message.unwrap());
        Ok(())
    }

    pub fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let data = DeleteData {
            path: path.to_string(),
            request_type: "delete".to_string(),
        };
        let buf = rmp_serde::encode::to_vec_named(&data)?;
        self.socket.send_multipart(
            [data.request_type.as_bytes(), data.path.as_bytes(), &buf],
            0,
        )?;
        let message = self.socket.recv_string(0)?;
        info!("Received reply {} {}", 0, message.unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lumped_object() {
        let lumped_object = LumpedObject::builder()
            .geometries(vec![Geometry::new(GeometryType::Box {
                width: 1.0,
                height: 1.0,
                depth: 1.0,
            })])
            .build();
        assert_eq!(lumped_object.geometries.len(), 1);
        assert!(lumped_object.texture.is_none());
        assert!(lumped_object.image.is_none());
        // We only use this field for the children (The geometries the object is composed of)
        assert!(lumped_object.object.geometry.is_none());
        assert_eq!(lumped_object.object.children.len(), 1);
        assert!(lumped_object.object.children[0].geometry.is_some());
        assert_eq!(
            lumped_object.object.children[0].geometry.unwrap(),
            lumped_object.geometries[0].uuid
        );
        assert!(lumped_object.material.map.is_none());
    }

    #[test]
    fn test_multiple_geometries() {
        let lumped_object = LumpedObject::builder()
            .geometries(vec![
                Geometry::new(GeometryType::Box {
                    width: 1.0,
                    height: 1.0,
                    depth: 1.0,
                }),
                Geometry::new(GeometryType::Cylinder {
                    radius_top: 0.2,
                    radius_bottom: 0.2,
                    height: 0.5,
                    radial_segments: 20,
                    height_segments: 10,
                    theta_start: 0.0,
                    theta_length: 2.0 * std::f64::consts::PI,
                }),
            ])
            .build();
        assert_eq!(lumped_object.geometries.len(), 2);
        assert!(lumped_object.texture.is_none());
        assert!(lumped_object.image.is_none());
        assert!(lumped_object.object.geometry.is_none());
        assert_eq!(lumped_object.object.children.len(), 2);
        assert!(lumped_object.object.children[0].geometry.is_some());
        assert_eq!(
            lumped_object.object.children[0].geometry.unwrap(),
            lumped_object.geometries[0].uuid
        );
        assert!(lumped_object.object.children[1].geometry.is_some());
        assert_eq!(
            lumped_object.object.children[1].geometry.unwrap(),
            lumped_object.geometries[1].uuid
        );
        assert!(lumped_object.material.map.is_none());
    }

    #[test]
    fn test_object_with_texture() {
        let lumped_object = LumpedObject::builder()
            .geometries(vec![Geometry::new(GeometryType::Box {
                width: 1.0,
                height: 1.0,
                depth: 1.0,
            })])
            .texture(Texture::new(TextureType::new_text(
                "Hello, meshcat!",
                12,
                "sans-serif",
            )))
            .build();
        assert_eq!(lumped_object.geometries.len(), 1);
        assert!(lumped_object.texture.is_some());
        assert!(lumped_object.image.is_none());
        assert!(lumped_object.object.geometry.is_none());
        assert_eq!(lumped_object.object.children.len(), 1);
        assert!(lumped_object.object.children[0].geometry.is_some());
        assert_eq!(
            lumped_object.object.children[0].geometry.unwrap(),
            lumped_object.geometries[0].uuid
        );
        assert!(lumped_object.material.map.is_some());
        assert_eq!(
            lumped_object.material.map.unwrap(),
            lumped_object.texture.unwrap().uuid
        );
    }

    #[test]
    fn test_object_with_texture_image() {
        let lumped_object = LumpedObject::builder()
            .geometries(vec![Geometry::new(GeometryType::Box {
                width: 1.0,
                height: 1.0,
                depth: 1.0,
            })])
            .image(Image::new("examples/data/HeadTextureMultisense.png"))
            .texture(Texture::new(TextureType::new_image()))
            .build();
        assert_eq!(lumped_object.geometries.len(), 1);
        assert!(lumped_object.texture.is_some());
        assert!(lumped_object.image.is_some());
        assert!(lumped_object.material.map.is_some());
        let texture = lumped_object.texture.unwrap();
        assert_eq!(lumped_object.material.map.unwrap(), texture.uuid);
        assert_eq!(
            texture.texture_type,
            TextureType::Image {
                image: Some(lumped_object.image.unwrap().uuid),
                repeat: [1, 1],
                wrap: [1001, 1001],
            }
        );
    }
}
