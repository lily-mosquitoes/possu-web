use std::rc::Rc;

use chrono::{
    DateTime,
    Datelike,
    FixedOffset,
    Local,
};
use yew::{
    function_component,
    html,
    use_effect_with_deps,
    use_state_eq,
    AttrValue,
    Html,
    Properties,
    UseStateHandle,
};

use crate::{
    components::Select,
    types::{
        datetime::{
            DateTimeRange,
            Month,
            Year,
        },
        select::SelectOption,
    },
};

#[derive(Properties, PartialEq)]
pub(crate) struct DateTimeSelectProps {
    id: AttrValue,
    label: AttrValue,
    datetime_range: Rc<DateTimeRange<FixedOffset>>,
    #[prop_or(Rc::new(Local::now().into()))]
    preselect: Rc<DateTime<FixedOffset>>,
}

#[function_component(DateTimeSelect)]
pub(crate) fn date_select(props: &DateTimeSelectProps) -> Html {
    let selected_year = use_state_eq(|| props.preselect.year());
    let years = use_state_eq(|| Vec::<SelectOption>::default());
    let set_years = {
        let selected_year = selected_year.clone();
        let years = years.clone();
        let list = props.datetime_range.list_years();
        move |_: &()| {
            if list.contains(&*selected_year) != true && list.len() > 0 {
                selected_year.set(list[list.len() - 1]);
            }
            let options: Vec<SelectOption> = list
                .iter()
                .map(|v| SelectOption::from(*v).selected(v == &*selected_year))
                .collect();
            years.set(options);
        }
    };
    use_effect_with_deps(set_years, ());

    let selected_month =
        use_state_eq(|| Month::from_u32(props.preselect.month()));
    let months = use_state_eq(|| Vec::<SelectOption>::default());
    let set_months = {
        let selected_month = selected_month.clone();
        let months = months.clone();
        let list = props.datetime_range.list_months_for_year(*selected_year);
        move |_: &UseStateHandle<Year>| {
            if list.contains(&*selected_month) != true && list.len() > 0 {
                selected_month.set(list[list.len() - 1]);
            }
            let options: Vec<SelectOption> = list
                .iter()
                .map(|v| SelectOption::from(*v).selected(v == &*selected_month))
                .collect();
            months.set(options);
        }
    };
    use_effect_with_deps(set_months, selected_year.clone());

    let selected_day = use_state_eq(|| props.preselect.day());
    let days = use_state_eq(|| Vec::<SelectOption>::default());
    let set_days = {
        let selected_day = selected_day.clone();
        let days = days.clone();
        let list = props
            .datetime_range
            .list_days_for_year_and_month(*selected_year, *selected_month);
        move |(_, _): &(UseStateHandle<Month>, UseStateHandle<Year>)| {
            if list.contains(&*selected_day) != true && list.len() > 0 {
                selected_day.set(list[list.len() - 1]);
            }
            let options: Vec<SelectOption> = list
                .iter()
                .map(|v| SelectOption::from(*v).selected(v == &*selected_day))
                .collect();
            days.set(options);
        }
    };
    use_effect_with_deps(
        set_days,
        (selected_month.clone(), selected_year.clone()),
    );

    html! {
        <section id={format!("{}_datetime_select", props.id)}>
            <Select
                id={format!("{}_year", props.id)}
                label={"Year"}
                options={Rc::from((*years).clone())}
            />
            <Select
                id={format!("{}_month", props.id)}
                label={"Month"}
                options={Rc::from((*months).clone())}
            />
            <Select
                id={format!("{}_day", props.id)}
                label={"Day"}
                options={Rc::from((*days).clone())}
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
        DateTime,
        Datelike,
        FixedOffset,
        Local,
        TimeZone,
    };
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
        let date1 = make_date(2023, 6, 21);
        let date2 = make_date(2024, 6, 21);
        DateTimeSelectProps {
            id: AttrValue::from(id.to_owned()),
            label: AttrValue::from(""),
            datetime_range: Rc::new(DateTimeRange::from(date1, date2)),
            preselect: Rc::new(make_now()),
        }
    }

    fn make_date(y: i32, m: u32, d: u32) -> DateTime<FixedOffset> {
        Local.with_ymd_and_hms(y, m, d, 12, 0, 0).unwrap().into()
    }

    fn make_now() -> DateTime<FixedOffset> {
        Local::now().into()
    }

    fn make_now_minus_days(d: i64) -> DateTime<FixedOffset> {
        make_now() - chrono::Duration::days(d)
    }

    fn make_now_plus_days(d: i64) -> DateTime<FixedOffset> {
        make_now() + chrono::Duration::days(d)
    }

    fn collect_options_values_from(select: HtmlSelectElement) -> Vec<String> {
        let mut options_values = vec![];
        for index in 0..select.length() {
            let option_element = select.get(index).expect("Element to exist");
            let value = option_element
                .get_attribute("value")
                .expect("value to exist");
            options_values.push(value);
        }
        options_values
    }

    static TEST_GIVEN_ID: &str = "test";
    static TEST_SECTION_ID: &str = "test_datetime_select";
    static TEST_YEAR_FIELD_ID: &str = "test_year_select_field";
    static TEST_MONTH_FIELD_ID: &str = "test_month_select_field";
    static TEST_DAY_FIELD_ID: &str = "test_day_select_field";

    #[wasm_bindgen_test]
    async fn datetime_select_is_section_with_expected_id() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let element = DOM::get_section_by_id(TEST_SECTION_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_year_select() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let year = DOM::get_select_by_id(TEST_YEAR_FIELD_ID);

        assert!(year.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_year_select_label() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let year_label = DOM::get_label_by_for(TEST_YEAR_FIELD_ID);

        assert!(year_label.is_some());
    }

    #[wasm_bindgen_test]
    async fn year_select_label_has_expected_inner_html() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let year_label = DOM::get_label_by_for(TEST_YEAR_FIELD_ID)
            .expect("Element to exist");

        assert_eq!(year_label.inner_html(), "Year");
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_month_select() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let month = DOM::get_select_by_id(TEST_MONTH_FIELD_ID);

        assert!(month.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_month_select_label() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let month_label = DOM::get_label_by_for(TEST_MONTH_FIELD_ID);

        assert!(month_label.is_some());
    }

    #[wasm_bindgen_test]
    async fn month_select_label_has_expected_inner_html() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let month_label = DOM::get_label_by_for(TEST_MONTH_FIELD_ID)
            .expect("Element to exist");

        assert_eq!(month_label.inner_html(), "Month");
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_day_select() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let day = DOM::get_select_by_id(TEST_DAY_FIELD_ID);

        assert!(day.is_some());
    }

    #[wasm_bindgen_test]
    async fn datetime_select_contains_day_select_label() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let day_label = DOM::get_label_by_for(TEST_DAY_FIELD_ID);

        assert!(day_label.is_some());
    }

    #[wasm_bindgen_test]
    async fn day_select_label_has_expected_inner_html() {
        let props = datetime_select_props_with_id(TEST_GIVEN_ID);
        render_datetime_select(props).await;

        let day_label =
            DOM::get_label_by_for(TEST_DAY_FIELD_ID).expect("Element to exist");

        assert_eq!(day_label.inner_html(), "Day");
    }

    #[wasm_bindgen_test]
    async fn year_select_field_has_options_with_given_range() {
        let tests = vec![
            // incorrect date order
            (make_date(2000, 1, 1), make_date(1999, 1, 1), 1..=0),
            // single year range is correct
            (make_date(1999, 1, 1), make_date(1999, 1, 1), 1999..=1999),
            // multi year range is correct
            (make_date(1999, 1, 1), make_date(2000, 1, 1), 1999..=2000),
            (make_date(1999, 1, 1), make_date(3000, 12, 1), 1999..=3000),
            (make_date(1999, 12, 31), make_date(3003, 8, 5), 1999..=3003),
        ];

        for (date1, date2, expected_options) in tests {
            let mut props = datetime_select_props_with_id(TEST_GIVEN_ID);
            props.datetime_range = Rc::new(DateTimeRange::from(date1, date2));
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_YEAR_FIELD_ID)
                .expect("Select to exist");

            let select_options_values = collect_options_values_from(select);

            let expected_options: Vec<String> =
                expected_options.map(|v| v.to_string()).collect();

            assert_eq!(select_options_values, expected_options);
        }
    }

    #[wasm_bindgen_test]
    async fn year_select_field_has_preselect_or_last_option_pre_selected() {
        let tests = vec![
            // preselect should be selected when available
            (
                make_now_minus_days(3650),
                make_now_plus_days(3650),
                make_now(),
                make_now().year(),
            ),
            // last available year should be selected otherwise
            (
                make_now_minus_days(3650),
                make_now_minus_days(1825),
                make_now(),
                make_now_minus_days(1825).year(),
            ),
        ];

        for (date1, date2, preselect, expected_option) in tests {
            let mut props = datetime_select_props_with_id(TEST_GIVEN_ID);
            props.datetime_range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_YEAR_FIELD_ID)
                .expect("Select to exist");

            let selected_option = select
                .get(select.selected_index() as u32)
                .expect("Option to exist")
                .get_attribute("value")
                .expect("value to exist");

            let expected_option = expected_option.to_string();

            assert_eq!(selected_option, expected_option);
        }
    }

    #[wasm_bindgen_test]
    async fn month_select_field_has_options_with_given_range_for_year() {
        let tests = vec![
            // incorrect date order
            (
                make_date(2000, 1, 1),
                make_date(1999, 1, 1),
                make_date(2000, 1, 1),
                1..=0,
            ),
            // single month range is correct
            (
                make_date(1999, 1, 1),
                make_date(1999, 1, 1),
                make_date(1999, 1, 1),
                1..=1,
            ),
            // multi month range is correct
            // partial year
            (
                make_date(1999, 4, 1),
                make_date(1999, 8, 1),
                make_date(1999, 4, 1),
                4..=8,
            ),
            // partial year to end
            (
                make_date(1999, 4, 1),
                make_date(2001, 8, 1),
                make_date(1999, 4, 1),
                4..=12,
            ),
            // full year
            (
                make_date(1999, 4, 1),
                make_date(2001, 8, 1),
                make_date(2000, 1, 1),
                1..=12,
            ),
            // partial year from start
            (
                make_date(1999, 4, 1),
                make_date(2001, 8, 1),
                make_date(2001, 1, 1),
                1..=8,
            ),
        ];

        for (date1, date2, preselect, expected_options) in tests {
            let mut props = datetime_select_props_with_id(TEST_GIVEN_ID);
            props.datetime_range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_MONTH_FIELD_ID)
                .expect("Select to exist");

            let select_options_values = collect_options_values_from(select);

            let expected_options: Vec<String> =
                expected_options.map(|v| v.to_string()).collect();

            assert_eq!(select_options_values, expected_options);
        }
    }

    #[wasm_bindgen_test]
    async fn month_select_field_has_preselect_or_last_option_pre_selected() {
        let tests = vec![
            // preselect should be selected when available
            (
                make_now_minus_days(90),
                make_now_plus_days(90),
                make_now(),
                make_now().month(),
            ),
            // last available month should be selected otherwise
            (
                make_now_minus_days(90),
                make_now_minus_days(60),
                make_now(),
                make_now_minus_days(60).month(),
            ),
        ];

        for (date1, date2, preselect, expected_option) in tests {
            let mut props = datetime_select_props_with_id(TEST_GIVEN_ID);
            props.datetime_range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_MONTH_FIELD_ID)
                .expect("Select to exist");

            let selected_option = select
                .get(select.selected_index() as u32)
                .expect("Option to exist")
                .get_attribute("value")
                .expect("value to exist");

            let expected_option = expected_option.to_string();

            assert_eq!(selected_option, expected_option);
        }
    }

    #[wasm_bindgen_test]
    async fn day_select_field_has_options_with_given_range_for_year_and_month()
    {
        let tests = vec![
            // incorrect date order
            (
                make_date(2000, 1, 1),
                make_date(1999, 1, 1),
                make_date(2000, 1, 1),
                1..=0,
            ),
            // single day range is correct
            (
                make_date(1999, 1, 4),
                make_date(1999, 1, 4),
                make_date(1999, 1, 4),
                4..=4,
            ),
            // multi day range is correct
            (
                make_date(1999, 1, 2),
                make_date(1999, 1, 20),
                make_date(1999, 1, 2),
                2..=20,
            ),
            // february with 28 days is correct
            (
                make_date(1999, 1, 1),
                make_date(2000, 12, 31),
                make_date(1999, 2, 1),
                1..=28,
            ),
            // february with 29 days is correct
            (
                make_date(2000, 1, 1),
                make_date(2001, 12, 31),
                make_date(2000, 2, 1),
                1..=29,
            ),
            // month with 30 days is correct
            (
                make_date(2000, 1, 1),
                make_date(2001, 12, 31),
                make_date(2000, 4, 1),
                1..=30,
            ),
            // month with 31 days is correct
            (
                make_date(2000, 1, 1),
                make_date(2001, 12, 31),
                make_date(2000, 5, 1),
                1..=31,
            ),
        ];

        for (date1, date2, preselect, expected_options) in tests {
            let mut props = datetime_select_props_with_id(TEST_GIVEN_ID);
            props.datetime_range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_DAY_FIELD_ID)
                .expect("Select to exist");

            let select_options_values = collect_options_values_from(select);

            let expected_options: Vec<String> =
                expected_options.map(|v| v.to_string()).collect();

            assert_eq!(select_options_values, expected_options);
        }
    }

    #[wasm_bindgen_test]
    async fn day_select_field_has_preselect_or_last_option_pre_selected() {
        let tests = vec![
            // preselect should be selected when available
            (
                make_now_minus_days(5),
                make_now_plus_days(5),
                make_now(),
                make_now().day(),
            ),
            // last available day should be selected otherwise
            (
                make_now_minus_days(10),
                make_now_minus_days(5),
                make_now(),
                make_now_minus_days(5).day(),
            ),
        ];

        for (date1, date2, preselect, expected_option) in tests {
            let mut props = datetime_select_props_with_id(TEST_GIVEN_ID);
            props.datetime_range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_DAY_FIELD_ID)
                .expect("Select to exist");

            let selected_option = select
                .get(select.selected_index() as u32)
                .expect("Option to exist")
                .get_attribute("value")
                .expect("value to exist");

            let expected_option = expected_option.to_string();

            assert_eq!(selected_option, expected_option);
        }
    }
}
