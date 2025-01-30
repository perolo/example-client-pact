

use pact_consumer::prelude::*;
//use expectest::prelude::*;
//use serde_json;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
}

#[tokio::test]
async fn simple_user_test() {
    let example = User {
        id: 1,
        name: "Alice".to_owned(),
    };
    // Define the Pact for the test, specify the names of the consuming
    // application and the provider application.
    let provider_service = PactBuilder::new("Consumer", "Example_Service")
        // Start a new interaction. We can add as many interactions as we want.
        .interaction("a User request", "", |mut i| {
            // Defines a provider state. It is optional.
            i.given("there is one user");
            i.request.path("/user");
            // Define the response we want returned. We assume a 200 OK
            // response by default.
            i.response
                .content_type("application/json")
                .body(r#"{"id":1,"name":"Alice"}"#);
            //.body(serde_json::json!(example));
            // Return the interaction builder back to the pact framework
            i
        })
        .start_mock_server(None, None);

    let mallory_url = provider_service.path("/user");
    let response = reqwest::get(mallory_url)
        .await
        .expect("could not fetch URL")
        .text()
        .await
        .expect("Could not read response body");
    let resp: User = serde_json::from_str(response.as_str()).unwrap();

    assert_eq!(response, r#"{"id":1,"name":"Alice"}"#);
    assert_eq!(serde_json::json!(example), serde_json::json!(resp));
}

fn main() {}
