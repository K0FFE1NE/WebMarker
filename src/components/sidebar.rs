use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Sidebar {
    state: bool,
    logo_classes: String,
    sidebar_classes: String,
    window_listener: Option<EventListener>,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub parent_ref: NodeRef,
}

pub enum SidebarMsg {
    LogoClick,
    LogoEnter,
    LogoLeave,
    OutClick,
    // When it's required to send a message (e.g. function return signatures)
    // but no work/render is required
    Nothing,
}

impl Component for Sidebar {
    type Message = SidebarMsg;
    type Properties = SidebarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: false,
            logo_classes: "max-w-0 opacity-0".to_owned(),
            sidebar_classes: "max-w-0".to_owned(),
            window_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let btn_interior = html!(
            <>
                <p class="text-4xl leading-none font-bold">{"W"}</p>
                <p class={classes!("text-xl", "overflow-hidden", "transition-all", self.logo_classes.clone())}>{"ebMarker"}</p>
            </>);

        let onclick = ctx.link().callback(|_| SidebarMsg::LogoClick);
        let onmouseenter = ctx.link().callback(|_| SidebarMsg::LogoEnter);
        let onmouseleave = ctx.link().callback(|_| SidebarMsg::LogoLeave);

        html!(
        <>
            if !self.state {
                <button onclick={onclick} onmouseenter={onmouseenter} onmouseleave={onmouseleave} class="px-2 py-4 absolute text-zinc-300 flex items-end hover:text-zinc-100 transition-all">
                    {btn_interior}
                </button>
            } else {
                <button onclick={ctx.link().callback(|_| SidebarMsg::LogoClick)} class="px-2 py-4 absolute text-zinc-300 flex items-end hover:text-zinc-100 transition-all">
                    {btn_interior}
                </button>
            }
            <div class={classes!("flex flex-col gap-y-2 bg-slate-900 transition-all h-full".to_owned(), self.sidebar_classes.clone())}></div>
        </>)
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarMsg::LogoClick => {
                if self.state {
                    self.sidebar_classes = "max-w-0".to_owned();
                    self.state = false;
                } else {
                    self.sidebar_classes = "max-w-[150px] px-2 py-4".to_owned();
                    self.state = true;
                }
                true
            }
            SidebarMsg::LogoEnter => {
                self.logo_classes = "max-w-[1000px] opacity-100".to_owned();
                true
            }
            SidebarMsg::LogoLeave => {
                self.logo_classes = "max-w-0 opacity-0".to_owned();
                true
            }
            SidebarMsg::OutClick => {
                if self.state {
                    self.sidebar_classes = "max-w-0".to_owned();
                    self.logo_classes = "max-w-0 opacity-0".to_owned();
                    self.state = false;
                    true
                } else {
                    false
                }
            }
            SidebarMsg::Nothing => false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let window = gloo::utils::window();
        let parent = ctx.props().parent_ref.cast::<HtmlElement>().unwrap();
        let onclick = ctx.link().callback(move |e: MouseEvent| {
            if e.target()
                .expect("A clicked on target should exist")
                .dyn_into::<HtmlElement>()
                .expect("Parent of a HtmlElement should be a HtmlElement")
                == parent
            {
                SidebarMsg::OutClick
            } else {
                SidebarMsg::Nothing
            }
        });

        let listener = EventListener::new(&window, "click", move |e| {
            let event = e
                .dyn_ref::<MouseEvent>()
                .expect("Click event should be a mouse event");
            onclick.emit(event.clone())
        });

        // ? I assume this will be 'dropped' once the sidebar is no longer rendered
        // In a functional component's use_effect, this would need to be 'manually' dropped,
        // but I assume here it will be dropped along with the struct with struct components
        self.window_listener = Some(listener);
    }
}
