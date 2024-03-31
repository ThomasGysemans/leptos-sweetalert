use leptos::*;
use leptos_sweetalert::*;

pub fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    Swal::init_escape_key_handler();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let success = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::SUCCESS));
    };

    let warning = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::WARNING));
    };

    let error = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::ERROR));
    };

    let info = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::INFO));
    };

    let question = move |_| {
        Swal::fire(SwalOptions::basic_icon("This is a title", &SwalIcon::QUESTION));
    };

    view! {
        <div>
            <h2>"EXAMPLES"</h2>
            <div>
                <p>"A basic message"</p>
                <button on:click=success>"Try success!"</button>
                <button on:click=warning>"Try warning !"</button>
                <button on:click=error>"Try error !"</button>
                <button on:click=info>"Try info !"</button>
                <button on:click=question>"Try question !"</button>
            </div>
        </div>
    }
}
