use yew::{
    function_component,
    html,
    Html,
};

use crate::components::{
    Input,
    InputType,
};

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <section id={"login_section"}>
            <Input
                id={"username_input"}
                label={"Username"}
            />
            <Input
                id={"password_input"}
                label={"Password"}
                input_type={InputType::Password}
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

    static USERNAME_INPUT_ID: &str = "username_input";
    static PASSWORD_INPUT_ID: &str = "password_input";
    static LOGIN_BUTTON_ID: &str = "login_button";

    // USERNAME INPUT TESTS
    #[wasm_bindgen_test]
    async fn page_contains_username_input_element() {
        render_login().await;

        let element = DOM::get_input_by_id(USERNAME_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn username_input_element_is_visible() {
        render_login().await;

        let element = DOM::get_input_by_id(USERNAME_INPUT_ID)
            .expect("Input Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn page_contains_label_for_username_input_element() {
        render_login().await;

        let element = DOM::get_label_by_for(USERNAME_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn username_input_label_element_is_visible() {
        render_login().await;

        let element = DOM::get_label_by_for(USERNAME_INPUT_ID)
            .expect("Label Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn username_input_label_element_has_expected_inner_html() {
        render_login().await;

        let element = DOM::get_label_by_for(USERNAME_INPUT_ID)
            .expect("Label Element to exist");

        assert_eq!(&element.inner_html(), "Username");
    }

    #[wasm_bindgen_test]
    async fn username_input_element_type_is_text() {
        render_login().await;

        let element = DOM::get_input_by_id(USERNAME_INPUT_ID)
            .expect("Input Element to exist");
        let input_type = element.get_attribute("type");

        assert_eq!(input_type, Some("text".to_string()));
    }

    // PASSWORD INPUT TESTS
    #[wasm_bindgen_test]
    async fn page_contains_password_input_element() {
        render_login().await;

        let element = DOM::get_input_by_id(PASSWORD_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn password_input_element_is_visible() {
        render_login().await;

        let element = DOM::get_input_by_id(PASSWORD_INPUT_ID)
            .expect("Input Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn page_contains_label_for_password_input_element() {
        render_login().await;

        let element = DOM::get_label_by_for(PASSWORD_INPUT_ID);

        assert!(element.is_some());
    }

    #[wasm_bindgen_test]
    async fn password_input_label_element_is_visible() {
        render_login().await;

        let element = DOM::get_label_by_for(PASSWORD_INPUT_ID)
            .expect("Label Element to exist");

        assert!(DOM::is_element_visible(&element));
    }

    #[wasm_bindgen_test]
    async fn password_input_label_element_has_expected_inner_html() {
        render_login().await;

        let element = DOM::get_label_by_for(PASSWORD_INPUT_ID)
            .expect("Label Element to exist");

        assert_eq!(&element.inner_html(), "Password");
    }

    #[wasm_bindgen_test]
    async fn password_input_element_type_is_password() {
        render_login().await;

        let element = DOM::get_input_by_id(PASSWORD_INPUT_ID)
            .expect("Input Element to exist");
        let input_type = element.get_attribute("type");

        assert_eq!(input_type, Some("password".to_string()));
    }

    // LOGIN BUTTON TESTS
    #[wasm_bindgen_test]
    async fn page_contains_login_button_element() {
        render_login().await;

        let button = DOM::get_button_by_id(LOGIN_BUTTON_ID);

        assert!(button.is_some());
    }

    #[wasm_bindgen_test]
    async fn login_button_element_is_visible() {
        render_login().await;

        let button = DOM::get_button_by_id(LOGIN_BUTTON_ID)
            .expect("Button Element to exist");

        assert!(DOM::is_element_visible(&button));
    }

    #[wasm_bindgen_test]
    async fn login_button_element_has_expected_inner_html() {
        render_login().await;

        let button = DOM::get_button_by_id(LOGIN_BUTTON_ID)
            .expect("Button Element to exist");

        assert_eq!(&button.inner_html(), "Login");
    }

    #[wasm_bindgen_test]
    async fn login_button_element_type_is_submit() {
        render_login().await;

        let button = DOM::get_button_by_id(LOGIN_BUTTON_ID)
            .expect("Button Element to exist");
        let button_type = button.get_attribute("type");

        assert_eq!(button_type, Some("submit".to_string()));
    }
}
