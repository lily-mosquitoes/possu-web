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
    let section_id = format!("{}_select", props.id);
    let label_id = format!("{}_select_label", props.id);
    let field_id = format!("{}_select_field", props.id);

    html! {
        <section id={section_id}>
            <label
                id={label_id}
                for={field_id.clone()}
            >
                { props.label.clone() }
            </label>
            <select
                id={field_id}
                onchange={props.onchange.clone()}
            >
            {
                props.options.iter().map(|option| {
                    let key = option.value.as_str();
                    let value = &option.value;
                    let selected = option.selected;
                    let disabled = option.disabled;
                    html! {
                        <option {key} {value} {selected} {disabled}>{value}</option>
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

    #[wasm_bindgen_test]
    async fn select_is_section_with_expected_id() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_section_by_id(&format!("{}_select", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn select_contains_field_with_expected_id() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn select_field_is_inside_select_section() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id))
            .expect("Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.id(), format!("{}_select", id));
    }

    #[wasm_bindgen_test]
    async fn select_field_is_select() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id))
            .expect("Element to exist");

        assert_eq!(element.tag_name().as_str(), "SELECT");
    }

    #[wasm_bindgen_test]
    async fn select_field_has_options_with_value_of_options() {
        let id = "test";
        let options = vec!["First", "Second", "Third"];
        let mut props = select_props_with_id(id);
        props.options =
            options.iter().map(|s| SelectOption::from(*s)).collect();
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id))
            .expect("Element to exist");
        let select = element
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut select_options_values = vec![];
        for index in 0..select.length() {
            let option_element = select.get(index).expect("Element to exist");
            let value = option_element
                .get_attribute("value")
                .expect("value to exist");
            select_options_values.push(value);
        }

        assert_eq!(select_options_values, options);
    }

    #[wasm_bindgen_test]
    async fn select_field_has_options_with_inner_html_of_options() {
        let id = "test";
        let options = vec!["First", "Second", "Third"];
        let mut props = select_props_with_id(id);
        props.options =
            options.iter().map(|s| SelectOption::from(*s)).collect();
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id))
            .expect("Element to exist");
        let select = element
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut select_options_inner_htmls = vec![];
        for index in 0..select.length() {
            let option_element = select.get(index).expect("Element to exist");
            let inner_html = option_element.inner_html();
            select_options_inner_htmls.push(inner_html);
        }

        assert_eq!(select_options_inner_htmls, options);
    }

    #[wasm_bindgen_test]
    async fn select_field_has_correct_selected_value_when_given() {
        let id = "test";
        let options = vec!["First", "Second", "Third"];
        let selected = "Second";
        let mut props = select_props_with_id(id);
        props.options = options
            .iter()
            .map(|s| {
                let mut option = SelectOption::from(*s);
                option.selected = s == &selected;
                option
            })
            .collect();
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id))
            .expect("Element to exist");
        let select = element
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let selected_value = match select.selected_index() {
            i if i < 0 => None,
            i => {
                let selected_element =
                    select.get(i as u32).expect("Element to exist");
                selected_element.get_attribute("value")
            },
        };

        assert_eq!(selected_value, Some(selected.to_string()));
    }

    #[wasm_bindgen_test]
    async fn select_field_has_disabled_values_when_given() {
        let id = "test";
        let options = vec!["First", "Second", "Third", "Fourth"];
        let disabled = vec!["Second", "Fourth"];
        let mut props = select_props_with_id(id);
        props.options = options
            .iter()
            .map(|s| {
                let mut option = SelectOption::from(*s);
                option.disabled = disabled.contains(s);
                option
            })
            .collect();
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_field", id))
            .expect("Element to exist");
        let select = element
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut disabled_options = vec![];
        for index in 0..select.length() {
            let option_element = select.get(index).expect("Element to exist");
            let option_element = option_element
                .dyn_into::<HtmlOptionElement>()
                .expect("Element to be Option");
            if option_element.disabled() {
                disabled_options.push(option_element.value())
            }
        }

        assert_eq!(disabled_options, disabled);
    }

    #[wasm_bindgen_test]
    async fn select_field_accepts_onchange_callback() {
        let id = "test";
        let options = vec!["First", "Second", "Third", "Fourth"];
        let change_to = (2, "Third");
        let mut props = select_props_with_id(id);
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

        let select =
            DOM::get_html_select_by_id(&format!("{}_select_field", id))
                .expect("Select to exist");
        select.set_selected_index(change_to.0);
        dispatch_change_event(&select).await;

        let onchange_output = DOM::get_test_div().inner_html();
        assert_eq!(onchange_output, change_to.1);
    }

    #[wasm_bindgen_test]
    async fn select_contains_label_with_expected_id() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_label", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn select_label_is_inside_select_section() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_label", id))
            .expect("Element to exist");
        let parent = element.parent_element().expect("Parent Element to exist");

        assert_eq!(parent.id(), format!("{}_select", id));
    }

    #[wasm_bindgen_test]
    async fn select_label_is_label() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_label", id))
            .expect("Element to exist");

        assert_eq!(element.tag_name().as_str(), "LABEL");
    }

    #[wasm_bindgen_test]
    async fn select_label_has_for_with_select_field_id() {
        let id = "test";
        let props = select_props_with_id(id);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_label", id))
            .expect("Element to exist");

        assert_eq!(
            element.get_attribute("for"),
            Some(format!("{}_select_field", id))
        );
    }

    #[wasm_bindgen_test]
    async fn select_label_has_inner_html_with_label_text() {
        let id = "test";
        let label = "test label text";
        let mut props = select_props_with_id(id);
        props.label = AttrValue::from(label);
        render_select(props).await;

        let element = DOM::get_element_by_id(&format!("{}_select_label", id))
            .expect("Element to exist");

        assert_eq!(&element.inner_html(), label);
    }
}
