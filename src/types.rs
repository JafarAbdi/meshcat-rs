use std::error::Error;

use base64::{engine::general_purpose, Engine as _};
use log::info;
use nalgebra::{Isometry3, Matrix3xX, Matrix4};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    pub uuid: Uuid,
    // Both will be set by the build function of LumpedObject
    pub geometry: Option<Uuid>,
    pub material: Option<Uuid>,
    // TODO: Change to Isometry3<f64> and handle to homogeneous matrix in the serializer
    pub matrix: Matrix4<f64>,
    #[serde(flatten)]
    pub object_type: ObjectType,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            uuid: Uuid::new_v4(),
            geometry: None,
            material: None,
            matrix: Isometry3::identity().to_homogeneous(),
            object_type: ObjectType::Mesh,
        }
    }
}

impl Object {
    pub fn new(origin: Isometry3<f64>, object_type: ObjectType) -> Self {
        Object {
            uuid: Uuid::new_v4(),
            geometry: None,
            material: None,
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

// textures, images, geometries, materials should be a Vec<_>,
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
    #[serde(rename = "geometries", serialize_with = "to_one_element_array")]
    pub geometry: Geometry,
    #[builder(default)]
    #[serde(rename = "materials", serialize_with = "to_one_element_array")]
    pub material: Material,
    #[builder(default)]
    pub object: Object,
}

// https://github.com/idanarye/rust-typed-builder/blob/master/examples/complicate_build.rs
#[allow(non_camel_case_types)]
impl<
        __metadata: LumpedObjectBuilder_Optional<Metadata>,
        __texture: LumpedObjectBuilder_Optional<Option<Texture>>,
        __image: LumpedObjectBuilder_Optional<Option<Image>>,
        __material: LumpedObjectBuilder_Optional<Material>,
        __object: LumpedObjectBuilder_Optional<Object>,
    >
    LumpedObjectBuilder<(
        __metadata,
        __texture,
        __image,
        (Geometry,),
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
        lumped_object.object.geometry = Some(lumped_object.geometry.uuid);
        LumpedObject {
            metadata: lumped_object.metadata,
            texture: lumped_object.texture,
            image: lumped_object.image,
            geometry: lumped_object.geometry,
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
#[derive(Clone, Debug, Serialize)]
pub struct Geometry {
    pub uuid: Uuid,
    #[serde(flatten)]
    pub geometry: GeometryType,
}

impl Geometry {
    pub fn new(geometry: GeometryType) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            geometry,
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
