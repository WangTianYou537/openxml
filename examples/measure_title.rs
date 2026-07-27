//! Minimal text-measure / SVG shape smoke example.
//!
//! Replaces a broken stub that imported the removed internal `font_measure_hack`.

use officexml::presentation::svg_to_shapes::svg_to_shapes;

fn main() {
    let svg = br#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="40" viewBox="0 0 200 40">
  <text x="0" y="28" font-size="24" font-family="Liberation Sans">Title</text>
</svg>"#;
    let conv = svg_to_shapes(svg, 6_000_000, 1_200_000, 2).expect("svg_to_shapes");
    println!("shapes={}", conv.shapes.len());
    for font in &conv.used_fonts {
        println!(
            "font typeface={:?} bold={} path={:?}",
            font.typeface, font.bold, font.path
        );
    }
}
