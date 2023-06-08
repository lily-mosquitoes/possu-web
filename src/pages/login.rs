use yew::{
    function_component,
    html,
    Html,
};

use crate::components::Input;

#[function_component(Login)]
pub(crate) fn login() -> Html {
    html! {
        <section id={"login"}>
            <Input
                id={"username"}
                label={"Username"}
                input_type={"text"}
            />
            <Input
                id={"password"}
                label={"Password"}
                input_type={"password"}
            />
            <button
                id={"login_button"}
                type={"submit"}
            >
                { "Login" }
            </button>
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

    // USERNAME INPUT TESTS
    #[wasm_bindgen_test]
    async fn username_input_field_with_label_exists() {
        render_login().await;

        let field = DOM::get_input_by_id("username_input_field");
        let label = DOM::get_label_by_for("username_input_field");

        assert!(field.is_some() && label.is_some());
    }

    #[wasm_bindgen_test]
    async fn username_input_field_and_label_are_visible() {
        render_login().await;

        let field = DOM::get_input_by_id("username_input_field")
            .expect("username input field to exist");
        let label = DOM::get_label_by_for("username_input_field")
            .expect("username input field label to exist");

        assert!(
            DOM::is_element_visible(&field)
                && DOM::is_element_visible(&label)
        );
    }

    #[wasm_bindgen_test]
    async fn username_input_field_label_has_expected_inner_html() {
        render_login().await;

        let label = DOM::get_label_by_for("username_input_field")
            .expect("username input field label to exist");

        assert_eq!(&label.inner_html(), "Username");
    }

    #[wasm_bindgen_test]
    async fn username_input_field_type_is_text() {
        render_login().await;

        let field = DOM::get_input_by_id("username_input_field")
            .expect("username input field to exist");
        let input_type = field.get_attribute("type");

        assert_eq!(input_type, Some("text".to_string()));
    }

    // USERNAME INPUT TESTS
    #[wasm_bindgen_test]
    async fn password_input_field_with_label_exists() {
        render_login().await;

        let field = DOM::get_input_by_id("password_input_field");
        let label = DOM::get_label_by_for("password_input_field");

        assert!(field.is_some() && label.is_some());
    }

    #[wasm_bindgen_test]
    async fn password_input_field_and_label_are_visible() {
        render_login().await;

        let field = DOM::get_input_by_id("password_input_field")
            .expect("password input field to exist");
        let label = DOM::get_label_by_for("password_input_field")
            .expect("password input field label to exist");

        assert!(
            DOM::is_element_visible(&field)
                && DOM::is_element_visible(&label)
        );
    }

    #[wasm_bindgen_test]
    async fn password_input_field_label_has_expected_inner_html() {
        render_login().await;

        let label = DOM::get_label_by_for("password_input_field")
            .expect("username input field label to exist");

        assert_eq!(&label.inner_html(), "Password");
    }

    #[wasm_bindgen_test]
    async fn password_input_field_type_is_password() {
        render_login().await;

        let field = DOM::get_input_by_id("password_input_field")
            .expect("password input field to exist");
        let input_type = field.get_attribute("type");

        assert_eq!(input_type, Some("password".to_string()));
    }

    // LOGIN BUTTON TESTS
    #[wasm_bindgen_test]
    async fn login_button_exists() {
        render_login().await;

        let button = DOM::get_button_by_id("login_button");

        assert!(button.is_some());
    }

    #[wasm_bindgen_test]
    async fn login_button_is_visible() {
        render_login().await;

        let button = DOM::get_button_by_id("login_button")
            .expect("login button to exist");

        assert!(DOM::is_element_visible(&button));
    }

    #[wasm_bindgen_test]
    async fn login_button_type_is_submit() {
        render_login().await;

        let button = DOM::get_button_by_id("login_button")
            .expect("login button to exist");
        let button_type = button.get_attribute("type");

        assert_eq!(button_type, Some("submit".to_string()));
    }

    #[wasm_bindgen_test]
    async fn login_button_has_expected_inner_html() {
        render_login().await;

        let button = DOM::get_button_by_id("login_button")
            .expect("login button to exist");

        assert_eq!(&button.inner_html(), "Login");
    }
}
