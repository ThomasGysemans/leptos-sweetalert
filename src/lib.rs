/// Defines an icon to be displayed in the Swal.
/// Use the pre-built ones to make sure you don't
/// accidentially make a mistake in the name of an icon.
#[derive(Debug, PartialEq)]
pub struct SwalIcon(&'static str);

impl SwalIcon {
    /// Shows an information icon (a question mark).
    pub const INFO: Self = Self("info");

    /// Shows an error icon (a cross).
    pub const ERROR: Self = Self("error");

    /// Shows a check (for a successfull operation).
    pub const SUCCESS: Self = Self("success");

    /// Shows a warning (an exclamation mark).
    pub const WARNING: Self = Self("warning");

    /// Shows no icon.
    /// It is the default icon that a Swal will use.
    pub const NONE: Self = Self("NONE");
}

impl std::fmt::Display for SwalIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Defines the parameters of a Sweet Alert.
/// It uses a generic parameter to allow you to use
/// either a string slices or Strings for the value
/// of some parameters.
///
/// # Example
///
/// ```
/// # use leptos_sweetalert::*;
/// 
/// // All three examples below do the exact same thing.
///
/// // Use methods to reduce the amount
/// // of fields you have to manually assign
/// // for this struct.
/// // In this case, the generic type (`S`) is deduced
/// // from the parameter, which is a `&'static str`.
/// let opt = SwalOptions::basic("This is a title");
/// 
/// // This is equivalent to the above.
/// // It may be useful in some cases when
/// // the compiler is unable to infer the
/// // type of `S`.
/// let opt = SwalOptions::<&'static str>::basic("This is a title");
///
/// // Look at the methods for a better developer experience.
/// // However, if you want to fully customize the parameters
/// // in a way that the methods doesn't allow you to, then use:
/// let opt = SwalOptions {
///     title: "This is a title",
///     ..SwalOptions::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SwalOptions<S: AsRef<str> + Clone + Default + leptos::IntoView> {
    /// The title of the alert.
    /// If its value is an empty string,
    /// no title will be displayed.
    pub title: S,

    /// A text or description to display below the title.
    /// If its value is an empty string,
    /// no description will be displayed.
    pub text: S,

    /// An icon to display above the title.
    /// By default, there is no icon, meaning
    /// the value of this property is `&SwalIcon::NONE`.
    pub icon: &'static SwalIcon,

    /// Should the default confirmation button be displayed?
    /// It defaults to `true`.
    pub show_confirm_button: bool,
}

impl<S: AsRef<str> + Clone + Default + leptos::IntoView> Default for SwalOptions<S> {
    fn default() -> Self {
        Self {
            title: S::default(),
            text: S::default(),
            icon: &SwalIcon::NONE,
            show_confirm_button: true,
        }
    }
}

impl<S: AsRef<str> + Clone + Default + leptos::IntoView> SwalOptions<S> {
    /// Creates Swal options for a simple alert with just a title.
    /// All other parameters are set to their default values.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let opts = SwalOptions::basic("This is a title");
    /// assert_eq!(opts.title, "This is a title");
    /// assert_eq!(opts.text, ""); // see default values.
    /// ```
    pub fn basic(title: S) -> Self {
        Self {
            title,
            ..Self::default()
        }
    }
}

#[allow(non_snake_case)]
pub mod Swal {
    use std::time::Duration;

    use super::SwalOptions;
    use leptos::html::{AnyElement, Div};
    use leptos::{set_timeout, *};
    use leptos_dom::HtmlElement;

    use web_sys::wasm_bindgen::JsCast;
    use web_sys::{window, Element, MouseEvent};

    #[allow(unused)]
    use log::info;

    /// Creates a Sweet Alert with the options defined in `opt`.
    /// See the docs for [SwalOptions](`#SwalOptions`) to know how to use it.
    pub fn fire<S: AsRef<str> + Clone + Default + leptos::IntoView>(opt: SwalOptions<S>) {
        if let Some(swal) = get_swal() {
            swal.remove();
        }
        document()
            .body()
            .expect("Could not find body")
            .append_child(
                &SwalComponent(opt)
                    .into_view()
                    .into_html_element()
                    .expect("Could not parse Swal to HTML"),
            )
            .expect("Could not append Swal to body");
        set_timeout(
            || {
                get_swal()
                    .unwrap()
                    .set_attribute("aria-hidden", "false")
                    .expect("Could not set aria-hidden of Swal")
            },
            Duration::from_secs_f32(0.01),
        );
    }

    fn get_swal() -> Option<Element> {
        document().get_element_by_id("swal")
    }

    fn get_transition_duration(el: &Element) -> f32 {
        let css_value = window()
            .expect("Could not get window")
            .get_computed_style(&el)
            .expect("Could not get computed style of Swal")
            .expect("Could not get computed style of Swal")
            .get_property_value("transition-duration");
        if let Ok(css_value) = css_value {
            css_value
                .get(0..css_value.len() - 1)
                .expect("Invalid CSS value for transition duration of Swal")
                .parse::<f32>()
                .expect("Could not parse transition duration of Swal")
        } else {
            0.0
        }
    }

    fn SwalComponent<S>(opt: SwalOptions<S>) -> HtmlElement<AnyElement>
    where
        S: AsRef<str> + Clone + Default + leptos::IntoView,
    {
        let swal_container_ref = create_node_ref::<Div>();

        let on_backdrop_clicked = move |ev: MouseEvent| {
            if let Some(container) = swal_container_ref.get() {
                if let Some(target) = ev.target() {
                    let actual_target = target.dyn_ref::<web_sys::HtmlElement>();
                    if actual_target.is_some() {
                        if !container.contains(Some(actual_target.unwrap())) {
                            // Here the goal is to remove the swal from the DOM
                            // as soon as the ending transition is over.
                            // My solution is to extract the transition duration
                            // from the computed styles and remove the node in a
                            // delayed closure (via set_timeout from leptos).
                            //
                            // Initially,
                            // I was going to listen to the "transitionend" event,
                            // but WebAssembly's only solution in my case would leak memory,
                            // as they so gently explain here:
                            // https://rustwasm.github.io/wasm-bindgen/examples/closures.html#srclibrs
                            // (which is awful and dumb)
                            let swal = get_swal().unwrap();
                            swal.set_attribute("aria-hidden", "true")
                                .expect("Could not change the Swal's aria-hidden attribute.");
                            set_timeout(
                                || get_swal().unwrap().remove(),
                                Duration::from_secs_f32(get_transition_duration(&swal)),
                            );
                        }
                    }
                }
            }
        };

        (view! {
            <div id="swal" on:click=on_backdrop_clicked class="swal-backdrop" aria-hidden="true">
                <div _ref=swal_container_ref class="swal-container">
                    <strong class="swal-title">{opt.title}</strong>
                    <p class="swal-text">{opt.text}</p>
                </div>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Swal component")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_swal() {
        let opts = SwalOptions::<&str>::default();
        assert_eq!(opts.title, "");
        assert_eq!(opts.text, "");
        assert_eq!(opts.icon, &SwalIcon::NONE);
        assert_eq!(opts.show_confirm_button, true);
    }

    #[test]
    fn test_basic() {
        let opts = SwalOptions::<&str>::basic("Hello");
        assert_eq!(opts.title, "Hello");
        assert_eq!(opts.text, "");
    }
}
