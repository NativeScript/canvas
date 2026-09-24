fn set(tag: &str, attrs: &[(&str, &str)]) {
    let mut node = canvas_svg::create_element(tag).expect("element");
    for (k, v) in attrs {
        canvas_svg::set_attribute(&mut node, k, v);
    }
}

#[test]
fn ellipse_attrs() {
    set("ellipse", &[("cx", "30"), ("cy", "90"), ("rx", "25"), ("ry", "15"), ("fill", "purple")]);
}

#[test]
fn line_attrs() {
    set("line", &[("x1", "70"), ("y1", "70"), ("x2", "140"), ("y2", "110"), ("stroke", "red"), ("stroke-width", "3")]);
}

#[test]
fn polygon_points() {
    set("polygon", &[("points", "75,120 100,145 50,145"), ("fill", "lime"), ("stroke", "black")]);
}

#[test]
fn polyline_points() {
    set("polyline", &[("points", "5,148 20,130 35,148 50,130"), ("fill", "none"), ("stroke", "blue"), ("stroke-width", "2")]);
}

#[test]
fn path_d() {
    set("path", &[("d", "M10 10 L70 10 L40 60 Z"), ("fill", "teal")]);
}

#[test]
fn g_transform() {
    set("g", &[("transform", "translate(80,10)")]);
}

#[test]
fn use_href() {
    set("use", &[("href", "#sharedCircle"), ("x", "30"), ("fill", "orange")]);
}

#[test]
fn text_attrs() {
    set("text", &[("x", "5"), ("y", "30"), ("font-size", "16"), ("fill", "darkblue")]);
}

#[test]
fn tspan_attrs() {
    set("tspan", &[("fill", "red")]);
}

#[test]
fn text_node() {
    let _ = canvas_svg::create_text_node("I love SVG!");
}
