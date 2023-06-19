use wasm_bindgen::JsCast;
use web_sys::{
    HtmlInputElement,
    InputEvent,
};
use yew::{
    function_component,
    html,
    use_state,
    AttrValue,
    Callback,
    Html,
};

use crate::components::{
    Input,
    InputMode,
    InputType,
};

#[function_component(NewEntry)]
pub fn new_entry() -> Html {
    let input_value = use_state(|| String::default());

    let format_input = {
        let input_value = input_value.clone();

        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let input =
                target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let new_value = convert_digit_string_to_money(&input.value());
                input_value.set(new_value);
            }
        })
    };

    html! {
        <section id={"new_entry"}>
            <Input
                id={"value"}
                label={"Value"}
                input_type={InputType::Text}
                inputmode={InputMode::Numeric}
                placeholder={"0.00"}
                oninput={format_input}
                value={AttrValue::from((*input_value).clone())}
            />
        </section>
    }
}

fn filter_digits(string: &str) -> String {
    string
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
}

fn truncate_to_valid_i64(string: &str) -> String {
    let mut string = match filter_digits(string).trim_start_matches('0') {
        s if s.len() == 0 => String::from("0"),
        s => s.to_owned(),
    };
    let max = i64::MAX.to_string();
    string.truncate(max.len());
    while let Err(_) = string.parse::<i64>() {
        string.truncate(string.len() - 1);
    }
    string
}

fn add_thousands_separator(string: &str) -> String {
    match string.len() {
        n if n <= 3 => string.to_string(),
        n if n <= 6 => {
            let (a, b) = string.split_at(n - 3);
            format!("{},{}", a, b)
        },
        n => {
            let (a, b) = string.split_at(n - 6);
            let a = add_thousands_separator(a);
            let b = add_thousands_separator(b);
            format!("{},{}", a, b)
        },
    }
}

fn convert_digit_string_to_money(string: &str) -> String {
    let string = truncate_to_valid_i64(string);

    match string.len() {
        0 => String::from("0.00"),
        1 => format!("0.0{}", string),
        2 => format!("0.{}", string),
        n => {
            let (a, b) = string.split_at(n - 2);
            format!("{}.{}", add_thousands_separator(a), b)
        },
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
    };

    use super::{
        add_thousands_separator,
        convert_digit_string_to_money,
        filter_digits,
        truncate_to_valid_i64,
        NewEntry,
    };
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
    // CATEGORY INPUT TESTS
    // DESCRIPTION INPUT TESTS
    // VALUE INPUT TESTS
    #[wasm_bindgen_test]
    fn filter_digits_works() {
        let tests = vec![
            ("092019828832l", "092019828832"),
            ("232jhdsa878dsa", "232878"),
            ("+242jhdsa878ds", "242878"),
            ("2f", "2"),
            ("-", ""),
            ("三", ""),
            ("1¾", "1"),
            ("8.34", "834"),
            ("128.97 €", "12897"),
        ];

        for case in tests {
            assert_eq!(&filter_digits(case.0), case.1);
        }
    }

    #[wasm_bindgen_test]
    fn truncate_to_valid_i64_works() {
        let tests = vec![
            ("9223372036854775807", "9223372036854775807"),
            ("9223372036854775808", "922337203685477580"),
            ("999999999999999999", "999999999999999999"),
            ("9999999999999999999", "999999999999999999"),
            (
                "9999999999999999999999999999999999999999999999999999",
                "999999999999999999",
            ),
        ];

        for case in tests {
            assert_eq!(&truncate_to_valid_i64(case.0), case.1);
        }
    }

    #[wasm_bindgen_test]
    fn add_thousands_separator_works() {
        let tests = vec![
            ("123", "123"),
            ("1234", "1,234"),
            ("12345", "12,345"),
            ("999999999999999999999999", "999,999,999,999,999,999,999,999"),
            ("9999999999999999999999999", "9,999,999,999,999,999,999,999,999"),
        ];

        for case in tests {
            assert_eq!(&add_thousands_separator(case.0), case.1);
        }
    }

    #[wasm_bindgen_test]
    fn convert_digit_string_to_money_works() {
        let tests = vec![
            ("-", "0.00"),
            ("jdsjakhsd", "0.00"),
            ("¾", "0.00"),
            ("", "0.00"),
            ("0", "0.00"),
            ("1", "0.01"),
            ("12", "0.12"),
            ("128", "1.28"),
            ("128", "1.28"),
            ("1289", "12.89"),
            ("12890", "128.90"),
            ("12890", "128.90"),
            ("128900", "1,289.00"),
            ("128900123", "1,289,001.23"),
            ("00000012890", "128.90"),
            ("000000128", "1.28"),
            ("00000012", "0.12"),
            ("0000000000000", "0.00"),
            ("9223372036854775807", "92,233,720,368,547,758.07"),
            ("9223372036854775808", "9,223,372,036,854,775.80"),
        ];

        for case in tests {
            assert_eq!(&convert_digit_string_to_money(case.0), case.1);
        }
    }

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
