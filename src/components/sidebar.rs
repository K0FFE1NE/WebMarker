use std::ops::Deref;

use gloo::events::EventListener;
use gloo::{console::log, events::EventListenerOptions};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, Node};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub parent_ref: NodeRef,
}

#[function_component(Sidebar)]
pub fn sidebar(SideBarProps { parent_ref }: &SideBarProps) -> Html {
    let logo_classes = use_state(|| "max-w-0 opacity-0");
    let sidebar_classes = use_state(|| "max-w-0");
    let open = use_state(|| false);
    let onmouseenter = {
        let classes = logo_classes.clone();
        Callback::from(move |_: MouseEvent| classes.set("max-w-[1000px] opacity-100"))
    };

    let onmouseleave = {
        let classes = logo_classes.clone();
        Callback::from(move |_: MouseEvent| classes.set("max-w-0 opacity-0"))
    };

    let toggle_state = {
        let state = open.clone();
        Callback::from(move |_: MouseEvent| {
            if *state {
                state.set(false);
            } else {
                state.set(true);
            }
        })
    };

    let onclick = toggle_state.clone();

    {
        let sidebar_classes = sidebar_classes.clone();
        use_effect_with_deps(
            move |open| {
                if **open {
                    sidebar_classes.set("max-w-[150px] px-2 py-4");
                } else {
                    sidebar_classes.set("max-w-0");
                }
            },
            open.clone(),
        );
    }

    {
        let open = open.clone();
        let cb = use_callback(|_e: Event, open| open.set(!**open), open);
        use_effect_with_deps(
            {
                move |parent_ref: &NodeRef| {
                    let mut click_listener = None;
                    if let Some(element) = parent_ref.cast::<HtmlElement>() {
                        let listener = EventListener::new(&element.clone(), "click", move |e| {
                            if e.target()
                                .expect("Target to exist")
                                .dyn_into::<HtmlElement>()
                                .unwrap()
                                == element.clone()
                            {
                                log!("wew");
                                cb.emit(e.clone())
                            }
                        });

                        click_listener = Some(listener);
                    }

                    move || drop(click_listener)
                }
            },
            parent_ref.clone(),
        );
    }

    html! {
        <>
            if !*open {
                <button onclick={onclick} onmouseenter={onmouseenter} onmouseleave={onmouseleave} class="px-2 py-4 absolute text-zinc-300 flex items-end hover:text-zinc-100 transition-all">
                    <p class="text-4xl leading-none font-bold">{"W"}</p>
                    <p class={classes!("text-xl", "overflow-hidden", "transition-all", *logo_classes)}>{"ebMarker"}</p>
                </button>
            } else {
                <button onclick={onclick} class="px-2 py-4 absolute text-zinc-300 flex items-end hover:text-zinc-100 transition-all">
                    <p class="text-4xl leading-none font-bold">{"W"}</p>
                    <p class={classes!("text-xl", "overflow-hidden", "transition-all", *logo_classes)}>{"ebMarker"}</p>
                </button>
            }
            <div class={classes!("flex flex-col gap-y-2 bg-slate-900 transition-all h-full".to_owned(), *sidebar_classes)}>
            </div>

        </>
    }
}
