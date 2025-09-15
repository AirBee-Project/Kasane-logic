use kasane_logic::id::{DimensionRange, SpaceTimeId};

fn main() {
    //時空間IDを作成してくれる
    let id = SpaceTimeId::new(
        5,
        DimensionRange::Single(1),
        DimensionRange::LimitRange(3, 7),
        DimensionRange::LimitRange(5, 8),
        0,
        DimensionRange::Any,
    )
    .unwrap();

    println!("元のID : {}", id);
    println!("変換ID : {}", id.top(3).unwrap())
}
