use component::cat_facts;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_cat_facts_app(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let fact = cat_facts::cat_facts::get_random_fact().map_err(|e| anyhow::anyhow!(e))?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(fact)
        .build())
}

wit_bindgen::generate!({
    inline: r"
    package component:cat-facts;

    interface cat-facts {
        /// Get a random cat fact.
        get-random-fact: func() -> result<string, string>;
    }
    
    world cat-facts-app {
        import cat-facts;
    }
    ",
});