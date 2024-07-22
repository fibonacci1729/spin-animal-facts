use bindings::exports::component::animal_facts::animal_facts::Guest;
use spin_sdk::http::{Request, Response};

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    /// Get a random animal fact.
    fn get_random_fact() -> Result<String, String> {
        let result: Result<Response, _> = spin_executor::run(spin_sdk::http::send(Request::get(
            "https://random-data-api.fermyon.app/animals/json",
        )));

        match result {
            Ok(resp) => {
                let fact = std::str::from_utf8(resp.body()).unwrap().to_string();
                Ok(fact)
            }
            Err(err) => {
                println!("failed to fetch a random animal fact: {err}");
                Ok("Here is a fact...".to_string())
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);