mod components;
pub mod pages;

#[cfg(test)]
mod dom {
    pub(crate) struct DOM;

    impl DOM {
        pub(crate) fn document() -> Option<web_sys::Document> {
            web_sys::window()?.document()
        }

        pub(crate) fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
            DOM::document()?.get_element_by_id(id)
        }

        pub(crate) fn get_section_by_id(id: &str) -> Option<web_sys::Element> {
            let element = DOM::document()?.get_element_by_id(id)?;
            match element.tag_name().as_str() {
                "SECTION" => Some(element),
                _ => None,
            }
        }

        pub(crate) fn get_input_by_id(id: &str) -> Option<web_sys::Element> {
            let element = DOM::document()?.get_element_by_id(id)?;
            match element.tag_name().as_str() {
                "INPUT" => Some(element),
                _ => None,
            }
        }

        pub(crate) fn get_select_by_id(id: &str) -> Option<web_sys::Element> {
            let element = DOM::document()?.get_element_by_id(id)?;
            match element.tag_name().as_str() {
                "SELECT" => Some(element),
                _ => None,
            }
        }

        pub(crate) fn get_button_by_id(id: &str) -> Option<web_sys::Element> {
            let element = DOM::document()?.get_element_by_id(id)?;
            match element.tag_name().as_str() {
                "BUTTON" => Some(element),
                _ => None,
            }
        }

        pub(crate) fn get_label_by_for(id: &str) -> Option<web_sys::Element> {
            let collection = DOM::document()?.get_elements_by_tag_name("LABEL");

            for i in 0..collection.length() {
                if let Some(element) = collection.item(i) {
                    if let Some(value) = element.get_attribute("for") {
                        if value == id {
                            return Some(element);
                        }
                    }
                }
            }

            None
        }

        pub(crate) fn get_computed_style(
            element: &web_sys::Element,
        ) -> Option<web_sys::CssStyleDeclaration> {
            web_sys::window()?
                .get_computed_style(element)
                .expect("ComputedStyle to return Some")
        }

        pub(crate) fn is_element_visible(element: &web_sys::Element) -> bool {
            match DOM::get_computed_style(element) {
                Some(style) => {
                    let display = style
                        .get_property_value("display")
                        .expect("property `display` to exist");

                    let visibility = style
                        .get_property_value("visibility")
                        .expect("property `visibility` to exist");

                    let opacity = style
                        .get_property_value("opacity")
                        .expect("property `opacity` to exist");

                    display != "none"
                        && visibility == "visible"
                        && opacity != "0"
                },
                None => false,
            }
        }
    }
}
