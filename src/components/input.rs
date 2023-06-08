use yew::{
    function_component,
    html,
    Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub(crate) struct InputProps {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) input_type: &'static str,
}

#[function_component(Input)]
pub(crate) fn input(props: &InputProps) -> Html {
    let input_id = format!("{}_input", props.id);
    let input_label_id = format!("{}_input_label", props.id);
    let input_field_id = format!("{}_input_field", props.id);

    html! {
        <section id={input_id}>
            <label
                id={input_label_id}
                for={input_field_id.clone()}
            >
                { props.label }
            </label>
            <input
                id={input_field_id}
                type={props.input_type}
            />
        </section>
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::{
        Input,
        InputProps,
    };
    use crate::dom::DOM;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn render_input(props: InputProps) {
        let output = DOM::get_element_by_id("output")
            .expect("Element `output` to exist");
        yew::Renderer::<Input>::with_root_and_props(output, props)
            .render();
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    #[wasm_bindgen_test]
    async fn input_has_id() {
        let id = "test";
        let props = InputProps {
            id,
            label: "",
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_contains_field_with_id() {
        let id = "test";
        let props = InputProps {
            id,
            label: "",
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_field", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_field_is_input() {
        let id = "test";
        let props = InputProps {
            id,
            label: "",
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_field", id))
                .expect("Element to exist");

        assert_eq!(element.tag_name().as_str(), "INPUT");
    }

    #[wasm_bindgen_test]
    async fn input_field_has_type_of_inputtype() {
        let id = "test";
        let input_type = "password";
        let props = InputProps {
            id,
            label: "",
            input_type,
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_field", id))
                .expect("Element to exist");

        assert_eq!(
            element.get_attribute("type"),
            Some(input_type.to_string())
        );
    }

    #[wasm_bindgen_test]
    async fn input_contains_label_with_id() {
        let id = "test";
        let props = InputProps {
            id,
            label: "",
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_label", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_label_is_label() {
        let id = "test";
        let props = InputProps {
            id,
            label: "",
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_label", id))
                .expect("Element to exist");

        assert_eq!(element.tag_name().as_str(), "LABEL");
    }

    #[wasm_bindgen_test]
    async fn input_label_has_for_with_input_field_id() {
        let id = "test";
        let props = InputProps {
            id,
            label: "",
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_label", id))
                .expect("Element to exist");

        assert_eq!(
            element.get_attribute("for"),
            Some(format!("{}_input_field", id))
        );
    }

    #[wasm_bindgen_test]
    async fn input_label_has_inner_html_with_label_text() {
        let id = "test";
        let label = "test label text";
        let props = InputProps {
            id,
            label,
            input_type: "",
        };
        render_input(props).await;

        let element =
            DOM::get_element_by_id(&format!("{}_input_label", id))
                .expect("Element to exist");

        assert_eq!(&element.inner_html(), label);
    }
}
