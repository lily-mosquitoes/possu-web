use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component,
    html,
    use_effect,
    use_state_eq,
    Html,
};

use crate::{
    components::{
        Input,
        MonetaryInput,
        Select,
    },
    requests,
    types::select::SelectOption,
};

#[function_component(NewEntry)]
pub fn new_entry() -> Html {
    let categories = use_state_eq(|| Vec::<SelectOption>::default());

    {
        let categories = categories.clone();
        use_effect(move || {
            spawn_local(async move {
                let response = requests::get_categories().await;
                if let Ok(list) = response {
                    let options =
                        list.iter().map(|s| SelectOption::from(s)).collect();
                    categories.set(options);
                }
            });
        });
    }

    html! {
        <section id={"new_entry"}>
            <Select
                id={"category"}
                label={"Category"}
                options={Rc::from((*categories).clone())}
            />
            <Input
                id={"description"}
                label={"Description"}
            />
            <MonetaryInput
                id={"value"}
                label={"Value"}
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
        HtmlSelectElement,
    };

    use super::NewEntry;
    use crate::dom::DOM;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn render_new_entry() {
        let output = DOM::get_element_by_id("output")
            .expect("Element `output` to exist");
        yew::Renderer::<NewEntry>::with_root(output).render();
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    async fn dispatch_input_event(target: &HtmlInputElement) {
        let event = Event::new("input").expect("valid event");
        target
            .dispatch_event(&event)
            .expect("Event to be dispatched");
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    // DATETIME INPUT TESTS
    // REPEAT INPUT TESTS
    // CATEGORY SELECT TESTS
    #[wasm_bindgen_test]
    async fn category_select_field_with_label_exists() {
        render_new_entry().await;

        let field = DOM::get_select_by_id("category_select_field");
        let label = DOM::get_label_by_for("category_select_field");

        assert!(field.is_some() && label.is_some());
    }

    #[wasm_bindgen_test]
    async fn category_select_field_and_label_are_visible() {
        render_new_entry().await;

        let field = DOM::get_select_by_id("category_select_field")
            .expect("category select field to exist");
        let label = DOM::get_label_by_for("category_select_field")
            .expect("category select field label to exist");

        assert!(
            DOM::is_element_visible(&field) && DOM::is_element_visible(&label)
        );
    }

    #[wasm_bindgen_test]
    async fn category_select_field_label_has_expected_inner_html() {
        render_new_entry().await;

        let label = DOM::get_label_by_for("category_select_field")
            .expect("category select field label to exist");

        assert_eq!(&label.inner_html(), "Category");
    }

    #[wasm_bindgen_test]
    async fn category_select_field_has_expected_options() {
        render_new_entry().await;

        let field = DOM::get_select_by_id("category_select_field")
            .expect("category select field to exist");
        let select = field
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut select_options = vec![];
        for index in 0..select.length() {
            let option_element = select.get(index).expect("Element to exist");
            let inner_html = option_element.inner_html();
            select_options.push(inner_html);
        }

        let expected_options = crate::requests::get_categories()
            .await
            .expect("Categories to be returned");

        assert_eq!(select_options, expected_options);
    }

    // DESCRIPTION INPUT TESTS
    #[wasm_bindgen_test]
    async fn description_input_field_with_label_exists() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("description_input_field");
        let label = DOM::get_label_by_for("description_input_field");

        assert!(field.is_some() && label.is_some());
    }

    #[wasm_bindgen_test]
    async fn description_input_field_and_label_are_visible() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("description_input_field")
            .expect("description input field to exist");
        let label = DOM::get_label_by_for("description_input_field")
            .expect("description input field label to exist");

        assert!(
            DOM::is_element_visible(&field) && DOM::is_element_visible(&label)
        );
    }

    #[wasm_bindgen_test]
    async fn description_input_field_label_has_expected_inner_html() {
        render_new_entry().await;

        let label = DOM::get_label_by_for("description_input_field")
            .expect("description input field label to exist");

        assert_eq!(&label.inner_html(), "Description");
    }

    #[wasm_bindgen_test]
    async fn description_input_field_type_is_text() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("description_input_field")
            .expect("description input field to exist");
        let input_type = field.get_attribute("type");

        assert_eq!(input_type, Some("text".to_string()));
    }

    // VALUE INPUT TESTS
    #[wasm_bindgen_test]
    async fn value_input_field_with_label_exists() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("value_input_field");
        let label = DOM::get_label_by_for("value_input_field");

        assert!(field.is_some() && label.is_some());
    }

    #[wasm_bindgen_test]
    async fn value_input_field_and_label_are_visible() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("value_input_field")
            .expect("value input field to exist");
        let label = DOM::get_label_by_for("value_input_field")
            .expect("value input field label to exist");

        assert!(
            DOM::is_element_visible(&field) && DOM::is_element_visible(&label)
        );
    }

    #[wasm_bindgen_test]
    async fn value_input_field_label_has_expected_inner_html() {
        render_new_entry().await;

        let label = DOM::get_label_by_for("value_input_field")
            .expect("value input field label to exist");

        assert_eq!(&label.inner_html(), "Value");
    }

    #[wasm_bindgen_test]
    async fn value_input_field_type_is_text() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("value_input_field")
            .expect("value input field to exist");
        let input_type = field.get_attribute("type");

        assert_eq!(input_type, Some("text".to_string()));
    }

    #[wasm_bindgen_test]
    async fn value_input_field_inputmode_is_numeric() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("value_input_field")
            .expect("value input field to exist");
        let input_type = field.get_attribute("inputmode");

        assert_eq!(input_type, Some("numeric".to_string()));
    }

    #[wasm_bindgen_test]
    async fn value_input_field_placeholder_has_expected_string() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("value_input_field")
            .expect("value input field to exist");
        let input_type = field.get_attribute("placeholder");

        assert_eq!(input_type, Some("0.00".to_string()));
    }

    #[wasm_bindgen_test]
    async fn value_input_field_value_is_formatted_on_input() {
        render_new_entry().await;

        let field = DOM::get_input_by_id("value_input_field")
            .expect("value input field to exist");
        let input = field
            .dyn_into::<HtmlInputElement>()
            .expect("Element to be Input");

        let tests = vec![
            ("", "0.00"),
            ("0", "0.00"),
            ("0000000000", "0.00"),
            ("2", "0.02"),
            ("23", "0.23"),
            ("fas", "0.00"),
            ("fas1", "0.01"),
            ("fas1234", "12.34"),
            ("fas0001234", "12.34"),
            ("fas0001s234juhda", "12.34"),
            ("fas0001s234juhda9", "123.49"),
            ("fas0001s234juhda900", "12,349.00"),
            ("9223372036854775807", "92,233,720,368,547,758.07"),
            ("9223372036854775808", "9,223,372,036,854,775.80"),
        ];

        for case in tests {
            input.set_value(case.0);
            dispatch_input_event(&input).await;

            assert_eq!(input.value(), case.1);
        }
    }
}
