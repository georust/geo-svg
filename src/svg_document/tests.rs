use super::*;
use crate::Stylable;
use geo::*;

#[test]
fn new_document_api_works() {
    let single_point = Point::from(Coord::<f32>::zero());
    let triangles = [[Coord::<f64>::zero(); 3]; 3].map(Triangle::from);
    let svg = SVGDocument::style_builder()
        .with_radius(1.0)
        .finish_style()
        .add_shape(&single_point)
        .add_shapes(&triangles)
        .finish_shapes();
    println!("{svg}");
}
