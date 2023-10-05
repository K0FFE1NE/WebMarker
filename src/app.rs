use crate::{components::sidebar::Sidebar, main};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// ? Probably put this in a separate file to avoid circular dependencies
#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

impl From<String> for Route {
    fn from(value: String) -> Self {
        match value.as_str() {
            "home" => Route::Home,
            _ => Route::NotFound,
        }
    }
}

fn switch(parent_ref: NodeRef, route: Route) -> Html {
    match route {
        Route::Home => html! {
            <div ref={parent_ref.clone()} class="h-full w-full">
                <Sidebar parent_ref={parent_ref} />
            </div>
        },
        Route::NotFound => html! { "404" }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let parent_ref = use_node_ref();
    let switcher = Callback::from(move |route| switch(parent_ref.clone(), route));
    html! {
        <main class="min-w-full h-screen bg-slate-700 font-mono">
            <BrowserRouter>
                <Switch<Route> render={switcher} />
            </BrowserRouter>
        </main>
    }
}
