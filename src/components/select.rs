use std::rc::Rc;

use web_sys::Event;
use yew::{
    function_component,
    html,
    AttrValue,
    Callback,
    Html,
    Properties,
};

use crate::types::select::SelectOption;

#[derive(Properties, PartialEq)]
pub(crate) struct SelectProps {
    pub(crate) id: AttrValue,
    pub(crate) label: AttrValue,
    pub(crate) options: Rc<[SelectOption]>,
    #[prop_or_default]
    pub(crate) onchange: Option<Callback<Event>>,
}

#[function_component(Select)]
pub(crate) fn select(props: &SelectProps) -> Html {
    html! {
        <section id={format!("{}_section", props.id)}>
            <label
                id={format!("{}_label", props.id)}
                for={props.id.clone()}
            >
                { props.label.clone() }
            </label>
            <select
                id={props.id.clone()}
                onchange={props.onchange.clone()}
            >
            {
                props.options.iter().map(|option| {
                    let key = option.value.as_str();
                    let value = &option.value;
                    let inner_html = &option.inner_html;
                    let selected = option.selected;
                    let disabled = option.disabled;
                    html! {
                        <option {key} {value} {selected} {disabled}>{inner_html}</option>
                    }
                }).collect::<Html>()
            }
            </select>
        </section>
    }
}

#[cfg(test)]
mod test {
    use std::{
        rc::Rc,
        time::Duration,
    };

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use web_sys::{
        Event,
        HtmlOptionElement,
        HtmlSelectElement,
    };
    use yew::{
        AttrValue,
        Callback,
        TargetCast,
    };

    use super::{
        Select,
        SelectProps,
    };
    use crate::{
        dom::DOM,
        types::select::SelectOption,
    };

    wasm_bindgen_test_configure!(run_in_browser);

