use std::collections::HashSet;

use crate::{
    function::tools::{
        ecef_to_point::ecef_to_point, point_to_ecef::point_to_ecef, point_to_id::point_to_id, ECEF,
    },
    id::{coordinates::Point, SpaceTimeId},
};

pub fn triangle(z: u8, a: Point, b: Point, c: Point) -> HashSet<SpaceTimeId> {
    let mut voxels_set = HashSet::new();

    // Point → ECEF
    let ea = point_to_ecef(a);
    let eb = point_to_ecef(b);
    let ec = point_to_ecef(c);

    // --- steps を Python と同じ計算式で決定 ---
    let min_lat_rad = a
        .latitude
        .abs()
        .min(b.latitude.abs())
        .min(c.latitude.abs())
        .to_radians();

    let r: f64 = 6_378_137.0; // WGS84 赤道半径
    let d = std::f64::consts::PI * r * min_lat_rad.cos() * 2f64.powf(-(2.0 + z as f64)); // voxel 東西距離の 1/8
    let l1 = ((ec.x - eb.x).powi(2) + (ec.y - eb.y).powi(2) + (ec.z - eb.z).powi(2)).sqrt();
    let l2 = ((ea.x - ec.x).powi(2) + (ea.y - ec.y).powi(2) + (ea.z - ec.z).powi(2)).sqrt();
    let l3 = ((ea.x - eb.x).powi(2) + (ea.y - eb.y).powi(2) + (ea.z - eb.z).powi(2)).sqrt();
    let steps = (l1.max(l2).max(l3) / d).ceil() as usize;

    // --- 三角形内部を走査 ---
    for i in 0..=steps {
        if i == 0 {
            let p = ecef_to_point(ea);
            let voxel = point_to_id(z, p);
            voxels_set.insert(voxel);
        } else {
            let t = i as f64 / steps as f64;

            // 辺 a-b, a-c を補間
            let line1 = ECEF {
                x: ea.x * (1.0 - t) + eb.x * t,
                y: ea.y * (1.0 - t) + eb.y * t,
                z: ea.z * (1.0 - t) + eb.z * t,
            };
            let line2 = ECEF {
                x: ea.x * (1.0 - t) + ec.x * t,
                y: ea.y * (1.0 - t) + ec.y * t,
                z: ea.z * (1.0 - t) + ec.z * t,
            };

            for j in 0..=i {
                let s = j as f64 / i as f64;

                // line1 と line2 を補間して内部点を得る
                let e = ECEF {
                    x: line1.x * (1.0 - s) + line2.x * s,
                    y: line1.y * (1.0 - s) + line2.y * s,
                    z: line1.z * (1.0 - s) + line2.z * s,
                };

                // ECEF → Point → Voxel
                let p = ecef_to_point(e);
                let voxel = point_to_id(z, p);
                voxels_set.insert(voxel);
            }
        }
    }

    voxels_set
}
