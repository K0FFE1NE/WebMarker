use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

pub struct SidebarBtn {
    name: String,
    to: String
}

#[derive(PartialEq, Properties)]
pub struct SidebarBtnProps {
    url: String,
    name: String,
}

pub enum SidebarBtnMsg {
    Click
}

impl Component for SidebarBtn {
    type Message = SidebarBtnMsg;

    type Properties = SidebarBtnProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            to: ctx.props().url.clone(),
            name: ctx.props().name.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.link().callback(|_| SidebarBtnMsg::Click)}>{&self.name}</button>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarBtnMsg::Click => {
                let navigator = ctx.link().navigator().expect("To be wrapped in router");
                navigator.push(&Route::from(self.to.clone()));
                true
            },
        }
    }
}