#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use reqwest::Body;
use serde_json::json;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    let onsubmit = move |evt: FormEvent| async move {
        let json_body = json!({
            "username": evt.values()["username"],
            "password": evt.values()["password"]
        });
        let resp = reqwest::Client::new()
            .post("http://127.0.0.1:8000/api/auth/login")
            .body::<Body>(json_body.to_string().into())
            .send()
            .await;

        match resp {
            // Parse data from here, such as storing a response token
            Ok(_data) => log::info!("Login successful!"),

            //Handle any errors from the fetch here
            Err(_err) => {
                log::error!("Login failed - you need a login server running on localhost:8080.")
            }
        }
    };

    rsx! {
        div {
            class: "container",
            link { rel: "stylesheet", href: "bootstrap.min.css" }
            script { src: "bootstrap.bundle.min.js" }
            div {
                class: "row mt-5",
                div {
                class: "border border-primary rounded p-3 bg-white w-50 mx-auto",
                h1 { class:"text-sm-start" ,"Login" }
                hr {}
                form { onsubmit,
                    label { class: "form-label", "Username" }
                    input { class: "form-control", r#type: "text", id: "username", name: "username" }
                    br {}
                    label { class: "form-label", "Password" }
                    input { class: "form-control", r#type: "password", id: "password", name: "password" }
                    br {}
                    button { class: "btn btn-primary", "Login" }
                }
            }
            }
        }
    }
}
