use std::{
    ops::Deref,
    rc::Rc,
};

use chrono::{
    DateTime,
    Datelike,
    FixedOffset,
};
use web_sys::{
    Event,
    HtmlSelectElement,
};
use yew::{
    function_component,
    html,
    use_effect_with_deps,
    use_state_eq,
    AttrValue,
    Callback,
    Html,
    Properties,
    TargetCast,
};

use crate::{
    components::Select,
    types::{
        datetime::{
            DateTimeRange,
            Day,
            Month,
            Year,
        },
        select::SelectOption,
    },
};

#[derive(Properties, PartialEq)]
pub(crate) struct DateTimeSelectProps {
    pub(crate) id: AttrValue,
    pub(crate) label: AttrValue,
    pub(crate) range: Rc<DateTimeRange<FixedOffset>>,
    pub(crate) preselect: Rc<DateTime<FixedOffset>>,
    #[prop_or_default]
    pub(crate) ondatetimechange:
        Option<Callback<Option<DateTime<FixedOffset>>>>,
}

#[function_component(DateTimeSelect)]
pub(crate) fn datetime_select(props: &DateTimeSelectProps) -> Html {
    // YEAR
    let selected_year = use_state_eq(|| {
        props.range.get_year_or_last(Some(props.preselect.year()))
    });
    let years: Vec<SelectOption> = props
        .range
        .list_years()
        .iter()
        .map(|&v| SelectOption::from(v).selected(Some(v) == *selected_year))
        .collect();

    // IMPL MONTHOFYEAR
    #[derive(PartialEq)]
    struct MonthOfYear {
        month: Option<Month>,
        year: Option<Year>,
    }
    impl Deref for MonthOfYear {
        type Target = Option<Month>;

        fn deref(&self) -> &Self::Target {
            &self.month
        }
    }
    impl From<(Option<Month>, Option<Year>)> for MonthOfYear {
        fn from((month, year): (Option<Month>, Option<Year>)) -> Self {
            Self { month, year }
        }
    }

    // MONTH
    let selected_month = use_state_eq(|| {
        let month = Some(Month::from_u32(props.preselect.month()));
        let month = props
            .range
            .get_month_or_last_for_year(month, *selected_year);
        MonthOfYear::from((month, *selected_year))
    });
    {
        let range = props.range.clone();
        let selected_month = selected_month.clone();
        use_effect_with_deps(
            move |selected_year| {
                let month = range.get_month_or_last_for_year(
                    **selected_month,
                    **selected_year,
                );
                selected_month.set(MonthOfYear::from((month, **selected_year)));
            },
            selected_year.clone(),
        );
    }
    let months: Vec<SelectOption> = match *selected_year {
        Some(year) => props
            .range
            .list_months_for_year(year)
            .iter()
            .map(|&v| {
                SelectOption::from(v).selected(Some(v) == **selected_month)
            })
            .collect(),
        None => Vec::default(),
    };

    // DAY
    let selected_day = use_state_eq(|| {
        props.range.get_day_or_last_for_month_and_year(
            Some(props.preselect.day()),
            **selected_month,
            *selected_year,
        )
    });
    {
        let range = props.range.clone();
        let selected_year = selected_year.clone();
        let selected_day = selected_day.clone();
        use_effect_with_deps(
            move |selected_month| {
                let day = range.get_day_or_last_for_month_and_year(
                    *selected_day,
                    ***selected_month,
                    *selected_year,
                );
                selected_day.set(day);
            },
            selected_month.clone(),
        );
    }
    let days: Vec<SelectOption> = match (*selected_year, **selected_month) {
        (Some(year), Some(month)) => props
            .range
            .list_days_for_year_and_month(year, month)
            .iter()
            .map(|&v| SelectOption::from(v).selected(Some(v) == *selected_day))
            .collect(),
        _ => Vec::default(),
    };

    // ONCHANGE
    let onchange = |e: Event| -> Option<i32> {
        e.target_dyn_into::<HtmlSelectElement>().and_then(|select| {
            let index = select.selected_index();
            select.get(index as u32).and_then(|selected| {
                selected
                    .get_attribute("value")
                    .and_then(|value| value.parse::<i32>().ok())
            })
        })
    };
    let onchange_year = {
        let selected_year = selected_year.clone();
        Callback::from(move |e: Event| {
            if let Some(value) = onchange(e) {
                selected_year.set(Some(value));
            }
        })
    };
    let onchange_month = {
        let selected_month = selected_month.clone();
        let selected_year = selected_year.clone();
        Callback::from(move |e: Event| {
            if let Some(value) = onchange(e) {
                let month = Some(Month::from_u32(value as u32));
                selected_month.set(MonthOfYear::from((month, *selected_year)));
            }
        })
    };
    let onchange_day = {
        let selected_day = selected_day.clone();
        Callback::from(move |e: Event| {
            if let Some(value) = onchange(e) {
                selected_day.set(Some(value as u32));
            }
        })
    };
    let report_change = {
        let preselect = props.preselect.clone();
        let ondatetimechange = props.ondatetimechange.clone();
        move |year: Option<Year>, month: Option<Month>, day: Option<Day>| {
            let selected_date = match (year, month, day) {
                (Some(year), Some(month), Some(day)) => {
                    preselect.with_year(year).and_then(|date| {
                        date.with_month(month as u32)
                            .and_then(|date| date.with_day(day))
                    })
                },
                _ => None,
            };

            if let Some(event) = ondatetimechange {
                event.emit(selected_date);
            }
        }
    };
    use_effect_with_deps(
        |(year, month, day)| report_change(**year, ***month, **day),
        (selected_year.clone(), selected_month.clone(), selected_day.clone()),
    );

    html! {
        <section id={props.id.clone()}>
            <Select
                id={format!("{}_year", props.id)}
                label={"Year"}
                options={Rc::from(years)}
                onchange={onchange_year}
            />
            <Select
                id={format!("{}_month", props.id)}
                label={"Month"}
                options={Rc::from(months)}
                onchange={onchange_month}
            />
            <Select
                id={format!("{}_day", props.id)}
                label={"Day"}
                options={Rc::from(days)}
                onchange={onchange_day}
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
    use web_sys::{
        Event,
        HtmlSelectElement,
    };
    use yew::{
        AttrValue,
        Callback,
    };

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
            range: Rc::new(DateTimeRange::from(date1, date2)),
            preselect: Rc::new(make_now()),
            ondatetimechange: None,
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

    async fn dispatch_change_event(target: &HtmlSelectElement) {
        let event = Event::new("change").expect("valid event");
        target
            .dispatch_event(&event)
            .expect("event to be dispatched");
        yew::platform::time::sleep(Duration::from_millis(10)).await;
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

    static TEST_ID: &str = "test_datetime_select";
    static TEST_YEAR_SELECT_ID: &str = "test_datetime_select_year";
    static TEST_MONTH_SELECT_ID: &str = "test_datetime_select_month";
    static TEST_DAY_SELECT_ID: &str = "test_datetime_select_day";

    #[wasm_bindgen_test]
    async fn component_contains_section_element_with_expected_id() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_section_by_id(TEST_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn component_contains_select_element_for_year() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_select_by_id(TEST_YEAR_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn component_contains_select_label_element_for_year() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_label_by_for(TEST_YEAR_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn year_select_label_element_has_expected_inner_html() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_label_by_for(TEST_YEAR_SELECT_ID)
            .expect("Label Element to exist");

        assert_eq!(element.inner_html(), "Year");
    }

    #[wasm_bindgen_test]
    async fn component_contains_select_element_for_month() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_select_by_id(TEST_MONTH_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn component_contains_select_label_element_for_month() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_label_by_for(TEST_MONTH_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn month_select_label_element_has_expected_inner_html() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_label_by_for(TEST_MONTH_SELECT_ID)
            .expect("Label Element to exist");

        assert_eq!(element.inner_html(), "Month");
    }

    #[wasm_bindgen_test]
    async fn component_contains_select_element_for_day() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_select_by_id(TEST_DAY_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn component_contains_select_label_element_for_day() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_label_by_for(TEST_DAY_SELECT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn day_select_label_element_has_expected_inner_html() {
        let props = datetime_select_props_with_id(TEST_ID);
        render_datetime_select(props).await;

        let element = DOM::get_label_by_for(TEST_DAY_SELECT_ID)
            .expect("Label Element to exist");

        assert_eq!(element.inner_html(), "Day");
    }

    #[wasm_bindgen_test]
    async fn year_select_element_has_options_with_given_range() {
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
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_YEAR_SELECT_ID)
                .expect("Select to exist");

            let select_options_values = collect_options_values_from(select);

            let expected_options: Vec<String> =
                expected_options.map(|v| v.to_string()).collect();

            assert_eq!(select_options_values, expected_options);
        }
    }

    #[wasm_bindgen_test]
    async fn year_select_element_has_preselect_or_last_option_pre_selected() {
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
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_YEAR_SELECT_ID)
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
    async fn month_select_element_has_options_with_given_range_for_year() {
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
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_MONTH_SELECT_ID)
                .expect("Select to exist");

            let select_options_values = collect_options_values_from(select);

            let expected_options: Vec<String> =
                expected_options.map(|v| v.to_string()).collect();

            assert_eq!(select_options_values, expected_options);
        }
    }

    #[wasm_bindgen_test]
    async fn month_select_element_has_preselect_or_last_option_pre_selected() {
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
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_MONTH_SELECT_ID)
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
    async fn day_select_element_has_options_with_given_range_for_year_and_month(
    ) {
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
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_DAY_SELECT_ID)
                .expect("Select to exist");

            let select_options_values = collect_options_values_from(select);

            let expected_options: Vec<String> =
                expected_options.map(|v| v.to_string()).collect();

            assert_eq!(select_options_values, expected_options);
        }
    }

    #[wasm_bindgen_test]
    async fn day_select_element_has_preselect_or_last_option_pre_selected() {
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
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let select = DOM::get_html_select_by_id(TEST_DAY_SELECT_ID)
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
    async fn when_selected_year_changes_recalculate_months_and_days() {
        let year_index = |y: i32| -> i32 {
            (1999..=2001).position(|e| e == y).unwrap() as i32
        };
        let month_index =
            |m: u32| -> i32 { (1..=12).position(|e| e == m).unwrap() as i32 };
        let day_index =
            |d: u32| -> i32 { (1..=31).position(|e| e == d).unwrap() as i32 };

        let tests = vec![
            // change from Feb with 29 days to Feb with 28 days
            (
                make_date(1999, 1, 1),
                make_date(2001, 12, 31),
                make_date(2000, 2, 29),
                1999,
                (2, 28),
            ),
            // change from full year to partial year
            (
                make_date(1999, 1, 1),
                make_date(2001, 4, 10),
                make_date(2000, 8, 3),
                2001,
                (4, 3),
            ),
            // change from full year to partial year and partial month
            (
                make_date(1999, 1, 1),
                make_date(2001, 4, 10),
                make_date(2000, 8, 12),
                2001,
                (4, 10),
            ),
        ];

        for (date1, date2, preselect, change_year_to, expected_selected) in
            tests
        {
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let year_select = DOM::get_html_select_by_id(TEST_YEAR_SELECT_ID)
                .expect("Select to exist");
            year_select.set_selected_index(year_index(change_year_to));
            dispatch_change_event(&year_select).await;

            let month_select = DOM::get_html_select_by_id(TEST_MONTH_SELECT_ID)
                .expect("Select to exist");
            let selected_month = month_select.selected_index();
            let day_select = DOM::get_html_select_by_id(TEST_DAY_SELECT_ID)
                .expect("Select to exist");
            let selected_day = day_select.selected_index();

            let expected_selected = (
                month_index(expected_selected.0),
                day_index(expected_selected.1),
            );

            assert_eq!((selected_month, selected_day), expected_selected);
        }
    }

    #[wasm_bindgen_test]
    async fn when_selected_month_changes_recalculate_days() {
        let year_index = |y: i32| -> i32 {
            (1999..=2001).position(|e| e == y).unwrap() as i32
        };
        let month_index =
            |m: u32| -> i32 { (1..=12).position(|e| e == m).unwrap() as i32 };
        let day_index =
            |d: u32| -> i32 { (1..=31).position(|e| e == d).unwrap() as i32 };

        let tests = vec![
            // change from month with 31 days to month with 30 days
            (
                make_date(1999, 1, 1),
                make_date(2001, 12, 31),
                make_date(2000, 5, 31),
                4,
                (2000, 30),
            ),
            // change from full month to full month
            (
                make_date(1999, 1, 1),
                make_date(2001, 5, 15),
                make_date(2000, 4, 3),
                5,
                (2000, 3),
            ),
            // change from full month to partial month
            (
                make_date(1999, 1, 1),
                make_date(2001, 5, 15),
                make_date(2001, 4, 20),
                5,
                (2001, 15),
            ),
        ];

        for (date1, date2, preselect, change_month_to, expected_selected) in
            tests
        {
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let month_select = DOM::get_html_select_by_id(TEST_MONTH_SELECT_ID)
                .expect("Select to exist");
            month_select.set_selected_index(month_index(change_month_to));
            dispatch_change_event(&month_select).await;

            let year_select = DOM::get_html_select_by_id(TEST_YEAR_SELECT_ID)
                .expect("Select to exist");
            let selected_year = year_select.selected_index();
            let day_select = DOM::get_html_select_by_id(TEST_DAY_SELECT_ID)
                .expect("Select to exist");
            let selected_day = day_select.selected_index();

            let expected_selected = (
                year_index(expected_selected.0),
                day_index(expected_selected.1),
            );

            assert_eq!((selected_year, selected_day), expected_selected);
        }
    }

    #[wasm_bindgen_test]
    async fn when_selected_day_changes_recalculate_nothing() {
        let year_index = |y: i32| -> i32 {
            (1999..=2001).position(|e| e == y).unwrap() as i32
        };
        let month_index =
            |m: u32| -> i32 { (1..=12).position(|e| e == m).unwrap() as i32 };
        let day_index =
            |d: u32| -> i32 { (1..=31).position(|e| e == d).unwrap() as i32 };

        let tests = vec![
            (
                make_date(1999, 1, 1),
                make_date(2001, 12, 31),
                make_date(2000, 5, 2),
                28,
                (2000, 5),
            ),
            (
                make_date(1999, 1, 1),
                make_date(2001, 5, 15),
                make_date(2000, 4, 30),
                3,
                (2000, 4),
            ),
        ];

        for (date1, date2, preselect, change_day_to, expected_selected) in tests
        {
            let mut props = datetime_select_props_with_id(TEST_ID);
            props.range = Rc::new(DateTimeRange::from(date1, date2));
            props.preselect = Rc::new(preselect);
            render_datetime_select(props).await;

            let day_select = DOM::get_html_select_by_id(TEST_DAY_SELECT_ID)
                .expect("Select to exist");
            day_select.set_selected_index(day_index(change_day_to));
            dispatch_change_event(&day_select).await;

            let year_select = DOM::get_html_select_by_id(TEST_YEAR_SELECT_ID)
                .expect("Select to exist");
            let selected_year = year_select.selected_index();
            let month_select = DOM::get_html_select_by_id(TEST_MONTH_SELECT_ID)
                .expect("Select to exist");
            let selected_month = month_select.selected_index();

            let expected_selected = (
                year_index(expected_selected.0),
                month_index(expected_selected.1),
            );

            assert_eq!((selected_year, selected_month), expected_selected);
        }
    }

    #[wasm_bindgen_test]
    async fn ondatetimechange_receives_selected_datetime() {
        let (date1, date2) = (make_date(1999, 1, 1), make_date(2003, 4, 8));
        let preselect = make_date(1999, 1, 1);
        let test_ondatetimechange =
            Callback::from(|date: Option<DateTime<FixedOffset>>| {
                if let Some(date) = date {
                    let test_div = DOM::get_test_div();
                    test_div.set_inner_html(&date.to_rfc2822());
                }
            });

        let year_index = |y: i32| -> i32 {
            (1999..=2003).position(|e| e == y).unwrap() as i32
        };
        let month_index =
            |m: u32| -> i32 { (1..=12).position(|e| e == m).unwrap() as i32 };
        let day_index =
            |d: u32| -> i32 { (1..=31).position(|e| e == d).unwrap() as i32 };

        let mut props = datetime_select_props_with_id(TEST_ID);
        props.range = Rc::new(DateTimeRange::from(date1, date2));
        props.preselect = Rc::new(preselect);
        props.ondatetimechange = Some(test_ondatetimechange);
        render_datetime_select(props).await;

        let year_select = DOM::get_html_select_by_id(TEST_YEAR_SELECT_ID)
            .expect("Select to exist");
        let month_select = DOM::get_html_select_by_id(TEST_MONTH_SELECT_ID)
            .expect("Select to exist");
        let day_select = DOM::get_html_select_by_id(TEST_DAY_SELECT_ID)
            .expect("Select to exist");

        enum ChangeOption {
            Year(i32),
            Month(u32),
            Day(u32),
        }

        let tests = vec![
            (ChangeOption::Year(2000), make_date(2000, 1, 1)),
            (ChangeOption::Month(2), make_date(2000, 2, 1)),
            (ChangeOption::Day(29), make_date(2000, 2, 29)),
            (ChangeOption::Year(1999), make_date(1999, 2, 28)),
            (ChangeOption::Month(6), make_date(1999, 6, 28)),
            (ChangeOption::Year(2003), make_date(2003, 4, 8)),
        ];

        for (to_change, expected_selected) in tests {
            match to_change {
                ChangeOption::Year(year) => {
                    year_select.set_selected_index(year_index(year));
                    dispatch_change_event(&year_select).await;
                },
                ChangeOption::Month(month) => {
                    month_select.set_selected_index(month_index(month));
                    dispatch_change_event(&month_select).await;
                },
                ChangeOption::Day(day) => {
                    day_select.set_selected_index(day_index(day));
                    dispatch_change_event(&day_select).await;
                },
            }

            let onchange_output = DOM::get_test_div().inner_html();

            assert_eq!(onchange_output, expected_selected.to_rfc2822());
        }
    }
}
