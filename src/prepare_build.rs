use std::fs;
use minidom::Element;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project() {
    // Get manifest contents
    let manifest_contents = fs::read_to_string("manifest.xml").expect("No manifest.xml was found");
    // parse it
    let root: Element = manifest_contents.parse().unwrap();

    let mut articles: Vec<String> = Vec::new();

    for children in root.children() {
        if children.is("application", "http://www.garmin.com/xml/connectiq") {
            let language_children = children.get_child("languages", "http://www.garmin.com/xml/connectiq");
            match language_children {
                None => {}
                Some(element) => {
                    for child in element.children() {
                        articles.push(child.text());
                    }
                }
            }
        }
    }
}
