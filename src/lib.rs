//! # geo-svg
//!
//! This crate is a lib to generate SVG strings from [geo](https://docs.rs/geo/latest/geo/).
//!
//! [![crate.io](https://img.shields.io/crates/v/geo-svg.svg)](https://crates.io/crates/geo-svg)
//! [![docs.rs](https://docs.rs/geo-svg/badge.svg)](https://docs.rs/geo-svg)
//!
//! Below is an example of a geometry collection rendered to SVG.
//!
//! ![example](https://raw.githubusercontent.com/lelongg/geo-svg/master/example.png)
//!
//! # Features
//!
//! - [GeometryCollection](https://docs.rs/geo/latest/geo/geometry/struct.GeometryCollection.html) and all variants of [Geometry](https://docs.rs/geo/latest/geo/geometry/enum.Geometry.html) are supported
//! - the viewport size is automatically computed to contain all shapes
//! - style and formatting options are available
//!
//! # Example
//!
//! The following will show how to convert a line to a SVG string.  
//! The [`add_shape`] method is provided by the [`SVGDocument`]/[`SVGDocumentBuilder`]/[`SVGDocumentPart`] struct which accepts all [geo](https://docs.rs/geo/latest/geo) types.
//!
//! ```
//! use geo_svg::{Color, SVGDocument, Stylable};
//! use geo::{Coord, Line, Point};
//!
//! let point = Point::new(10.0, 28.1);
//! let line = Line::new(
//!     Coord { x: 114.19, y: 22.26, },
//!     Coord { x: 15.93, y: -15.76, },
//! );
//!
//! let svg = SVGDocument::style_builder()
//!     .with_radius(2.0)
//!     .finish_style()
//!     .add_shape(&point)
//!     .finish_shapes()
//!     .and(
//!         SVGDocument::style_builder()
//!             .with_stroke_width(2.5)
//!             .finish_style()
//!             .add_shape(&line)
//!             .finish_shapes(),
//!     )
//!     .with_fill_color(Color::Named("red"))
//!     .with_stroke_color(Color::Rgb(200, 0, 100))
//!     .with_fill_opacity(0.7);
//!
//! println!("{}", svg);
//!
//! let expect = r#"<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="7 -18.26 109.69 49.36"  fill="red" fill-opacity="0.7" stroke="rgb(200,0,100)"><circle cx="10.0" cy="28.1" r="2"/><path d="M 114.19 22.26 L 15.93 -15.76" stroke-width="2.5"/></svg>"#;
//!
//! assert_eq!(expect, svg.render().as_str());
//! ```

mod properties;
mod svg_document;
mod svg_trait;
mod text;

pub use properties::*;
pub use svg_document::*;
pub use svg_trait::*;
pub use text::*;
