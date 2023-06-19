use possu_web::pages;
use yew::{
    function_component,
    html,
    Html,
};
use yew_router::{
    components::Redirect,
    router::BrowserRouter,
    switch::Switch,
    Routable,
};

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Login,
    #[at("/entries/new")]
    NewEntry,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <pages::Login /> },
        Route::NewEntry => html! { <pages::NewEntry /> },
        Route::NotFound => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
