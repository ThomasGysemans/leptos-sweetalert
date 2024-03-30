use leptos::*;
use leptos_sweetalert::*;

pub fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let (options, set_options) = create_signal(Swal::default());

    view! {
        <SwalComponent options={options} setter={set_options} />
        <div>
            <h2>"EXAMPLES"</h2>
            <div>
                <p>"A basic message"</p>
                <button on:click=move |_| { set_options.update(|o| *o = Swal::basic("This is a title")) }>"Try me!"</button>
            </div>
        </div>
    }
}
