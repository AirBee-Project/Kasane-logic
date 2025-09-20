use std::collections::HashSet;

use crate::{
    function::tools::{
        ecef_to_point::ecef_to_point, point_to_ecef::point_to_ecef, point_to_id::point_to_id, ECEF,
    },
    id::{coordinates::Point, SpaceTimeId},
};

/// a と b の間の voxel 線分を返す
pub fn line(z: u8, a: Point, b: Point) -> HashSet<SpaceTimeId> {
    let mut result = HashSet::new();

    // Point → ECEF
    let ea = point_to_ecef(a);
    let eb = point_to_ecef(b);

    // --- steps を Python と同じ計算で求める ---
    let min_lat_rad = a.latitude.abs().min(b.latitude.abs()).to_radians();
    let r: f64 = 6_378_137.0; // WGS84 赤道半径
    let d = std::f64::consts::PI * r * min_lat_rad.cos() * 2f64.powf(-(3.0 + z as f64)); // voxel 東西距離の 1/16
    let distance = ((ea.x - eb.x).powi(2) + (ea.y - eb.y).powi(2) + (ea.z - eb.z).powi(2)).sqrt();
    let steps = (distance / d).ceil() as usize;

    // --- 線分を分割して補間 ---
    for i in 0..=steps {
        let t = i as f64 / steps as f64;

        // ECEF補間
        let e = ECEF {
            x: ea.x * (1.0 - t) + eb.x * t,
            y: ea.y * (1.0 - t) + eb.y * t,
            z: ea.z * (1.0 - t) + eb.z * t,
        };

        // ECEF → Point
        let p = ecef_to_point(e);

        // Point → Voxel
        let voxel = point_to_id(z, p);

        result.insert(voxel);
    }

    result
}
