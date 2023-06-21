use std::rc::Rc;

use chrono::{
    DateTime,
    Datelike,
    TimeZone,
    Utc,
};
use yew::{
    function_component,
    html,
    use_state_eq,
    AttrValue,
    Html,
    Properties,
};

use crate::{
    components::Select,
    types::{
        datetime::{
            DateTimeRange,
            Month,
        },
        select::SelectOption,
    },
};

#[derive(Properties, PartialEq)]
pub(crate) struct DateTimeSelectProps {
    id: AttrValue,
    label: AttrValue,
    datetime_range: Rc<DateTimeRange<Utc>>,
}

#[function_component(DateTimeSelect)]
pub(crate) fn date_select(props: &DateTimeSelectProps) -> Html {
    let selected_year = use_state_eq(|| Utc::now().year());
    let selected_month = use_state_eq(|| Month::from_u32(Utc::now().month()));
    let selected_day = use_state_eq(|| Utc::now().day());

    let years = props.datetime_range.list_years();
    if years.contains(&*selected_year) != true && years.len() > 0 {
        selected_year.set(years[years.len() - 1]);
    }
    let years: Vec<SelectOption> = years
        .iter()
        .map(|v| SelectOption::from(*v).selected(v == &*selected_year))
        .collect();

    let months = props.datetime_range.list_months_for_year(*selected_year);
    if months.contains(&*selected_month) != true && months.len() > 0 {
        selected_month.set(months[months.len() - 1]);
    }
    let months: Vec<SelectOption> = months
        .iter()
        .map(|v| {
            let mut opt = SelectOption::from(*v);
            opt.selected = v == &*selected_month;
            opt
        })
        .collect();

    let days = props
        .datetime_range
        .list_days_for_year_and_month(*selected_year, *selected_month);
    if days.contains(&*selected_day) != true && days.len() > 0 {
        selected_day.set(days[days.len() - 1]);
    }
    let days: Vec<SelectOption> = days
        .iter()
        .map(|v| {
            let mut opt = SelectOption::from(*v);
            opt.selected = v == &*selected_day;
            opt
        })
        .collect();

    html! {
        <section id={format!("{}_datetime_select", props.id)}>
            <Select
                id={format!("{}_year", props.id)}
                label={"Year"}
                options={Rc::from(years)}
            />
            <Select
                id={format!("{}_month", props.id)}
                label={"Month"}
                options={Rc::from(months)}
            />
            <Select
                id={format!("{}_day", props.id)}
                label={"Day"}
                options={Rc::from(days)}
            />
        </section>
    }
}

#[cfg(test)]
mod test {
    use std::{
        rc::Rc,
        time::Duration,
    };

    use chrono::{
        Datelike,
        TimeZone,
        Utc,
    };
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use web_sys::HtmlSelectElement;
    use yew::AttrValue;

    use super::{
        DateTimeSelect,
        DateTimeSelectProps,
    };
    use crate::{
        dom::DOM,
        types::datetime::DateTimeRange,
    };

    wasm_bindgen_test_configure!(run_in_browser);

