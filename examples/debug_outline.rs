fn main() {
    // access via conversion
    let svg = std::fs::read("/tmp/noto-outline.svg").unwrap();
    let conv =
        officexml::presentation::svg_to_shapes::svg_to_shapes(&svg, 6_000_000, 1_200_000, 2).unwrap();
    println!("shapes {}", conv.shapes.len());
    for (i, sp) in conv.shapes.iter().enumerate() {
        let xml = officexml::element::write_element(sp).unwrap();
        let s = String::from_utf8_lossy(&xml);
        // extract off/ext
        let off_x = between(&s, "x=\"", "\"");
        let off_y = {
            if let Some(i) = s.find("<a:off ") {
                let rest = &s[i..];
                between(rest, "y=\"", "\"")
            } else {
                "0"
            }
        };
        let cx = between(&s, "cx=\"", "\"");
        let cy = between(&s, "cy=\"", "\"");
        println!("sp{i} off=({off_x},{off_y}) ext=({cx},{cy})");
        // scale: 6000000/600 = 10000 emu per px for x, 1200000/120=10000 for y
        let sx = 10000.0;
        println!(
            "  px box ({:.1},{:.1},{:.1},{:.1})",
            off_x.parse::<f64>().unwrap_or(0.0) / sx,
            off_y.parse::<f64>().unwrap_or(0.0) / sx,
            cx.parse::<f64>().unwrap_or(0.0) / sx,
            cy.parse::<f64>().unwrap_or(0.0) / sx,
        );
    }
}
fn between<'a>(s: &'a str, a: &str, b: &str) -> &'a str {
    let Some(i) = s.find(a) else { return "0" };
    let i = i + a.len();
    let Some(e) = s[i..].find(b) else { return "0" };
    &s[i..i + e]
}
