use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let logo_classes = use_state(|| "max-w-0 opacity-0");
    let open = use_state(|| false);
    let onmouseenter = {
        let classes = logo_classes.clone();
        Callback::from(move |_: MouseEvent| classes.set("max-w-[1000px] opacity-100"))
    };

    let onmouseleave = {
        let classes = logo_classes.clone();
        Callback::from(move |_: MouseEvent| classes.set("max-w-0 opacity-0"))
    };

    html! {
        <div class="w-fit flex flex-col px-4 py-2 gap-y-2">
            if !*open {
                <button onmouseenter={onmouseenter} onmouseleave={onmouseleave} class="text-zinc-300 flex items-end hover:text-zinc-100 transition-all">
                    <p class="text-4xl leading-none font-bold">{"W"}</p>
                    <p class={classes!("text-xl", "overflow-hidden", "transition-all", *logo_classes)}>{"ebMarker"}</p>
                </button>
            }
        </div>
    }
}