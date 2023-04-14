use std::collections::HashMap;

use itertools::Itertools;
use meshcat::types::*;
use nalgebra::{Isometry3, Translation3, UnitQuaternion};

fn load_urdf(meshcat: &Meshcat, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Preprocess the URDF to get the full names (meshcat's paths) of the joints and links
    let mut names = HashMap::new();
    let urdf_robot = urdf_rs::read_file(path).unwrap();
    for joint in &urdf_robot.joints {
        let joint_name = names
            .entry(&joint.parent.link)
            .or_insert("/".to_owned() + &joint.parent.link);
        let joint_fullname = joint_name.clone() + "/" + &joint.name;
        let child_fullname = joint_fullname.clone() + "/" + &joint.child.link;
        names.insert(&joint.name, joint_fullname);
        names.insert(&joint.child.link, child_fullname);
    }

    // Make sure to delete the old URDF
    for name in names.values() {
        meshcat.delete(name)?;
    }

    // Publish the URDF to meshcat
    for link in &urdf_robot.links {
        if !link.visual.is_empty() {
            meshcat.set_object(
                &names[&link.name],
                LumpedObject::builder()
                    .geometries(link.visual.iter().map(Geometry::from).collect_vec())
                    .build(),
            )?;
        }
    }
    for joint in &urdf_robot.joints {
        meshcat.set_transform(
            &names[&joint.name],
            Isometry3::from_parts(
                Translation3::new(
                    joint.origin.xyz[0],
                    joint.origin.xyz[1],
                    joint.origin.xyz[2],
                ),
                UnitQuaternion::from_euler_angles(
                    joint.origin.rpy[0],
                    joint.origin.rpy[1],
                    joint.origin.rpy[2],
                ),
            ),
        )?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let meshcat = Meshcat::new("tcp://127.0.0.1:6000");
    load_urdf(&meshcat, "examples/data/sample.urdf")?;
    load_urdf(&meshcat, "examples/data/panda_description/panda.urdf")?;
    meshcat.set_transform("/panda_link0", Isometry3::translation(1.0, 0.0, 0.0))?;
    Ok(())
}
