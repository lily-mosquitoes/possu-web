use web_sys::InputEvent;
use yew::{
    function_component,
    html,
    AttrValue,
    Callback,
    Html,
    Properties,
};

#[derive(Properties, PartialEq)]
pub(crate) struct InputProps {
    pub(crate) id: AttrValue,
    pub(crate) label: AttrValue,
    #[prop_or(InputType::Text)]
    pub(crate) input_type: InputType,
    #[prop_or(InputMode::Text)]
    pub(crate) inputmode: InputMode,
    #[prop_or_default]
    pub(crate) placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub(crate) value: Option<AttrValue>,
    #[prop_or_default]
    pub(crate) oninput: Option<Callback<InputEvent>>,
}

#[function_component(Input)]
pub(crate) fn input(props: &InputProps) -> Html {
    html! {
        <section id={format!("{}_section", props.id)}>
            <label
                id={format!("{}_label", props.id)}
                for={props.id.clone()}
            >
                { props.label.clone() }
            </label>
            <input
                id={props.id.clone()}
                type={props.input_type.to_string()}
                inputmode={props.inputmode.to_string()}
                placeholder={props.placeholder.clone()}
                value={props.value.clone()}
                oninput={props.oninput.clone()}
            />
        </section>
    }
}

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
    use yew::{
        AttrValue,
        Callback,
        TargetCast,
    };

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

    fn input_props_with_id(id_prefix: &str) -> InputProps {
        InputProps {
            id: AttrValue::from(id_prefix.to_owned()),
            label: AttrValue::from(""),
            input_type: InputType::Text,
            inputmode: InputMode::Text,
            placeholder: None,
            value: None,
            oninput: None,
        }
    }

    async fn dispatch_input_event(target: &HtmlInputElement) {
        let event = Event::new("input").expect("valid event");
        target
            .dispatch_event(&event)
            .expect("event to be dispatched");
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    static TEST_ID: &str = "test_input";

    #[wasm_bindgen_test]
    async fn component_contains_input_element_with_expected_id() {
        let props = input_props_with_id(TEST_ID);
        render_input(props).await;

        let element = DOM::get_input_by_id(TEST_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn input_element_is_inside_section() {
        let props = input_props_with_id(TEST_ID);
        render_input(props).await;

        let element =
            DOM::get_input_by_id(TEST_ID).expect("Input Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.tag_name(), "SECTION");
    }

    #[wasm_bindgen_test]
    async fn input_element_has_given_input_type() {
        let input_type = InputType::Password;
        let mut props = input_props_with_id(TEST_ID);
        props.input_type = input_type;
        render_input(props).await;

        let element =
            DOM::get_input_by_id(TEST_ID).expect("Input Element to exist");

        assert_eq!(element.get_attribute("type"), Some(input_type.to_string()));
    }

    #[wasm_bindgen_test]
    async fn input_element_has_given_inputmode() {
        let inputmode = InputMode::Numeric;
        let mut props = input_props_with_id(TEST_ID);
        props.inputmode = inputmode;
        render_input(props).await;

        let element =
            DOM::get_input_by_id(TEST_ID).expect("Input Element to exist");

        assert_eq!(
            element.get_attribute("inputmode"),
            Some(inputmode.to_string())
        );
    }

    #[wasm_bindgen_test]
    async fn input_element_does_not_have_placeholder_when_not_given() {
        let mut props = input_props_with_id(TEST_ID);
        props.placeholder = None;
        render_input(props).await;

        let element =
            DOM::get_input_by_id(TEST_ID).expect("Input Element to exist");

        assert_eq!(element.get_attribute("placeholder"), None);
    }

    #[wasm_bindgen_test]
    async fn input_element_has_placeholder_when_given() {
        let placeholder = "this is a placeholder";
        let mut props = input_props_with_id(TEST_ID);
        props.placeholder = Some(AttrValue::from(placeholder));
        render_input(props).await;

        let element =
            DOM::get_input_by_id(TEST_ID).expect("Input Element to exist");

        assert_eq!(
            element.get_attribute("placeholder"),
            Some(placeholder.to_string())
        );
    }

    #[wasm_bindgen_test]
    async fn input_element_does_not_have_value_when_not_given() {
        let mut props = input_props_with_id(TEST_ID);
        props.value = None;
        render_input(props).await;

        let element = DOM::get_input_by_id(TEST_ID)
            .expect("Input Element to exist")
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        assert_eq!(element.value(), "");
    }

    #[wasm_bindgen_test]
    async fn input_element_has_value_when_given() {
        let value = "this is a value";
        let mut props = input_props_with_id(TEST_ID);
        props.value = Some(AttrValue::from(value));
        render_input(props).await;

        let element = DOM::get_input_by_id(TEST_ID)
            .expect("Input Element to exist")
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        assert_eq!(element.value(), value);
    }

    #[wasm_bindgen_test]
    async fn component_executes_given_oninput() {
        let filter_digits_function = |s: &str| -> String {
            s.chars().filter(|c| c.is_digit(10)).collect()
        };
        let filter_digits = Callback::from(move |e: InputEvent| {
            if let Some(element) = e.target_dyn_into::<HtmlInputElement>() {
                let value = filter_digits_function(&element.value());
                element.set_value(&value);
            };
        });
        let substitute_by_hello = Callback::from(|e: InputEvent| {
            if let Some(element) = e.target_dyn_into::<HtmlInputElement>() {
                element.set_value("hello");
            };
        });
        let nothing = Callback::from(|_| {});

        let test_inputs = vec!["djs3a564ld92l", "1234", "odkxa", ""];
        let test_callbacks = vec![
            (0, Some(filter_digits)),
            (1, Some(substitute_by_hello)),
            (2, Some(nothing)),
            (3, None),
        ];
        let expected_output = |&index, input| -> String {
            match (index, input) {
                (0, input) => filter_digits_function(input),
                (1, _) => String::from("hello"),
                (_, input) => input.to_string(),
            }
        };

        for input in test_inputs {
            for (index, callback) in &test_callbacks {
                let mut props = input_props_with_id(TEST_ID);
                props.oninput = callback.clone();
                render_input(props).await;

                let element = DOM::get_input_by_id(TEST_ID)
                    .expect("Element to exist")
                    .dyn_into::<HtmlInputElement>()
                    .expect("Element to be Input");

                element.set_value(&input);
                dispatch_input_event(&element).await;

                assert_eq!(element.value(), expected_output(index, input));
            }
        }
    }

    #[wasm_bindgen_test]
    async fn component_contains_label_element_for_input() {
        let props = input_props_with_id(TEST_ID);
        render_input(props).await;

        let element = DOM::get_label_by_for(TEST_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn label_element_is_inside_section() {
        let props = input_props_with_id(TEST_ID);
        render_input(props).await;

        let element =
            DOM::get_label_by_for(TEST_ID).expect("Label Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.tag_name(), "SECTION");
    }

    #[wasm_bindgen_test]
    async fn label_element_has_inner_html_given_by_label_prop() {
        let label = "test label text";
        let mut props = input_props_with_id(TEST_ID);
        props.label = AttrValue::from(label);
        render_input(props).await;

        let element =
            DOM::get_label_by_for(TEST_ID).expect("Label Element to exist");

        assert_eq!(&element.inner_html(), label);
    }
}
