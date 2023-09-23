use crate::components::sidebar::Sidebar;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let main_ref = use_node_ref();
    html! {
        <main ref={main_ref.clone()} class="min-w-full h-screen bg-slate-700 font-mono">
            <Sidebar parent_ref={main_ref} />
        </main>
    }
}
