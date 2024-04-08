use pgrx::prelude::*;

use anyhow::Result;
use reqwest::blocking::Client;
use serde_json;
use urlencoding;

pgrx::pg_module_magic!();

pub fn get_query_plan(query_text: &str) -> Result<pgrx::Json> {
    Ok(Spi::connect(|mut client| {
        client
            .update(
                &format!("EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) {query_text}"),
                None,
                None,
            )?
            .first()
            .get_one::<pgrx::Json>()
    })?
    .unwrap())
}

#[pg_extern]
fn explain_ui(query_text: String) -> Result<String> {
    let plan_json = get_query_plan(&query_text)?;
    let plan_json_str = serde_json::to_string(&plan_json).unwrap();

    // Construct URL-encoded form data manually
    let form_data = format!(
        "query={}&title={}&plan={}",
        urlencoding::encode(&query_text),
        urlencoding::encode(&query_text),
        urlencoding::encode(&plan_json_str)
    );

    // Create a client
    let client = Client::new();

    // Perform the POST request
    let res = client
        .post("https://explain.dalibo.com/new")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()?;

    Ok(res.url().to_string())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    // Todo: write tests?
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
