use crate::{Color, SVGDocument, Stylable};
use geo::{LineString, Point, Polygon};

#[test]
fn two_unified_point_documents_with_fallback_style() {
    let part1 = SVGDocument::style_builder()
        .with_fill_color(Color::Named("red"))
        .with_radius(10.0)
        .with_stroke_color(Color::Named("black"))
        .finish_style()
        .add_shape(&Point::new(0.0, 0.0))
        .finish_shapes();
    let part2 = SVGDocument::style_builder()
        .with_radius(5.0)
        .with_stroke_color(Color::Named("blue"))
        .finish_style()
        .add_shape(&Point::new(50.0, 0.0))
        .finish_shapes();

    let svg = part1
        .and(part2)
        .with_stroke_width(1.0)
        .with_opacity(0.5)
        .with_fill_opacity(0.5)
        .with_fill_color(Color::Named("green"));

    println!("{svg}");
}

#[test]
fn test_polygon() {
    let svg = SVGDocument::style_builder()
        .with_fill_color(Color::Named("black"))
        .with_stroke_color(Color::Named("red"))
        .finish_style()
        .add_shape(&Polygon::new(
            LineString(vec![
                (210.0, 0.0).into(),
                (300.0, 0.0).into(),
                (300.0, 90.0).into(),
                (210.0, 90.0).into(),
            ]),
            vec![LineString(vec![
                (230.0, 20.0).into(),
                (280.0, 20.0).into(),
                (280.0, 70.0).into(),
                (230.0, 70.0).into(),
            ])],
        ))
        .finish_shapes();

    println!("{svg}",);
}
