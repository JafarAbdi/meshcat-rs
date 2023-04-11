use std::error::Error;
use std::time::Duration;

use meshcat::types::*;
use meshcat::utils;
use nalgebra::Isometry3;
use nalgebra::Matrix3xX;

pub fn point_cloud() -> LumpedObject {
    let points = Matrix3xX::<f64>::new_random(100000);
    let colors = points.clone();
    LumpedObject::builder()
        .geometry(Geometry::new(GeometryType::Buffer {
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
        }))
        .material(
            Material::builder()
                .vertex_colors(true)
                .material_type(MaterialType::Points { size: 0.001 })
                .build(),
        )
        .object(Object::new(
            Isometry3::from_parts(
                nalgebra::Translation3::new(2.0, -2.0, 0.0),
                nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            ),
            ObjectType::Points,
        ))
        .build()
}

fn valkyrie_head() -> LumpedObject {
    LumpedObject::builder()
        .image(Image::new("examples/data/HeadTextureMultisense.png"))
        .texture(Texture::new(TextureType::new_image()))
        .geometry(Geometry::new(
            utils::load_mesh("examples/data/head_multisense.obj").expect("Failed to load mesh"),
        ))
        .object(Object::new(
            Isometry3::from_parts(
                nalgebra::Translation3::new(0.0, 0.0, 0.0),
                nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            ),
            ObjectType::Mesh,
        ))
        .build()
}

fn main() -> Result<(), Box<dyn Error>> {
    let meshcat = Meshcat::new("tcp://127.0.0.1:6000");

    meshcat.set_object("/head_1", valkyrie_head())?;
    meshcat.set_object("/head_1/head", valkyrie_head())?;
    meshcat.set_transform(
        "/head_1/head",
        Isometry3::from_parts(
            nalgebra::Translation3::new(1.0, 1.0, 0.0),
            nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        ),
    )?;
    meshcat.set_object("/point_cloud", point_cloud())?;
    meshcat.set_object(
        "/convex_dae",
        LumpedObject::builder()
            .geometry(Geometry::new(utils::load_mesh(
                "examples/data/mesh_0_convex_piece_0.dae",
            )?))
            .build(),
    )?;
    // TODO: Investigate why this doesn't work.
    // meshcat.set_object(
    //     "/convex_dae",
    //     LumpedObject::builder()
    //         .geometry(Geometry::new(utils::load_mesh(
    //             "examples/data/mesh_0_convex_piece_0.dae",
    //         )?))
    //         .build(),
    // )?;
    meshcat.set_object(
        "/convex_stl",
        LumpedObject::builder()
            .geometry(Geometry::new(utils::load_mesh(
                "examples/data/mesh_0_convex_piece_0.obj",
            )?))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(1.0, -1.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/text",
        utils::scene_text(TextureType::new_text("Hello, meshcat!", 100, "sans-serif")),
    )?;
    meshcat.set_object(
        "/torus",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Torus {
                radius: 0.5,
                tube: 0.2,
                radial_segments: 12,
                tubular_segments: 48,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(0.0, 2.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .material(Material::builder().color(0x00ff00).build())
            .build(),
    )?;
    meshcat.set_object(
        "/tetrahedron",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Tetrahedron {
                radius: 0.5,
                detail: 0,
            }))
            .material(Material::builder().color(0xff0000).build())
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(1.0, 0.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/ring",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Ring {
                inner_radius: 0.5,
                outer_radius: 1.0,
                theta_segments: 32,
                phi_segments: 1,
                theta_start: 0.0,
                theta_length: 2.0 * std::f64::consts::PI,
            }))
            .material(Material::builder().color(0x0000ff).build())
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(2.0, 2.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/plane",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Plane {
                width: 0.25,
                height: 0.25,
                width_segments: 1,
                height_segments: 1,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(2.0, 2.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/octahedron",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Octahedron {
                radius: 0.5,
                detail: 0,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(-1.0, -1.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/icosahedron",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Icosahedron {
                radius: 0.5,
                detail: 0,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(-2.0, -2.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/dodecahedron",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Dodecahedron {
                radius: 0.5,
                detail: 0,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(-3.0, -3.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/cylinder",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Cylinder {
                radius_top: 0.5,
                radius_bottom: 0.5,
                height: 1.0,
                radial_segments: 32,
                height_segments: 1,
                theta_start: 0.0,
                theta_length: 2.0 * std::f64::consts::PI,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(0.0, -1.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .material(Material::builder().color(0x00ffff).build())
            .build(),
    )?;
    meshcat.set_object(
        "/circle",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Circle {
                radius: 0.5,
                segments: 32,
                theta_start: 0.0,
                theta_length: 2.0 * std::f64::consts::PI,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(0.0, -2.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;
    meshcat.set_object(
        "/cone",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Cone {
                radius: 0.5,
                height: 1.0,
                radial_segments: 32,
                height_segments: 1,
                theta_start: 0.0,
                theta_length: 2.0 * std::f64::consts::PI,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(0.0, -3.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .material(Material::builder().color(0x00ffff).build())
            .build(),
    )?;
    meshcat.set_object(
        "/sphere",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Sphere {
                radius: 0.5,
                width_segments: 12,
                height_segments: 12,
            }))
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(-2.0, 2.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .material(Material::builder().color(0x0000ff).build())
            .build(),
    )?;
    meshcat.set_object(
        "/box",
        LumpedObject::builder()
            .geometry(Geometry::new(GeometryType::Box {
                width: 0.5,
                height: 0.5,
                depth: 0.5,
            }))
            .material(Material::builder().color(0xff00ff).build())
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(0.0, 1.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .build(),
    )?;

    let delta_angle = 0.1;
    let mut angle = 0.0;
    for _ in 0..100 {
        angle += delta_angle;
        meshcat.set_transform(
            "/head_1",
            Isometry3::from_parts(
                nalgebra::Translation3::new(0.0, 0.0, 0.0),
                nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, angle),
            ),
        )?;
        meshcat.set_transform(
            "/head_1/head",
            Isometry3::from_parts(
                nalgebra::Translation3::new(1.0, 1.0, 0.0),
                nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, angle),
            ),
        )?;
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
