use crate::{
    function::tools::point_to_id::point_to_id,
    id::{coordinates::Point, SpaceTimeId},
};

pub fn point(z: u8, a: Point) -> SpaceTimeId {
    point_to_id(z, a)
}
