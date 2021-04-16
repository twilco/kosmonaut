/// This file is a direct copy-paste from [Kuchiki](https://github.com/kuchiki-rs/kuchiki/blob/master/src/tests.rs).
/// Thanks to the authors of Kuchiki for their work.
use html5ever::tree_builder::QuirksMode;
use std::path::Path;

use tempdir::TempDir;

use crate::iter::NodeIterator;
use crate::parser::parse_html;
use crate::selectors_integration::filter_selectors;
use crate::tree::{NodeData, NodeRef};
use html5ever::tendril::TendrilSink;
use kosmonaut_selectors::{Selectors, Specificity};

#[test]
fn text_nodes() {
    let html = r"
<!doctype html>
<title>Test case</title>
<p>Content contains <b>Important</b> data</p>";
    let document = parse_html().one(html);
    let paragraph = document.select_str("p").unwrap().collect::<Vec<_>>();
    assert_eq!(paragraph.len(), 1);
    assert_eq!(
        paragraph[0].text_contents(),
        "Content contains Important data"
    );
    let texts = paragraph[0]
        .as_node()
        .descendants()
        .text_nodes()
        .collect::<Vec<_>>();
    assert_eq!(texts.len(), 3);
    assert_eq!(&*texts[0].borrow(), "Content contains ");
    assert_eq!(&*texts[1].borrow(), "Important");
    assert_eq!(&*texts[2].borrow(), " data");
    {
        let mut x = texts[0].borrow_mut();
        x.truncate(0);
        x.push_str("Content doesn't contain ");
    }
    assert_eq!(&*texts[0].borrow(), "Content doesn't contain ");
}

#[test]
fn parse_and_serialize() {
    let html = r"
<!doctype html>
<title>Test case</title>
<p>Content";
    let document = parse_html().one(html);
    assert_eq!(
        document.as_document().unwrap().quirks_mode(),
        QuirksMode::NoQuirks
    );
    assert_eq!(
        document.to_string(),
        r"<!DOCTYPE html><html><head><title>Test case</title>
</head><body><p>Content</p></body></html>"
    );
}

#[test]
fn parse_file() {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("test_data/foo.html".to_string());

    let html = r"<!DOCTYPE html><html><head>
        <title>Test case</title>
    </head>
    <body>
        <p>Foo</p>
    

</body></html>";
    let document = parse_html().from_utf8().from_file(&path).unwrap();
    assert_eq!(document.to_string(), html);
}

#[test]
fn serialize_and_read_file() {
    let tempdir = TempDir::new("test_rm_tempdir").unwrap();
    let mut path = tempdir.path().to_path_buf();
    path.push("temp.html");

    let html = r"<!DOCTYPE html><html><head><title>Title</title></head><body>Body</body></html>";
    let document = parse_html().one(html);
    let _ = document.serialize_to_file(path.clone());

    let document2 = parse_html().from_utf8().from_file(&path).unwrap();
    assert_eq!(document.to_string(), document2.to_string());
}

#[test]
fn select() {
    let html = r"
<title>Test case</title>
<p class=foo>Foo
<p>Bar
<p class=foo>Foo
";

    let document = parse_html().one(html);
    let matching = document.select_str("p.foo").unwrap().collect::<Vec<_>>();
    assert_eq!(matching.len(), 2);
    let child = matching[0].as_node().first_child().unwrap();
    assert_eq!(&**child.as_text().unwrap().borrow(), "Foo\n");
    assert_eq!(matching[0].attributes.borrow().get("class"), Some("foo"));
    assert_eq!(
        matching[0].attributes.borrow().get(local_name!("class")),
        Some("foo")
    );

    let selectors = Selectors::compile_str("p.foo").unwrap();
    let matching2 =
        filter_selectors(document.descendants().elements(), &selectors).collect::<Vec<_>>();
    assert_eq!(matching, matching2);
}

#[test]
fn select_first() {
    let html = r"
<title>Test case</title>
<p class=foo>Foo
<p>Bar
<p class=foo>Baz
";

    let document = parse_html().one(html);
    let matching = document.select_first("p.foo").unwrap();
    let child = matching.as_node().first_child().unwrap();
    assert_eq!(&**child.as_text().unwrap().borrow(), "Foo\n");
    assert_eq!(matching.attributes.borrow().get("class"), Some("foo"));
    assert_eq!(
        matching.attributes.borrow().get(local_name!("class")),
        Some("foo")
    );

    assert!(document.select_first("p.bar").is_err());
}

#[test]
fn to_string() {
    let html = r"<!DOCTYPE html>
<html>
    <head>
        <title>Test case</title>
    </head>
    <body>
        <p class=foo>Foo
    </body>
</html>";

    let document = parse_html().one(html);
    assert_eq!(
        document
            .inclusive_descendants()
            .nth(11)
            .unwrap()
            .to_string(),
        "<p class=\"foo\">Foo\n    \n</p>"
    );
}

#[test]
fn specificity() {
    let selectors = Selectors::compile_str(".example, :first-child, div").unwrap();
    let specificities = selectors
        .0
        .iter()
        .map(|s| s.specificity())
        .collect::<Vec<_>>();
    assert_eq!(specificities.len(), 3);
    assert_eq!(specificities[0], specificities[1]);
    assert!(specificities[0] > specificities[2]);
    assert!(specificities[1] > specificities[2]);
}

/// Returns a single <div></div> NodeRef.
///   * classes - What would go inside <div class="HERE">.  Space-separated list of classnames.
///   * text - Text to insert inside the div
fn get_div(classes: &str, text: &str) -> NodeRef {
    let div = format!(r#"<div class="{}">{}</div>"#, classes, text);
    let mut ret: Option<NodeRef> = None;
    parse_html()
        .from_utf8()
        .read_from(&mut div.as_bytes())
        .unwrap()
        .inclusive_descendants()
        .for_each(|node| {
            if let NodeData::Element(data) = node.data() {
                if let local_name!("div") = data.name.local {
                    ret = Some(node)
                }
            }
        });
    ret.expect("should've been able to get div from test_utils#get_div()")
}

#[test]
fn match_most_specific_works() {
    let selectors = Selectors::compile_str("div, div.specific, div.specific.even-more-specific")
        .expect("should've been able to compile selectors in match_most_specific_works()");
    let div = get_div("specific even-more-specific", "hello");
    // specificity of most specific selector, `div.specific.even-more-specific`, is 2049
    assert_eq!(
        div.into_element_ref()
            .expect("should be able to get element ref for canned node")
            .most_specific_match(&selectors)
            .expect("should've found a most-specific match")
            .specificity(),
        Specificity::new(2049)
    )
}
