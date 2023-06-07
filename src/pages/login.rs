use yew::{
    function_component,
    html,
    Html,
};

#[function_component(Login)]
pub(crate) fn login() -> Html {
    html! {
        <section id={"login"}>
            <input
                id={"username_input_field"}
                type={"text"}
            />
            <input
                id={"password_input_field"}
                type={"password"}
            />
        </section>
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::Login;
    use crate::dom::DOM;

    wasm_bindgen_test_configure!(run_in_browser);

    async fn render_login() {
        let output = DOM::get_element_by_id("output")
            .expect("Element `output` to exist");
        yew::Renderer::<Login>::with_root(output).render();
        yew::platform::time::sleep(Duration::from_millis(10)).await;
    }

    #[wasm_bindgen_test]
    async fn username_input_field_exists() {
        render_login().await;

        let element = DOM::get_element_by_id("username_input_field");

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn username_input_field_is_visible() {
        render_login().await;

        let element = DOM::get_element_by_id("username_input_field")
            .expect("Element `username_input_field` to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn username_input_field_is_text_input() {
        render_login().await;

        let element = DOM::get_element_by_id("username_input_field")
            .expect("Element `username_input_field` to exist");
        let tag = element.tag_name();
        let input_type = element.get_attribute("type");

        let is_text_input =
            &tag == "INPUT" && input_type == Some("text".to_string());

        assert!(is_text_input);
    }

    #[wasm_bindgen_test]
    async fn password_input_field_exists() {
        render_login().await;

        let element = DOM::get_element_by_id("password_input_field");

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn password_input_field_is_visible() {
        render_login().await;

        let element = DOM::get_element_by_id("password_input_field")
            .expect("Element `password_input_field` to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn password_input_field_is_text_input() {
        render_login().await;

        let element = DOM::get_element_by_id("password_input_field")
            .expect("Element `password_input_field` to exist");
        let tag = element.tag_name();
        let input_type = element.get_attribute("type");

        let is_password_input = &tag == "INPUT"
            && input_type == Some("password".to_string());

        assert!(is_password_input);
    }
}
