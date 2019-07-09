# geo-svg

This crate is a lib to generate SVG strings from [geo-types](https://docs.rs/geo-types/0.4.3/geo_types/).

[![crate.io](https://img.shields.io/crates/v/geo-svg.svg)](https://crates.io/crates/geo-svg)
[![docs.rs](https://docs.rs/geo-svg/badge.svg)](https://docs.rs/geo-svg)

Below is an example of a geometry collection rendered to SVG.

![example](https://raw.githubusercontent.com/lelongg/geo-svg/master/example.png)

## Features

- [GeometryCollection](https://docs.rs/geo-types/0.4.3/geo_types/struct.GeometryCollection.html) and all variants of [Geometry](https://docs.rs/geo-types/0.4.3/geo_types/enum.Geometry.html) are supported
- the viewport size is automatically computed to contain all shapes
- style and formatting options are available

## Example

The following will show how to convert a line to a SVG string.
The [`to_svg`] method is provided by the [`ToSvg`] trait which is implemented for all [geo-types](https://docs.rs/geo-types/0.4.3/geo_types/).

```rust
use geo_types::{Coordinate, Line, Point};
use geo_svg::{Color, ToSvg};
let point = Point::new(10.0, 28.1);
let line = Line::new(
    Coordinate { x: 114.19, y: 22.26 },
    Coordinate { x: 15.93, y: -15.76 },
);

let svg = point
    .to_svg()
    .with_radius(2.0)
    .and(line.to_svg().with_stroke_width(2.5))
    .with_fill_color(Color::Named("red"))
    .with_stroke_color(Color::Rgb(200, 0, 100))
    .with_fill_opacity(0.7);

println!("{}", svg);
```

### Result

```xml
<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="7 -18.26 109.69 49.36"><circle cx="10" cy="28.1" r="2" fill="red" fill-opacity="0.7" stroke="rgb(200,0,100)"/><path d="M 114.19 22.26 L 15.93 -15.76" fill="red" fill-opacity="0.7" stroke="rgb(200,0,100)" stroke-width="2.5"/></svg>
```

[`ToSvg`]: svg/trait.ToSvg.html
[`to_svg`]: svg/trait.ToSvg.html#method.to_svg
