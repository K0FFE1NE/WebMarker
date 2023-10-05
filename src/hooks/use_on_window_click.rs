use gloo::events::EventListener;
use web_sys::Event;
use yew::Callback;

pub fn use_on_window_click(callback: Callback<Event>) -> EventListener {
    EventListener::new(&gloo::utils::window(), "click", move |e| {
        callback.emit(e.clone());
    })
}
