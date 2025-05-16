// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait XmlNode: std::fmt::Debug {
    type Children: Iterator<Item = Self>;

    fn get_children(&self, tag_name: &str) -> Self::Children;

    fn get_first_child(&self, tag_name: &str) -> Option<Self>
    where
        Self: Sized,
    {
        self.get_children(tag_name).next()
    }

    fn text(&self) -> Option<String>;
}

#[cfg(not(target_family = "wasm"))]
pub use native::read_node;

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::XmlNode;
    use crate::Result;
    use std::iter::Map;
    use std::vec::IntoIter;
    use xml_dom::level2::{Element, Node, NodeType, RefNode};
    use xml_dom::parser::read_xml;

    #[derive(Debug)]
    struct XmlNodeImpl(RefNode);

    impl XmlNode for XmlNodeImpl {
        type Children = Map<IntoIter<RefNode>, fn(RefNode) -> XmlNodeImpl>;

        fn get_children(&self, tag_name: &str) -> Self::Children {
            let child_elements = if self.0.node_type() == NodeType::Document {
                // Implement manually for documents
                let mut children = self.0.child_nodes();
                children.retain(|child| {
                    child.node_type() == NodeType::Element && child.tag_name() == tag_name
                });
                children
            } else {
                self.0.get_elements_by_tag_name(tag_name)
            };
            child_elements.into_iter().map(|child| XmlNodeImpl(child))
        }

        fn text(&self) -> Option<String> {
            let mut text = String::new();
            get_text(&self.0, &mut text);

            if !text.is_empty() {
                Some(text)
            } else {
                None
            }
        }
    }

    pub fn read_node(input: &str) -> Result<impl XmlNode> {
        let ref_node = read_xml(input)?;
        Ok(XmlNodeImpl(ref_node))
    }

    fn get_text(node: &RefNode, sink: &mut String) {
        if node.node_type() == NodeType::Text {
            if let Some(value) = node.node_value() {
                sink.push_str(value.as_str());
            }
        }

        for child in node.child_nodes() {
            get_text(&child, sink);
        }
    }
}

#[cfg(target_family = "wasm")]
pub use web::read_node;

#[cfg(target_family = "wasm")]
mod web {
    use super::XmlNode;
    use crate::result::{Error, Result};
    use std::iter::Map;
    use std::vec::IntoIter;
    use wasm_bindgen::JsCast;
    use web_sys::{Document, DomParser, Element, HtmlCollection, Node, SupportedType};

    #[derive(Debug)]
    struct XmlNodeImpl(Node);

    impl XmlNode for XmlNodeImpl {
        type Children = Map<IntoIter<Element>, fn(Element) -> XmlNodeImpl>;

        fn get_children(&self, tag_name: &str) -> Self::Children {
            if self.0.clone().is_instance_of::<Document>() {
                let doc = self.0.clone().dyn_into::<Document>().unwrap();
                let children = doc.get_elements_by_tag_name(tag_name);
                to_vec(children)
                    .into_iter()
                    .map(|child| XmlNodeImpl(child.into()))
            } else {
                let el = self.0.clone().dyn_into::<Element>().unwrap();
                let children = el.get_elements_by_tag_name(tag_name);
                to_vec(children)
                    .into_iter()
                    .map(|child| XmlNodeImpl(child.into()))
            }
        }

        fn text(&self) -> Option<String> {
            self.0.text_content()
        }
    }

    pub fn read_node(input: &str) -> Result<impl XmlNode> {
        let parser = DomParser::new().map_err(Error::from_js)?;
        let document = parser
            .parse_from_string(input, SupportedType::ApplicationXml)
            .map_err(Error::from_js)?;
        Ok(XmlNodeImpl(document.into()))
    }

    fn to_vec(collection: HtmlCollection) -> Vec<Element> {
        let mut result = Vec::with_capacity(collection.length() as usize);
        for i in 0..result.capacity() {
            result.push(collection.item(i as u32).unwrap());
        }
        result
    }
}
