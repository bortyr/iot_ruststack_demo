use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Deserialize)]
struct Measurement {
    id: usize,
    title: String,
    url: String,
    speaker: String,
    // Measurement
    // body: String,
}

impl Debug for Measurement {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id: {}", &self.id)?;
        writeln!(f, "title: {}", &self.title)?;
        // Tutorial
        writeln!(f, "speaker: {}", &self.speaker)?;
        writeln!(f, "url: {}", &self.url)?;
        // Measurement
        // writeln!(f, "body: {}", &self.body)?;
        Ok(())
    }
}

#[function_component(App)]
fn app() -> Html {

    let measurements: UseStateHandle<Vec<Measurement>> = use_state(|| vec![]);
    {
        let measurements = measurements.clone();
        use_effect_with_deps(move |_| {
            let measurements = measurements.clone();
            wasm_bindgen_futures::spawn_local(async move {
            let fetched_measurements: Vec<Measurement> = Request::get("/tutorial/data.json")
            // let fetched_measurements: Vec<Measurement> = Request::get("http://localhost:8000/getm.json")
               .send()
               .await
               .unwrap_or_else(|error| {
                        panic!("Error during sending GET request: {:?}", error);
                })
               .json()
               .await
               .unwrap_or_else(|error| {
                        panic!("Error during parsing JSON: {:?}", error);
                });
                measurements.set(fetched_measurements);
            });
            || ()
        }, ())
    }

    html! {
        <>
        <h1>{ "Hello World" }</h1>
        <div>
            <p>{ format!("Is empty: {}", measurements.is_empty()) }</p>
            <p>{ format!("Length of vector: {}", measurements.len()) }</p>
            <p>{ format!("Contents of vector: {:?}", measurements) }</p>
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
