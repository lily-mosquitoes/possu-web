mod pages;

#[cfg(test)]
mod dom {
    pub(crate) struct DOM;

    impl DOM {
        pub(crate) fn document() -> Option<web_sys::Document> {
            web_sys::window()?.document()
        }

        pub(crate) fn get_element_by_id(
            id: &str,
        ) -> Option<web_sys::Element> {
            DOM::document()?.get_element_by_id(id)
        }

        pub(crate) fn get_computed_style(
            element: &web_sys::Element,
        ) -> Option<web_sys::CssStyleDeclaration> {
            web_sys::window()?
                .get_computed_style(element)
                .expect("ComputedStyle to return Some")
        }

        pub(crate) fn is_element_visible(
            element: &web_sys::Element,
        ) -> bool {
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
