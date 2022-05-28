#![recursion_limit = "256"]

use wasm_web_helpers::{
    error::Result,
    fetch::{fetch_success_text, Request, Response},
    spawn_local,
};
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html};
use yew_mdc_widgets::{auto_init, console, dom, Button, List, ListItem, MdcWidget, TextField, TopAppBar};

struct Root {
    responses: Vec<String>,
}

enum Msg {
    Submit,
    Fetch(String),
    Error(String),
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { responses: Vec::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                let uri = dom::existing::select_element::<HtmlInputElement>("#uri > input").value();
                if !uri.is_empty() {
                    let request = Request::get(&format!("/echo/{}", uri));
                    let callback = ctx.link().callback(|result: Result<(Response, Result<String>)>| {
                        match result.and_then(|(_, body)| body) {
                            Ok(body) => Msg::Fetch(body),
                            Err(err) => Msg::Error(format!("Fetch error: {:?}", err)),
                        }
                    });
                    spawn_local(async move {
                        callback.emit(fetch_success_text(request).await);
                    });
                }
                false
            },
            Msg::Fetch(data) => {
                self.responses.push(data);
                true
            },
            Msg::Error(error) => {
                console::error!(&error);
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let top_app_bar = TopAppBar::new()
            .id("top-app-bar")
            .title("Echo lapp")
            .enable_shadow_when_scroll_window();

        let mut list = List::ul().divider();
        for uri in self.responses.iter().rev() {
            list = list.item(ListItem::new().text(uri)).divider();
        }

        html! {
            <>
                <div class = "app-content">
                    { top_app_bar }
                    <div class = "mdc-top-app-bar--fixed-adjust">
                        <div class = "content-container">
                            <h1 class = "title mdc-typography--headline5">{ "Echo" }</h1>
                            <div class = "mdc-layout-grid">
                                <div class = "mdc-layout-grid__inner">
                                    <div class = "mdc-layout-grid__cell mdc-layout-grid__cell--span-4 mdc-layout-grid__cell--align-bottom">
                                        { TextField::filled().id("uri").class("expand").label("URI") }
                                    </div>
                                    <div class = "mdc-layout-grid__cell mdc-layout-grid__cell--span-1 mdc-layout-grid__cell--align-bottom">
                                        { Button::raised().label("submit").on_click(ctx.link().callback(|_| Msg::Submit)) }
                                    </div>
                                </div>
                            </div>
                            { list }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        auto_init();
    }
}

fn main() {
    let root = dom::existing::get_element_by_id("root");
    yew::Renderer::<Root>::with_root(root).render();
}