    async fn render_datetime_select(props: DateTimeSelectProps) {
        let output = DOM::get_element_by_id("output")
            .expect("Element `output` to exist");
        yew::Renderer::<DateTimeSelect>::with_root_and_props(output, props)
            .render();
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    fn datetime_select_props_with_id(id: &str) -> DateTimeSelectProps {
        let date1 = Utc.with_ymd_and_hms(2023, 6, 21, 12, 0, 0).unwrap();
        let date2 = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
        DateTimeSelectProps {
            id: AttrValue::from(id.to_owned()),
            label: AttrValue::from(""),
            datetime_range: Rc::new(DateTimeRange::from(date1, date2)),
        }
    }

    #[wasm_bindgen_test]
    async fn datetime_select_is_section_with_expected_id() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let element =
            DOM::get_section_by_id(&format!("{}_datetime_select", id));

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_year_select() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let year = DOM::get_select_by_id(&format!("{}_year_select_field", id));

        assert!(year.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_year_select_label() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let year_label =
            DOM::get_label_by_for(&format!("{}_year_select_field", id));

        assert!(year_label.is_some());
    }

    #[wasm_bindgen_test]
    async fn year_select_label_has_expected_inner_html() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let year_label =
            DOM::get_label_by_for(&format!("{}_year_select_field", id))
                .expect("Element to exist");

        assert_eq!(year_label.inner_html(), "Year");
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_month_select() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let month =
            DOM::get_select_by_id(&format!("{}_month_select_field", id));

        assert!(month.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_month_select_label() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let month_label =
            DOM::get_label_by_for(&format!("{}_month_select_field", id));

        assert!(month_label.is_some());
    }

    #[wasm_bindgen_test]
    async fn month_select_label_has_expected_inner_html() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let month_label =
            DOM::get_label_by_for(&format!("{}_month_select_field", id))
                .expect("Element to exist");

        assert_eq!(month_label.inner_html(), "Month");
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_day_select() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let day = DOM::get_select_by_id(&format!("{}_day_select_field", id));

        assert!(day.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_day_select_label() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let day_label =
            DOM::get_label_by_for(&format!("{}_day_select_field", id));

        assert!(day_label.is_some());
    }

    #[wasm_bindgen_test]
    async fn day_select_label_has_expected_inner_html() {
        let id = "test";
        let props = datetime_select_props_with_id(id);
        render_datetime_select(props).await;

        let day_label =
            DOM::get_label_by_for(&format!("{}_day_select_field", id))
                .expect("Element to exist");

        assert_eq!(day_label.inner_html(), "Day");
    }

    #[wasm_bindgen_test]
    async fn year_select_field_has_options_with_given_range() {
        let id = "test";
        let start_date = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2005, 1, 1, 0, 0, 0).unwrap();
        let mut props = datetime_select_props_with_id(id);
        props.datetime_range =
            Rc::new(DateTimeRange::from(start_date, end_date));
        render_datetime_select(props).await;

        let select =
            DOM::get_select_by_id(&format!("{}_year_select_field", id))
                .expect("Select Element to exist")
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

        let expected_options: Vec<String> = (start_date.year()
            ..=end_date.year())
            .map(|y| y.to_string())
            .collect();

        assert_eq!(select_options_values, expected_options);
    }

    #[wasm_bindgen_test]
    async fn month_select_field_has_options_with_given_range_for_year() {
        let id = "test";
        let start_date = Utc.with_ymd_and_hms(1990, 3, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(1990, 8, 1, 0, 0, 0).unwrap();
        let mut props = datetime_select_props_with_id(id);
        props.datetime_range =
            Rc::new(DateTimeRange::from(start_date, end_date));
        render_datetime_select(props).await;

        let select =
            DOM::get_select_by_id(&format!("{}_month_select_field", id))
                .expect("Select Element to exist")
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

        let expected_options: Vec<String> = (start_date.month()
            ..=end_date.month())
            .map(|m| m.to_string())
            .collect();

        assert_eq!(select_options_values, expected_options);
    }

    #[wasm_bindgen_test]
    async fn day_select_field_has_options_with_given_range_for_year_and_month()
    {
        let id = "test";
        let start_date = Utc.with_ymd_and_hms(1990, 8, 2, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(1990, 8, 16, 0, 0, 0).unwrap();
        let mut props = datetime_select_props_with_id(id);
        props.datetime_range =
            Rc::new(DateTimeRange::from(start_date, end_date));
        render_datetime_select(props).await;

        let select = DOM::get_select_by_id(&format!("{}_day_select_field", id))
            .expect("Select Element to exist")
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

        let expected_options: Vec<String> = (start_date.day()..=end_date.day())
            .map(|d| d.to_string())
            .collect();

        assert_eq!(select_options_values, expected_options);
    }
}
