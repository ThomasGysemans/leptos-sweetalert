use leptos::*;
use leptos_sweetalert::*;

pub fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let on_click = move |_| {
        Swal::fire(SwalOptions::basic("This is a title"));
    };

    view! {
        <div>
            <h2>"EXAMPLES"</h2>
            <div>
                <p>"A basic message"</p>
                <button on:click=on_click>"Try me!"</button>
            </div>
        </div>
    }
}
