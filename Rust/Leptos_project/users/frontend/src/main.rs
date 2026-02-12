use leptos::*;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::Response;

#[derive(Clone, Debug, Deserialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[component]
fn App() -> impl IntoView {
    // Reactive signal for users
    let users = create_rw_signal(Vec::<User>::new());

    // Fetch users from Actix backend on startup
    spawn_local({
        let users = users.clone();
        async move {
            let window = web_sys::window().unwrap();

            // Perform fetch
            let resp_value = wasm_bindgen_futures::JsFuture::from(
                window.fetch_with_str("http://127.0.0.1:9112/users")
            )
            .await
            .unwrap();

            let resp: Response = resp_value.dyn_into().unwrap();

            let json = wasm_bindgen_futures::JsFuture::from(resp.json().unwrap())
                .await
                .unwrap();

            let data: Vec<User> = serde_wasm_bindgen::from_value(json).unwrap();

            users.set(data);
        }
    });

    // Render the table
    view! {
        <main>
            <h1>"Users"</h1>
            <table border="1" cellpadding="5">
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"First Name"</th>
                        <th>"Last Name"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || users.get().clone().into_iter().map(|u| view! {
                        <tr>
                            <td>{u.id}</td>
                            <td>{u.first_name}</td>
                            <td>{u.last_name}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </main>
    }
}

fn main() {
    // Enable better error messages in browser
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    mount_to_body(App);
}
