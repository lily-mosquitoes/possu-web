use yew::AttrValue;

use crate::types::datetime::{
    Day,
    Month,
    Year,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct SelectOption {
    pub(crate) value: AttrValue,
    pub(crate) inner_html: AttrValue,
    pub(crate) selected: bool,
    pub(crate) disabled: bool,
}

impl SelectOption {
    pub(crate) fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub(crate) fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl From<&str> for SelectOption {
    fn from(value: &str) -> Self {
        Self {
            value: AttrValue::from(value.to_owned()),
            inner_html: AttrValue::from(value.to_owned()),
            ..Default::default()
        }
    }
}

impl From<&String> for SelectOption {
    fn from(value: &String) -> Self {
        Self {
            value: AttrValue::from(value.to_owned()),
            inner_html: AttrValue::from(value.to_owned()),
            ..Default::default()
        }
    }
}

impl From<Year> for SelectOption {
    fn from(value: Year) -> Self {
        Self {
            value: AttrValue::from(value.to_string()),
            inner_html: AttrValue::from(value.to_string()),
            ..Default::default()
        }
    }
}

impl From<Month> for SelectOption {
    fn from(value: Month) -> Self {
        Self {
            value: AttrValue::from((value as u32).to_string()),
            inner_html: AttrValue::from(value.to_string()),
            ..Default::default()
        }
    }
}

impl From<Day> for SelectOption {
    fn from(value: Day) -> Self {
        Self {
            value: AttrValue::from(value.to_string()),
            inner_html: AttrValue::from(value.to_string()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use yew::AttrValue;

    use super::SelectOption;
    use crate::types::datetime::{
        Day,
        Month,
        Year,
    };

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn associated_function_selected_returns_struct_with_selected_set_as_expected(
    ) {
        for expected in [true, false] {
            let test = SelectOption::default().selected(expected);
            assert_eq!(test.selected, expected);
        }
    }

    #[wasm_bindgen_test]
    fn associated_function_disabled_returns_struct_with_disabled_set_as_expected(
    ) {
        for expected in [true, false] {
            let test = SelectOption::default().disabled(expected);
            assert_eq!(test.disabled, expected);
        }
    }

    #[wasm_bindgen_test]
    fn constructor_from_exists_for_str_borrow() {
        let input: &str = "test";
        let expected = SelectOption {
            value: AttrValue::from(input),
            inner_html: AttrValue::from(input),
            ..Default::default()
        };
        let test = SelectOption::from(input);
        assert_eq!(test, expected);
    }

    #[wasm_bindgen_test]
    fn constructor_from_exists_for_string_borrow() {
        let input: String = String::from("test");
        let expected = SelectOption {
            value: AttrValue::from(input.to_owned()),
            inner_html: AttrValue::from(input.to_owned()),
            ..Default::default()
        };
        let test = SelectOption::from(&input);
        assert_eq!(test, expected);
    }

    #[wasm_bindgen_test]
    fn constructor_from_exists_for_year() {
        let input: Year = 1990;
        let expected = SelectOption {
            value: AttrValue::from(input.to_string()),
            inner_html: AttrValue::from(input.to_string()),
            ..Default::default()
        };
        let test = SelectOption::from(input);
        assert_eq!(test, expected);
    }

    #[wasm_bindgen_test]
    fn constructor_from_exists_for_month() {
        let input: Month = Month::November;
        let expected = SelectOption {
            value: AttrValue::from((input as u32).to_string()),
            inner_html: AttrValue::from(input.to_string()),
            ..Default::default()
        };
        let test = SelectOption::from(input);
        assert_eq!(test, expected);
    }

    #[wasm_bindgen_test]
    fn constructor_from_exists_for_day() {
        let input: Day = 12;
        let expected = SelectOption {
            value: AttrValue::from(input.to_string()),
            inner_html: AttrValue::from(input.to_string()),
            ..Default::default()
        };
        let test = SelectOption::from(input);
        assert_eq!(test, expected);
    }
}
