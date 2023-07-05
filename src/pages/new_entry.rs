use std::rc::Rc;

use chrono::Utc;
use chrono_tz::Tz;
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
        DateTimeSelect,
        Input,
        MonetaryInput,
        Select,
    },
    requests,
    types::{
        datetime::DateTimeRange,
        select::SelectOption,
    },
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

    let timezone = Tz::Europe__Helsinki;
    let now = Utc::now().with_timezone(&timezone).fixed_offset();
    let selectable_range = DateTimeRange::from(
        now - chrono::Duration::days(2 * 365),
        now + chrono::Duration::days(2 * 365),
    );

    html! {
        <section id={"new_entry"}>
            <DateTimeSelect
                id={"datetime_select"}
                label={"Date"}
                range={Rc::from(selectable_range)}
                preselect={Rc::from(now)}
            />
            <Select
                id={"category_select"}
                label={"Category"}
                options={Rc::from((*categories).clone())}
            />
            <Input
                id={"description_input"}
                label={"Description"}
            />
            <MonetaryInput
                id={"value_input"}
                label={"Value"}
            />
        </section>
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use chrono::{
        DateTime,
        Datelike,
        FixedOffset,
        Utc,
    };
    use chrono_tz::Tz;
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

    static DATETIME_SELECT_ID: &str = "datetime_select";
    static CATEGORY_SELECT_ID: &str = "category_select";
    static DESCRIPTION_INPUT_ID: &str = "description_input";
    static VALUE_INPUT_ID: &str = "value_input";

    // DATETIME INPUT TESTS
    #[wasm_bindgen_test]
    async fn page_contains_datetime_select_elements() {
        render_new_entry().await;

        for postfix in &["year", "month", "day"] {
            let id = format!("{}_{}", DATETIME_SELECT_ID, postfix);
            let element = DOM::get_select_by_id(&id);

            assert!(element.is_some());
        }
    }

    #[wasm_bindgen_test]
    async fn datetime_select_elements_are_visible() {
        render_new_entry().await;

        for postfix in &["year", "month", "day"] {
            let id = format!("{}_{}", DATETIME_SELECT_ID, postfix);
            let element =
                DOM::get_select_by_id(&id).expect("Select Element to exist");

            assert!(DOM::is_element_visible(&element));
        }
    }

    #[wasm_bindgen_test]
    async fn page_contains_datetime_select_label_elements() {
        render_new_entry().await;

        for postfix in &["year", "month", "day"] {
            let id = format!("{}_{}", DATETIME_SELECT_ID, postfix);
            let element = DOM::get_label_by_for(&id);

            assert!(element.is_some());
        }
    }

    #[wasm_bindgen_test]
    async fn datetime_select_label_elements_are_visible() {
        render_new_entry().await;

        for postfix in &["year", "month", "day"] {
            let id = format!("{}_{}", DATETIME_SELECT_ID, postfix);
            let element =
                DOM::get_label_by_for(&id).expect("Label Element to exist");

            assert!(DOM::is_element_visible(&element));
        }
    }

    #[wasm_bindgen_test]
    async fn datetime_select_label_elements_have_expected_inner_htmls() {
        render_new_entry().await;

        for (postfix, expected_inner_html) in
            &[("year", "Year"), ("month", "Month"), ("day", "Day")]
        {
            let id = format!("{}_{}", DATETIME_SELECT_ID, postfix);
            let element =
                DOM::get_label_by_for(&id).expect("Label Element to exist");

            assert_eq!(&element.inner_html(), expected_inner_html);
        }
    }

    #[wasm_bindgen_test]
    async fn datetime_select_component_has_attribute_selected_datetime() {
        render_new_entry().await;

        let element = DOM::get_section_by_id(DATETIME_SELECT_ID)
            .expect("Section Element to exist");

        let attribute = element.get_attribute("selected_datetime");

        assert!(attribute.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_component_allows_2_years_before_and_after_now() {
        render_new_entry().await;

        let id = format!("{}_year", DATETIME_SELECT_ID);
        let element = DOM::get_select_by_id(&id)
            .expect("Select Element to exist")
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut available_years = vec![];
        for index in 0..element.length() {
            let option = element.get(index as u32).expect("Element to exist");
            available_years.push(option.inner_html());
        }

        let expected_start = Utc::now()
            .with_timezone(&Tz::Europe__Helsinki)
            .fixed_offset()
            - chrono::Duration::days(2 * 365);
        let expected_end = Utc::now()
            .with_timezone(&Tz::Europe__Helsinki)
            .fixed_offset()
            + chrono::Duration::days(2 * 365);
        let expected_available_years: Vec<String> = (expected_start.year()
            ..=expected_end.year())
            .map(|v| v.to_string())
            .collect();

        assert_eq!(available_years, expected_available_years);
    }

    #[wasm_bindgen_test]
    async fn datetime_select_component_has_current_datetime_preselected_in_selected_datetime_attribute(
    ) {
        render_new_entry().await;

        let element = DOM::get_section_by_id(DATETIME_SELECT_ID)
            .expect("Section Element to exist");

        let attribute = element
            .get_attribute("selected_datetime")
            .expect("Attribute to exist");

        let now = Utc::now()
            .with_timezone(&Tz::Europe__Helsinki)
            .fixed_offset();

        assert_eq!(attribute, now.to_rfc2822());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_component_has_current_datetime_preselected_in_select_elements(
    ) {
        render_new_entry().await;

        #[derive(Debug, PartialEq, Default)]
        struct Selected {
            year: Option<String>,
            month: Option<String>,
            day: Option<String>,
        }
        impl Selected {
            fn change(&mut self, attribute: &str, value: Option<String>) {
                match attribute {
                    "year" => self.year = value,
                    "month" => self.month = value,
                    "day" => self.day = value,
                    _ => panic!("no such attribute"),
                }
            }

            fn with_datetime(datetime: &DateTime<FixedOffset>) -> Self {
                Self {
                    year: Some(datetime.year().to_string()),
                    month: Some(datetime.month().to_string()),
                    day: Some(datetime.day().to_string()),
                }
            }
        }

        let mut selected = Selected::default();

        for postfix in &["year", "month", "day"] {
            let id = format!("{}_{}", DATETIME_SELECT_ID, postfix);
            let element = DOM::get_select_by_id(&id)
                .expect("Select Element to exist")
                .dyn_into::<HtmlSelectElement>()
                .expect("Element to be Select");

            let selected_index = element.selected_index();
            let option = element
                .get(selected_index as u32)
                .expect("Element to exist");
            selected.change(postfix, option.get_attribute("value"));
        }

        let now = Utc::now().with_timezone(&Tz::Europe__Helsinki);
        let expected_selected = Selected::with_datetime(&now.fixed_offset());

        assert_eq!(selected, expected_selected);
    }

    // REPEAT INPUT TESTS
    // CATEGORY SELECT TESTS
    #[wasm_bindgen_test]
    async fn page_contains_category_select_element() {
        render_new_entry().await;

        let element = DOM::get_select_by_id(CATEGORY_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn category_select_element_is_visible() {
        render_new_entry().await;

        let element = DOM::get_select_by_id(CATEGORY_SELECT_ID)
            .expect("Select Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn page_contains_category_select_label_element() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(CATEGORY_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn category_select_label_element_is_visible() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(CATEGORY_SELECT_ID)
            .expect("Label Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn category_select_label_element_has_expected_inner_html() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(CATEGORY_SELECT_ID)
            .expect("Label Element to exist");

        assert_eq!(&element.inner_html(), "Category");
    }

    #[wasm_bindgen_test]
    async fn category_select_element_has_expected_options() {
        render_new_entry().await;

        let element = DOM::get_select_by_id(CATEGORY_SELECT_ID)
            .expect("Select Element to exist")
            .dyn_into::<HtmlSelectElement>()
            .expect("Element to be Select");

        let mut select_options = vec![];
        for index in 0..element.length() {
            let option = element.get(index).expect("Element to exist");
            let inner_html = option.inner_html();
            select_options.push(inner_html);
        }

        let expected_options = crate::requests::get_categories()
            .await
            .expect("Categories to be returned");

        assert_eq!(select_options, expected_options);
    }

    // DESCRIPTION INPUT TESTS
    #[wasm_bindgen_test]
    async fn page_contains_description_input_element() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(DESCRIPTION_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn description_input_element_is_visible() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(DESCRIPTION_INPUT_ID)
            .expect("Input Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn page_contains_label_for_description_input_element() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(DESCRIPTION_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn description_input_label_element_is_visible() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(DESCRIPTION_INPUT_ID)
            .expect("Label Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn description_input_label_element_has_expected_inner_html() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(DESCRIPTION_INPUT_ID)
            .expect("Label Element to exist");

        assert_eq!(&element.inner_html(), "Description");
    }

    #[wasm_bindgen_test]
    async fn description_input_element_type_is_text() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(DESCRIPTION_INPUT_ID)
            .expect("Input Element to exist");
        let input_type = element.get_attribute("type");

        assert_eq!(input_type, Some("text".to_string()));
    }

    // VALUE INPUT TESTS
    #[wasm_bindgen_test]
    async fn page_contains_value_input_element() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(VALUE_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn value_input_element_is_visible() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(VALUE_INPUT_ID)
            .expect("Input Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn page_contains_label_for_value_input_element() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(VALUE_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn value_input_label_element_is_visible() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(VALUE_INPUT_ID)
            .expect("Label Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn value_input_label_element_has_expected_inner_html() {
        render_new_entry().await;

        let element = DOM::get_label_by_for(VALUE_INPUT_ID)
            .expect("Label Element to exist");

        assert_eq!(&element.inner_html(), "Value");
    }

    #[wasm_bindgen_test]
    async fn value_input_element_type_is_text() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(VALUE_INPUT_ID)
            .expect("Input Element to exist");
        let input_type = element.get_attribute("type");

        assert_eq!(input_type, Some("text".to_string()));
    }

    #[wasm_bindgen_test]
    async fn value_input_element_inputmode_is_numeric() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(VALUE_INPUT_ID)
            .expect("Input Element to exist");
        let input_mode = element.get_attribute("inputmode");

        assert_eq!(input_mode, Some("numeric".to_string()));
    }

    #[wasm_bindgen_test]
    async fn value_input_element_placeholder_is_string_representing_monetary_zero(
    ) {
        render_new_entry().await;

        let element = DOM::get_input_by_id(VALUE_INPUT_ID)
            .expect("Input Element to exist");
        let placeholder = element.get_attribute("placeholder");

        assert_eq!(placeholder, Some("0.00".to_string()));
    }

    #[wasm_bindgen_test]
    async fn value_input_element_value_is_formatted_on_input() {
        render_new_entry().await;

        let element = DOM::get_input_by_id(VALUE_INPUT_ID)
            .expect("Input Element to exist")
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
            element.set_value(case.0);
            dispatch_input_event(&element).await;

            assert_eq!(element.value(), case.1);
        }
    }
}
