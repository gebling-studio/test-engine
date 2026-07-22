mod method;
mod request;
mod response;
mod rest_api;
mod simple;

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use rest_api::RestAPI;
pub use simple::*;

#[cfg(test)]
mod test {

    use anyhow::Result;
    use serde::Deserialize;

    use crate::rest::{Request, RestAPI};

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct User {
        id:       u32,
        username: String,
        email:    String,
    }

    static API: RestAPI = RestAPI::new("https://jsonplaceholder.typicode.com/");

    static USERS: Request<(), Vec<User>> = API.get("users");

    #[cfg(not_wasm)]
    mod not_wasm_test {

        use pretty_assertions::assert_eq;

        use super::*;
        use crate::rest::get;

        #[test]
        fn size_of_none() {
            const SIZE_OF_NONE: usize = size_of::<()>();
            assert_eq!(SIZE_OF_NONE, 0);
        }

        #[tokio::test]
        async fn test_rest() -> Result<()> {
            let users = USERS.await?;

            assert_eq!(users.len(), 10);

            Ok(())
        }

        #[tokio::test]
        async fn test_simple() -> Result<()> {
            let users: Vec<User> = get("https://jsonplaceholder.typicode.com/users").await?;

            assert_eq!(users.len(), 10);

            Ok(())
        }
    }

    #[cfg(wasm)]
    mod wasm_test {
        use std::sync::atomic::{AtomicUsize, Ordering};

        use wasm_bindgen_test::wasm_bindgen_test;

        use super::*;

        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test(unsupported = test)]
        fn test_rest() -> Result<()> {
            static USERS_COUNT: AtomicUsize = AtomicUsize::new(0);

            wasm_bindgen_test::console_log!("Hello");

            hreads::spawn(async {
                let users = USERS.await.unwrap();

                wasm_bindgen_test::console_log!("Users: {}", users.len());

                USERS_COUNT.store(users.len(), Ordering::Relaxed);

                // assert_eq!(users.len(), 11);
            });

            hreads::block_on(async {
                hreads::sleep(2.0);
            });

            wasm_bindgen_test::console_log!("Romaaa luuunnn");
            wasm_bindgen_test::console_log!("Romaaa luuunnn");
            wasm_bindgen_test::console_log!("Romaaa luuunnn");
            wasm_bindgen_test::console_log!("Romaaa luuunnn");

            // assert_eq!(USERS_COUNT.load(Ordering::Relaxed), 10);

            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));
            wasm_bindgen_test::console_log!("Romaaa luuunnn: {}", USERS_COUNT.load(Ordering::Relaxed));

            hreads::busy_sleep(1.0);
            wasm_bindgen_test::console_log!("Romaaa luuunnn 2: {}", USERS_COUNT.load(Ordering::Relaxed));

            hreads::sleep(1.0);

            wasm_bindgen_test::console_log!("Romaaa luuunnn 3: {}", USERS_COUNT.load(Ordering::Relaxed));

            Ok(())
        }
    }
}
