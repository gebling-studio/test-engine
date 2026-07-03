use netrun::rest::RestAPI;
use serde::Deserialize;
use test_engine::net::Request;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct User {
    pub id:   u32,
    pub name: String,
}

pub(crate) static TEST_REST_API: RestAPI = RestAPI::new("https://jsonplaceholder.typicode.com/");

pub(crate) static TEST_REST_REQUEST: Request<(), Vec<User>> = TEST_REST_API.get("users");
