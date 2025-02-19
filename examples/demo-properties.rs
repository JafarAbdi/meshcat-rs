use std::error::Error;
use std::time::Duration;

use meshcat::types::*;
use nalgebra::Isometry3;
use nalgebra::UnitQuaternion;
use nalgebra::Vector3;
use nalgebra::Vector4;

fn main() -> Result<(), Box<dyn Error>> {
    let meshcat = Meshcat::new("tcp://127.0.0.1:6000");

    meshcat.set_object(
        "/torus",
        LumpedObject::builder()
            .geometries(vec![Geometry::new(GeometryType::Torus {
                radius: 0.5,
                tube: 0.2,
                radial_segments: 12,
                tubular_segments: 48,
            })])
            .object(Object::new(
                Isometry3::from_parts(
                    nalgebra::Translation3::new(0.0, 0.0, 0.0),
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
                ),
                ObjectType::Mesh,
            ))
            .material(Material::builder().color(0x00ff00).build())
            .build(),
    )?;

    meshcat.set_property("/Axes", PropertyType::Visible(false))?;
    meshcat.set_property(
        "/Background",
        PropertyType::TopColor(Vector3::new(0.5, 0.8, 0.5)),
    )?;
    meshcat.set_property(
        "/Background",
        PropertyType::BottomColor(Vector3::new(0.6, 0.0, 0.5)),
    )?;

    let delta_angle = 0.1;
    let mut angle = 0.0;
    for _ in 0..100 {
        angle += delta_angle;
        meshcat.set_property(
            "/torus",
            PropertyType::Scale(
                Vector3::new(1.0, 1.0, 1.0) * (1.0 + f64::sin(angle) * f64::sin(angle)),
            ),
        )?;
        meshcat.set_property(
            "/torus",
            PropertyType::Position(Vector3::new(0.0, 0.0, f64::sin(angle))),
        )?;
        meshcat.set_property(
            "/torus",
            PropertyType::Quaternion(
                *UnitQuaternion::from_euler_angles(0.0, angle, 0.0).as_vector(),
            ),
        )?;
        meshcat.set_property(
            "/torus",
            PropertyType::Quaternion(
                *UnitQuaternion::from_euler_angles(0.0, angle, 0.0).as_vector(),
            ),
        )?;
        meshcat.set_property(
            "/torus",
            PropertyType::Color(Vector4::new(0.5, 0.8, 0.5, 0.5)),
        )?;

        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
