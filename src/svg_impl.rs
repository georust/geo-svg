use super::{svg, ToSvg, ViewBox};
use geo_types::{
    Coordinate, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon,
};
use svg_fmt::{black, green, path, red, Circle, Fill, LineSegment, Stroke, Style};

const STROKE_WIDTH: f64 = 0.1;

impl ToSvg for GeometryCollection<f64> {
    fn to_svg_str(&self) -> String {
        self.0
            .iter()
            .map(|geometry| geometry.to_svg_str())
            .collect()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.0
            .iter()
            .fold(ViewBox::default(), |view_box, geometry| {
                view_box.add(&geometry.view_box())
            })
    }
}

impl ToSvg for Geometry<f64> {
    fn to_svg_str(&self) -> String {
        match self {
            Geometry::Point(point) => point.to_svg_str(),
            Geometry::Line(line) => line.to_svg_str(),
            Geometry::LineString(line_tring) => line_tring.to_svg_str(),
            Geometry::Polygon(polygon) => polygon.to_svg_str(),
            Geometry::MultiPoint(multi_point) => multi_point.to_svg_str(),
            Geometry::MultiLineString(multi_line_string) => multi_line_string.to_svg_str(),
            Geometry::MultiPolygon(multi_polygon) => multi_polygon.to_svg_str(),
            Geometry::GeometryCollection(geometry_collection) => geometry_collection.to_svg_str(),
        }
    }

    fn view_box(&self) -> svg::ViewBox {
        match self {
            Geometry::Point(point) => point.view_box(),
            Geometry::Line(line) => line.view_box(),
            Geometry::LineString(line_tring) => line_tring.view_box(),
            Geometry::Polygon(polygon) => polygon.view_box(),
            Geometry::MultiPoint(multi_point) => multi_point.view_box(),
            Geometry::MultiLineString(multi_line_string) => multi_line_string.view_box(),
            Geometry::MultiPolygon(multi_polygon) => multi_polygon.view_box(),
            Geometry::GeometryCollection(geometry_collection) => geometry_collection.view_box(),
        }
    }
}

impl ToSvg for MultiPolygon<f64> {
    fn to_svg_str(&self) -> String {
        self.0.iter().map(|polygon| polygon.to_svg_str()).collect()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.0.iter().fold(ViewBox::default(), |view_box, polygon| {
            view_box.add(&polygon.view_box())
        })
    }
}

impl ToSvg for Polygon<f64> {
    fn to_svg_str(&self) -> String {
        let line_string_to_path = |line_string: &LineString<f64>| {
            line_string
                .points_iter()
                .fold(None, |path, point| {
                    let path = match path {
                        Some(path) => path,
                        None => svg_fmt::path().move_to(point.x() as f32, point.y() as f32),
                    };
                    Some(path.line_to(point.x() as f32, point.y() as f32))
                })
                .unwrap_or_else(path)
        };

        let exterior = line_string_to_path(&self.exterior())
            .fill(Fill::Color(green()))
            .stroke(Stroke::Color(green(), STROKE_WIDTH as f32))
            .opacity(0.5)
            .to_string();

        let interiors = self.interiors().iter().map(|interior| {
            line_string_to_path(&interior)
                .fill(Fill::Color(red()))
                .stroke(Stroke::Color(red(), STROKE_WIDTH as f32))
                .opacity(0.5)
                .to_string()
        });

        let points = self
            .exterior()
            .points_iter()
            .skip(1)
            .chain(
                self.interiors()
                    .iter()
                    .flat_map(|interior| interior.points_iter().skip(1)),
            )
            .flat_map(|point| point.to_svg().elements);

        std::iter::once(exterior)
            .chain(interiors)
            .chain(points)
            .collect::<String>()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.exterior()
            .points_iter()
            .chain(
                self.interiors()
                    .iter()
                    .flat_map(|interior| interior.points_iter()),
            )
            .fold(svg::ViewBox::default(), |view_box, point| {
                view_box.add(&point.view_box())
            })
    }
}

impl ToSvg for MultiLineString<f64> {
    fn to_svg_str(&self) -> String {
        self.0
            .iter()
            .map(|line_string| line_string.to_svg_str())
            .collect()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.0
            .iter()
            .fold(ViewBox::default(), |view_box, line_string| {
                view_box.add(&line_string.view_box())
            })
    }
}

impl ToSvg for LineString<f64> {
    fn to_svg_str(&self) -> String {
        self.points_iter()
            .fold(None, |path, point| {
                let path = match path {
                    Some(path) => path,
                    None => svg_fmt::path().move_to(point.x() as f32, point.y() as f32),
                };
                Some(path.line_to(point.x() as f32, point.y() as f32))
            })
            .unwrap_or_else(path)
            .fill(Fill::None)
            .stroke(Stroke::Color(black(), STROKE_WIDTH as f32))
            .opacity(0.5)
            .to_string()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.points_iter()
            .fold(svg::ViewBox::default(), |view_box, point| {
                view_box.add(&point.view_box())
            })
    }
}

impl ToSvg for Line<f64> {
    fn to_svg_str(&self) -> String {
        LineSegment {
            x1: self.start.x as f32,
            y1: self.start.y as f32,
            x2: self.end.x as f32,
            y2: self.end.y as f32,
            color: black(),
            width: STROKE_WIDTH as f32,
        }
        .to_string()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.start.view_box().add(&self.end.view_box())
    }
}

impl ToSvg for MultiPoint<f64> {
    fn to_svg_str(&self) -> String {
        self.0.iter().map(|point| point.to_svg_str()).collect()
    }

    fn view_box(&self) -> svg::ViewBox {
        self.0.iter().fold(ViewBox::default(), |view_box, point| {
            view_box.add(&point.view_box())
        })
    }
}

impl ToSvg for Point<f64> {
    fn to_svg_str(&self) -> String {
        Circle {
            x: self.x() as f32,
            y: self.y() as f32,
            radius: STROKE_WIDTH as f32,
            style: Style::default(),
        }
        .fill(Fill::Color(black()))
        .to_string()
    }

    fn view_box(&self) -> svg::ViewBox {
        svg::ViewBox::new(
            (self.x() - STROKE_WIDTH) as f32,
            (self.y() - STROKE_WIDTH) as f32,
            (self.x() + STROKE_WIDTH) as f32,
            (self.y() + STROKE_WIDTH) as f32,
        )
    }
}

impl ToSvg for Coordinate<f64> {
    fn to_svg_str(&self) -> String {
        Point::from(*self).to_svg_str()
    }

    fn view_box(&self) -> svg::ViewBox {
        Point::from(*self).view_box()
    }
}
