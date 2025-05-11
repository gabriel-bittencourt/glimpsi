#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::app::components::ContentComponent;
use shared::types::content::Content;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}


pub fn App() -> Element {

    // Fetching configured shortcuts
    let data = use_resource(|| async move {
        let res = invoke_without_args("get_content").await;

        serde_wasm_bindgen::from_value::<Content>(res)
    });

    match data.read_unchecked().as_ref() {
        Some(Ok(content)) => {
            rsx! {
                link { rel: "stylesheet", href: "/assets/styles.css" }
                div {
                    class: "container",
                    div {
                        class: "grid",
                        ContentComponent { content: content.clone() }
                    }
                }   
            }
        },
        Some(Err(_)) => {
            rsx! { "Error loading configuration" }
        },
        None => {
            rsx! { "Loading..." }
        }
    }

}