    async fn render_select(props: SelectProps) {
        let output = DOM::get_element_by_id("output")
            .expect("Element `output` to exist");
        yew::Renderer::<Select>::with_root_and_props(output, props).render();
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    fn select_props_with_id(id: &str) -> SelectProps {
        SelectProps {
            id: AttrValue::from(id.to_owned()),
            label: AttrValue::from(""),
            options: Rc::new([]),
            onchange: None,
        }
    }

    async fn dispatch_change_event(target: &HtmlSelectElement) {
        let event = Event::new("change").expect("valid event");
        target
            .dispatch_event(&event)
            .expect("event to be dispatched");
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    static TEST_ID: &str = "test_select";

    #[wasm_bindgen_test]
    async fn component_contains_select_element_with_expected_id() {
        let props = select_props_with_id(TEST_ID);
        render_select(props).await;

        let element = DOM::get_select_by_id(TEST_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn select_element_is_inside_section() {
        let props = select_props_with_id(TEST_ID);
        render_select(props).await;

        let element =
            DOM::get_select_by_id(TEST_ID).expect("Select Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.tag_name(), "SECTION");
    }

    #[wasm_bindgen_test]
    async fn select_element_has_given_options_values() {
        let values = vec!["First", "Second", "Third"];
        let options: Rc<[SelectOption]> = values
            .iter()
            .map(|&value| SelectOption {
                value: AttrValue::from(value),
                inner_html: AttrValue::from(format!("text-{}", value)),
                ..Default::default()
            })
            .collect();
        let mut props = select_props_with_id(TEST_ID);
        props.options = options;
        render_select(props).await;

        let element = DOM::get_select_by_id(TEST_ID)
            .expect("Select Element to exist")
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut select_options_values = vec![];
        for index in 0..element.length() {
            let option_element = element.get(index).expect("Element to exist");
            let value = option_element
                .get_attribute("value")
                .expect("value to exist");
            select_options_values.push(value);
        }

        assert_eq!(select_options_values, values);
    }

    #[wasm_bindgen_test]
    async fn select_element_has_given_options_inner_htmls() {
        let inner_htmls = vec!["First", "Second", "Third"];
        let options: Rc<[SelectOption]> = inner_htmls
            .iter()
            .map(|&inner_html| SelectOption {
                value: AttrValue::from(format!("value-{}", inner_html)),
                inner_html: AttrValue::from(inner_html),
                ..Default::default()
            })
            .collect();
        let mut props = select_props_with_id(TEST_ID);
        props.options = options;
        render_select(props).await;

        let element = DOM::get_select_by_id(TEST_ID)
            .expect("Select Element to exist")
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut select_options_inner_htmls = vec![];
        for index in 0..element.length() {
            let option_element = element.get(index).expect("Element to exist");
            select_options_inner_htmls.push(option_element.inner_html());
        }

        assert_eq!(select_options_inner_htmls, inner_htmls);
    }

    #[wasm_bindgen_test]
    async fn select_element_has_given_selected_option() {
        let options = vec!["First", "Second", "Third"];
        let selected = "Second";
        let mut props = select_props_with_id(TEST_ID);
        props.options = options
            .iter()
            .map(|s| {
                let mut option = SelectOption::from(*s);
                option.selected = s == &selected;
                option
            })
            .collect();
        render_select(props).await;

        let element = DOM::get_select_by_id(TEST_ID)
            .expect("Select Element to exist")
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let selected_value = match element.selected_index() {
            i if i < 0 => None,
            i => {
                let option = element.get(i as u32).expect("Element to exist");
                option.get_attribute("value")
            },
        };

        assert_eq!(selected_value, Some(selected.to_string()));
    }

    #[wasm_bindgen_test]
    async fn select_element_has_given_disabled_options() {
        let options = vec!["First", "Second", "Third", "Fourth"];
        let disabled = vec!["Second", "Fourth"];
        let mut props = select_props_with_id(TEST_ID);
        props.options = options
            .iter()
            .map(|s| {
                let mut option = SelectOption::from(*s);
                option.disabled = disabled.contains(s);
                option
            })
            .collect();
        render_select(props).await;

        let element = DOM::get_select_by_id(TEST_ID)
            .expect("Select Element to exist")
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut disabled_options = vec![];
        for index in 0..element.length() {
            let option = element
                .get(index)
                .expect("Element to exist")
                .dyn_into::<HtmlOptionElement>()
                .expect("Element to be Option");
            if option.disabled() {
                disabled_options.push(option.value())
            }
        }

        assert_eq!(disabled_options, disabled);
    }

    #[wasm_bindgen_test]
    async fn component_executes_given_onchange() {
        let options = vec!["First", "Second", "Third", "Fourth"];
        let change_to = (2, "Third");
        let mut props = select_props_with_id(TEST_ID);
        props.options = options
            .iter()
            .map(|v| SelectOption::from(*v).selected(*v == "Second"))
            .collect();
        props.onchange = Some(Callback::from(|e: Event| {
            let value =
                e.target_dyn_into::<HtmlSelectElement>().and_then(|select| {
                    let index = select.selected_index();
                    select
                        .get(index as u32)
                        .and_then(|selected| selected.get_attribute("value"))
                });
            if let Some(value) = value {
                let test_div = DOM::get_test_div();
                test_div.set_inner_html(&value);
            }
        }));
        render_select(props).await;

        let element = DOM::get_html_select_by_id(TEST_ID)
            .expect("Html Select Element to exist");
        element.set_selected_index(change_to.0);
        dispatch_change_event(&element).await;

        let onchange_output = DOM::get_test_div().inner_html();
        assert_eq!(onchange_output, change_to.1);
    }

    #[wasm_bindgen_test]
    async fn component_contains_label_element_for_select() {
        let props = select_props_with_id(TEST_ID);
        render_select(props).await;

        let element = DOM::get_label_by_for(TEST_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn label_element_is_inside_section() {
        let props = select_props_with_id(TEST_ID);
        render_select(props).await;

        let element =
            DOM::get_label_by_for(TEST_ID).expect("Label Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.tag_name(), "SECTION");
    }

    #[wasm_bindgen_test]
    async fn label_element_has_inner_html_given_by_label_prop() {
        let label = "test label text";
        let mut props = select_props_with_id(TEST_ID);
        props.label = AttrValue::from(label);
        render_select(props).await;

        let element = DOM::get_label_by_for(TEST_ID).expect("Element to exist");

        assert_eq!(&element.inner_html(), label);
    }
}
