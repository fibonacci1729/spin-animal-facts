use bindings::exports::component::cat_facts::cat_facts::Guest;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    /// Get a random cat fact.
    fn get_random_fact() -> Result<String, String> {
        // TODO: make outbound request to a cat facts API
        Ok("There's a cat...".to_string())
    }
}

bindings::export!(Component with_types_in bindings);
