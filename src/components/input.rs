use web_sys::InputEvent;
use yew::{
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum InputType {
    Text,
    Password,
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Password => write!(f, "password"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum InputMode {
    Text,
    Numeric,
}

impl std::fmt::Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Numeric => write!(f, "numeric"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct InputProps {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    #[prop_or(InputType::Text)]
    pub(crate) input_type: InputType,
    #[prop_or(InputMode::Text)]
    pub(crate) inputmode: InputMode,
    #[prop_or_default]
    pub(crate) placeholder: Option<&'static str>,
    #[prop_or_default]
    pub(crate) value: Option<&'static str>,
    #[prop_or_default]
    pub(crate) oninput: Option<Callback<InputEvent>>,
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
                type={props.input_type.to_string()}
                inputmode={props.inputmode.to_string()}
                placeholder={props.placeholder}
                value={props.value}
                oninput={props.oninput.clone()}
            />
        </section>
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use web_sys::{
        Event,
        HtmlInputElement,
        InputEvent,
    };
    use yew::Callback;

    use super::{
        Input,
        InputMode,
        InputProps,
        InputType,
    };
    use crate::dom::DOM;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn render_input(props: InputProps) {
        let output = DOM::get_element_by_id("output")
            .expect("Element `output` to exist");
        yew::Renderer::<Input>::with_root_and_props(output, props).render();
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    fn input_props_with_id(id: &'static str) -> InputProps {
        InputProps {
            id,
            label: "",
            input_type: InputType::Text,
            inputmode: InputMode::Text,
            placeholder: None,
            value: None,
            oninput: None,
        }
    }

    fn input_event() -> Event {
        Event::new("input").expect("valid event")
    }

    fn example_callback_with_test_and_expected_value(
    ) -> (Callback<InputEvent>, String, String) {
        let callback = Callback::from(|e: InputEvent| {
            let input = e
                .target()
                .expect("target to exist")
                .dyn_into::<HtmlInputElement>()
                .expect("input to exist");
            let value = input.value();
            let value: String =
                value.chars().filter(|c| c.is_digit(10)).collect();
            input.set_value(&value);
        });

        let test_input_value = "nj8672ndja982n".to_string();

        let expected_value = "8672982".to_string();

        (callback, test_input_value, expected_value)
    }

    #[wasm_bindgen_test]
    async fn input_is_section_with_expected_id() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_section_by_id(&format!("{}_input", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_contains_field_with_expected_id() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_field_is_inside_input_section() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.id(), format!("{}_input", id));
    }

    #[wasm_bindgen_test]
    async fn input_field_is_input() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");

        assert_eq!(element.tag_name().as_str(), "INPUT");
    }

    #[wasm_bindgen_test]
    async fn input_field_has_type_of_input_type() {
        let id = "test";
        let input_type = InputType::Password;
        let mut props = input_props_with_id(id);
        props.input_type = input_type;
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");

        assert_eq!(element.get_attribute("type"), Some(input_type.to_string()));
    }

    #[wasm_bindgen_test]
    async fn input_field_has_inputmode_of_inputmode() {
        let id = "test";
        let inputmode = InputMode::Numeric;
        let mut props = input_props_with_id(id);
        props.inputmode = inputmode;
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");

        assert_eq!(
            element.get_attribute("inputmode"),
            Some(inputmode.to_string())
        );
    }

    #[wasm_bindgen_test]
    async fn input_field_does_not_have_placeholder_when_not_given() {
        let id = "test";
        let mut props = input_props_with_id(id);
        props.placeholder = None;
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");

        assert_eq!(element.get_attribute("placeholder"), None);
    }

    #[wasm_bindgen_test]
    async fn input_field_has_placeholder_when_given() {
        let id = "test";
        let placeholder = "this is a placeholder";
        let mut props = input_props_with_id(id);
        props.placeholder = Some(placeholder);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");

        assert_eq!(
            element.get_attribute("placeholder"),
            Some(placeholder.to_string())
        );
    }

    #[wasm_bindgen_test]
    async fn input_field_does_not_have_value_when_not_given() {
        let id = "test";
        let mut props = input_props_with_id(id);
        props.value = None;
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");
        let input = element
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        assert_eq!(input.value(), "");
    }

    #[wasm_bindgen_test]
    async fn input_field_has_value_when_given() {
        let id = "test";
        let value = "this is a value";
        let mut props = input_props_with_id(id);
        props.value = Some(value);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");
        let input = element
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        assert_eq!(input.value(), value);
    }

    #[wasm_bindgen_test]
    async fn input_field_does_not_have_oninput_when_not_given() {
        let id = "test";
        let test_input_and_expected_value = "saw234kldfs778";
        let mut props = input_props_with_id(id);
        props.oninput = None;
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");
        let input = element
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        input.set_value(test_input_and_expected_value);
        let _ = input.dispatch_event(&input_event());

        assert_eq!(input.value(), test_input_and_expected_value);
    }

    #[wasm_bindgen_test]
    async fn input_field_has_oninput_when_given() {
        let id = "test";
        let (callback, test_input_value, expected_value) =
            example_callback_with_test_and_expected_value();
        let mut props = input_props_with_id(id);
        props.oninput = Some(callback);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_field", id))
            .expect("Element to exist");
        let input = element
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        input.set_value(&test_input_value);
        let _ = input.dispatch_event(&input_event());

        assert_eq!(input.value(), expected_value);
    }

    #[wasm_bindgen_test]
    async fn input_contains_label_with_expected_id() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_label", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_label_is_inside_input_section() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_label", id))
            .expect("Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.id(), format!("{}_input", id));
    }

    #[wasm_bindgen_test]
    async fn input_label_is_label() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_label", id))
            .expect("Element to exist");

        assert_eq!(element.tag_name().as_str(), "LABEL");
    }

    #[wasm_bindgen_test]
    async fn input_label_has_for_with_input_field_id() {
        let id = "test";
        let props = input_props_with_id(id);
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_label", id))
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
        let mut props = input_props_with_id(id);
        props.label = label;
        render_input(props).await;

        let element = DOM::get_element_by_id(&format!("{}_input_label", id))
            .expect("Element to exist");

        assert_eq!(&element.inner_html(), label);
    }
}
