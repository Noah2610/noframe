extern crate noframe;

use noframe_derive::Mask;

#[test]
fn derive_mask() {
    use noframe::geo::prelude::*;

    #[derive(Mask)]
    struct MyMask {
        point:  Point,
        size:   Size,
        origin: Origin,
    }

    let point  = Point::new(10.0, 20.0);
    let size   = Size::new(20.0, 30.0);
    let origin = Origin::Center;
    let mut my_mask = MyMask {
        point:  point.clone(),
        size:   size.clone(),
        origin: origin.clone(),
    };

    assert_eq!(point,  *my_mask.point());
    assert_eq!(size,   *my_mask.size());
    assert_eq!(origin, *my_mask.origin());

    let add = Vector::new(7.0, 12.0);
    *my_mask.point_mut() += add;

    assert_eq!(point + add, *my_mask.point());
}
