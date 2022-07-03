use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct Measurement {
    id: usize,
    title: String,
    speaker: String,
    url: String,
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
               .send()
               .await
               .unwrap()
               .json()
               .await
               .unwrap();
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
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
