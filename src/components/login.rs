use yew::prelude::*;
use web_sys::window;
use gloo_net::http::Request;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = email.clone();
            let password = password.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("http://localhost:5000/api/auth/login")
                    .header("Content-Type", "application/json")
                    .body(serde_json::json!({
                        "email": *email,
                        "password": *password
                    }).to_string())
                    .expect("Failed to create request")
                    .send()
                    .await
                    .unwrap();

                if resp.ok() {
                    let json: serde_json::Value = resp.json().await.unwrap();
                    if let Some(token) = json["token"].as_str() {
                        window().unwrap().local_storage().unwrap().unwrap().set_item("token", token).unwrap();
                        navigator.push(&Route::Home);
                    }
                }
            });
        })
    };

    html! {
        <div class="flex justify-center items-center h-screen">
            <form class="bg-white p-8 rounded shadow-md w-96" {onsubmit}>
                <h1 class="text-2xl font-bold mb-4 text-center">{"Login"}</h1>
                <input type="email" placeholder="Email"
                    class="border p-2 w-full mb-4"
                    value = {(*email).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        email.set(input.value());
                    })}
                />
                <input type="password" placeholder="Password"
                    class="border p-2 w-full mb-4"
                    value = {(*password).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        password.set(input.value());
                    })}
                />
                <button type="submit" class="bg-blue-500 text-white p-2 w-full rounded">{"Login"}</button>
                <p class="mt-2 text-sm">{"Don't have an account? "}<a class="text-blue-500"><Link<Route> to={Route::Register} >{"Register"}</Link<Route>></a></p>
            </form>
        </div>
    }
}