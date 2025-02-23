use std::collections::HashMap;

use web_sys::{Document, Element};

use crate::{AttributeValue, CreatedNode, EventAttribFn, VElement, VirtualNode};

mod add_events;

impl VElement {
    /// Build a DOM element by recursively creating DOM nodes for this element and it's
    /// children, it's children's children, etc.
    pub fn create_element_node(&self) -> CreatedNode<Element> {
        let document = web_sys::window().unwrap().document().unwrap();

        let element = if html_validation::is_svg_namespace(&self.tag) {
            document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), &self.tag)
                .unwrap()
        } else {
            document.create_element(&self.tag).unwrap()
        };

        let mut closures = HashMap::new();

        self.attrs.iter().for_each(|(name, value)| {
            match value {
                AttributeValue::String(s) => {
                    element.set_attribute(name, s).unwrap();
                }
                AttributeValue::Bool(b) => {
                    if *b {
                        element.set_attribute(name, "").unwrap();
                    }
                }
            };
        });

        self.add_events(&element, &mut closures);

        self.append_children_to_dom(&element, &document, &mut closures);

        #[cfg(target_arch = "wasm32")]
        {
            &self.special_attributes.maybe_call_on_create_elem(&element);

            if let Some(inner_html) = &self.special_attributes.dangerous_inner_html {
                element.set_inner_html(inner_html);
            }
        }

        CreatedNode {
            node: element,
            closures,
        }
    }
}

impl VElement {
    fn append_children_to_dom(
        &self,
        element: &Element,
        document: &Document,
        closures: &mut HashMap<u32, Vec<EventAttribFn>>,
    ) {
        let mut previous_node_was_text = false;

        self.children.iter().for_each(|child| {
            match child {
                VirtualNode::Text(text_node) => {
                    let current_node = element.as_ref() as &web_sys::Node;

                    // We ensure that the text siblings are patched by preventing the browser from merging
                    // neighboring text nodes. Originally inspired by some of React's work from 2016.
                    //  -> https://reactjs.org/blog/2016/04/07/react-v15.html#major-changes
                    //  -> https://github.com/facebook/react/pull/5753
                    //
                    // `ptns` = Percy text node separator
                    if previous_node_was_text {
                        let separator = document.create_comment("ptns");
                        current_node
                            .append_child(separator.as_ref() as &web_sys::Node)
                            .unwrap();
                    }

                    current_node
                        .append_child(&text_node.create_text_node())
                        .unwrap();

                    previous_node_was_text = true;
                }
                VirtualNode::Element(element_node) => {
                    previous_node_was_text = false;

                    let child = element_node.create_element_node();
                    let child_elem: Element = child.node;

                    closures.extend(child.closures);

                    element.append_child(&child_elem).unwrap();
                }
            }
        });
    }
}
