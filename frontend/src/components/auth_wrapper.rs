use web_sys::{window, Storage};
use yew::prelude::*;

// Include the generated authentication configuration
include!(concat!(env!("OUT_DIR"), "/auth_config.rs"));

#[derive(Properties, PartialEq)]
pub struct AuthWrapperProps {
    pub children: Children,
}

#[function_component(AuthWrapper)]
pub fn auth_wrapper(props: &AuthWrapperProps) -> Html {
    let is_authenticated = use_state(|| false);
    let is_checking = use_state(|| true);

    {
        let is_authenticated = is_authenticated.clone();
        let is_checking = is_checking.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let auth_result = check_authentication().await;
                is_authenticated.set(auth_result);
                is_checking.set(false);
            });
        });
    }

    if *is_checking {
        return html! {
            <div class="auth-checking">
                <div class="loading-spinner"></div>
                <p>{"Checking authentication..."}</p>
            </div>
        };
    }

    if *is_authenticated {
        html! {
            <div class="authenticated-content">
                { props.children.clone() }
            </div>
        }
    } else {
        html! {
            <div class="access-denied">
                <div class="access-denied-content">
                    <p>{"Access denied"}</p>
                </div>
            </div>
        }
    }
}

async fn check_authentication() -> bool {
    // Check for HTTP Basic Authentication header
    // Since we're in a browser environment, we simulate the header check via localStorage
    // In a real server-side app, this would check the actual Authorization header
    if let Some(storage) = get_local_storage() {
        if let Ok(Some(auth_header)) = storage.get_item("Authorization") {
            // Compare with the expected header from environment variable
            return auth_header == get_expected_auth_header();
        }
    }

    false
}

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}
