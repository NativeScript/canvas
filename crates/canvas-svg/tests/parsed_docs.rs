
const SRC: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 300 300">
  <rect id="box" x="10" y="10" width="50" height="50" fill="green">
    <animate attributeName="x" from="10" to="200" dur="2s" repeatCount="indefinite"/>
  </rect>
  <g id="grp"><circle id="dot" cx="100" cy="100" r="20" fill="red"/></g>
</svg>"#;

#[test]
fn get_element_by_id_on_a_parsed_document() {
    let mut doc = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).expect("parses");
    // Parsed ids live only in Skia's id map; ours is filled by register_id.
    println!("box => {}", doc.get_element_by_id("box").is_some());
    println!("grp => {}", doc.get_element_by_id("grp").is_some());
    println!("dot => {}", doc.get_element_by_id("dot").is_some());
}

#[test]
fn ids_from_the_parser_resolve() {
    let mut doc = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).expect("parses");
    assert!(doc.get_element_by_id("box").is_some(), "top-level id");
    assert!(doc.get_element_by_id("grp").is_some(), "group id");
    assert!(doc.get_element_by_id("dot").is_some(), "nested id");
    assert!(doc.get_element_by_id("nope").is_none());
}
