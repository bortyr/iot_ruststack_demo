use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, PartialEq, Deserialize)]
struct Measurement {
    pub id: usize,
    pub temperature: String,
}

impl Debug for Measurement {
fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.id)?;
        write!(f, "{}", &self.temperature)?;
        Ok(())
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{} <{}>", self.id, self.temperature)
    }
}

struct Measurements(pub Vec<Measurement>);

impl Display for Measurements {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for v in &self.0 {
            writeln!(f, "{}:\t{}\t", v.id, v.temperature)?;
        }
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
            let fetched_measurements: Vec<Measurement> = Request::get("http://localhost:8000/getm.json")
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

    let values: Measurements = Measurements((&measurements).to_vec());


    html! {
        <>
        <body class="has-background-primary" style="width: 100%!important;">
            <br/>
            <h1 class="title is-1 has-text-centered">{ "IoT eHealth Acquisition System Software Stack Based On Rust" }</h1>
            <br/>
        </body>
        <div class="columns">
            <div class="column">
            <br/>
            <nav class="level">
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{format!("Is database empty ?")}</p>
                  <p class="title">{ format!("{}", measurements.is_empty()) }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{format!("Amount of measurements in database")}</p>
                  <p class="title">{ format!("{}", measurements.len()) }</p>
                </div>
              </div>
              </nav>
            <br/>
            <h2 class="title is-2 has-text-centered">{ "Measurements" }</h2>
            <p class="has-text-centered">{ "Temperature" }</p>
            <p class="has-text-centered multiline">{ format!("{}", values) }</p>
            </div>
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
