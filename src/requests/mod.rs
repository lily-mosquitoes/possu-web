pub(crate) async fn get_categories() -> Result<Vec<String>, String> {
    Ok(vec![String::from("Placeholder")])
}

pub(crate) struct Backend {
    pub(crate) url: &'static str,
}

impl From<&'static str> for Backend {
    fn from(value: &'static str) -> Self {
        Self { url: value }
    }
}

pub(crate) enum RequestResult<T> {
    Ok(T),
    Error(String),
    Unreachable,
}

pub(crate) async fn post_login(
    backend: &Backend,
    username: &str,
    password: &str,
) -> RequestResult<()> {
    RequestResult::Ok(())
}
