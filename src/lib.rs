use leptos::*;
use leptos::html::Div;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};

/// Defines an icon to be displayed in the Swal.
/// Use the pre-built ones to make sure you don't
/// accidentially make a mistake in the name of an icon.
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
#[derive(Clone)]
pub struct Swal<S: AsRef<str> + Clone + Default + leptos::IntoView> {
    /// Whether or not the alert is open.
    open: bool,

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

impl<S: AsRef<str> + Clone + Default + leptos::IntoView> Default for Swal<S> {
    fn default() -> Self {
        Self {
            open: false,
            title: S::default(),
            text: S::default(),
            icon: &SwalIcon::NONE,
            show_confirm_button: true,
        }
    }
}

impl<S: AsRef<str> + Clone + Default + leptos::IntoView> Swal<S> {
    /// Opens the swal with just a simple title.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_sweetalert::*;
    /// # use web_sys::MouseEvent;
    ///
    /// // Declaring the generic type explicitly won't be necessary
    /// // most of the time in your project, but in a test I must set it.
    /// let (swal, set_swal) = create_signal(Swal::<&str>::default());
    ///
    /// // This will display a swal with a simple title:
    /// // "The button was clicked."
    /// let on_button_clicked = move |ev: MouseEvent| {
    ///     set_swal.update(|c| *c = Swal::basic("The button was clicked."));
    /// };
    /// ```
    pub fn basic(title: S) -> Self {
        Self {
            title,
            open: true,
            ..Self::default()
        }
    }

    /// Opens the swal with just a simple title.
    /// Contrary to [basic], this will modify `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_sweetalert::*;
    /// # use web_sys::MouseEvent;
    ///
    /// // Declaring the generic type explicitly won't be necessary
    /// // most of the time in your project, but in a test I must set it.
    /// let (swal, set_swal) = create_signal(Swal::<&str>::default());
    ///
    /// // This will display a swal with a simple title:
    /// // "The button was clicked."
    /// let on_button_clicked = move |ev: MouseEvent| {
    ///     set_swal.update(|c| c.to_basic("The button was clicked."));
    /// };
    /// ```
    pub fn to_basic(&mut self, title: S) {
        *self = Self::basic(title);
    }

    /// Checks if the current swal is open.
    pub fn is_open(&self) -> bool {
        return self.open;
    }

    /// Closes the current swal.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Re-opens the current swal.
    ///
    /// Using it while it has never been opened before
    /// is quite useless since it will only display empty
    /// text in an empty box.
    pub fn reopen(&mut self) {
        self.open = true;
    }
}

/// The view for the SweetAlert.
/// It is necessary to manually include it since
/// Leptos doesn't provide any portal that would allow
/// the creation of this view dynamically.
///
/// # Example
///
/// ```
/// # use leptos::*;
/// # use leptos_sweetalert::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (swal, set_swal) = create_signal(Swal::<&str>::default());
///
///     // Include the component wherever you need it.
///     // There can be several of them, but it may lead
///     // to undefined behaviors.
///     view! {
///         <SwalComponent options={swal} setter={set_swal} />
///     }
/// }
/// ```
#[component]
pub fn SwalComponent<S: AsRef<str> + Clone + Default + leptos::IntoView + 'static>(
    /// The options of a swal.
    options: ReadSignal<Swal<S>>,

    /// The setter of the signal so that it is 
    /// possible to make changes to the parameters dynamically.
    /// It is necessary to make sure that the swal can be closed.
    setter: WriteSignal<Swal<S>>
) -> impl IntoView {
    let swal_container_ref = create_node_ref::<Div>();
    
    // This will only get triggered if the swal is already open, since
    // a hidden swal has "pointer-events" set to "none" in the CSS.
    let on_backdrop_clicked = move |ev: MouseEvent| {
        if let Some(container) = swal_container_ref.get() {
            if let Some(target) = ev.target() {
                let actual_target = target.dyn_ref::<HtmlElement>();
                if actual_target.is_some() {
                    if !container.contains(Some(actual_target.unwrap())) {
                        setter.update(|c| c.close());
                    }
                }
            }
        }
    };

    view! {
        <div on:click=on_backdrop_clicked class="swal-backdrop" aria-hidden={move || if options.get().open { "false" } else { "true" } }>
            <div _ref=swal_container_ref class="swal-container">
                <strong class="swal-title">{move || options.get().title}</strong>
                <p class="swal-text">{move || options.get().text}</p>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_swal() {
        let swal: Swal<&str> = Swal::default();
        assert_eq!(swal.title, "");
        assert_eq!(swal.text, "");
        assert_eq!(swal.show_confirm_button, true);
        assert_eq!(swal.open, false);
    }

    #[test]
    fn test_basic() {
        let mut swal: Swal<&str> = Swal::basic("Hello");
        assert_eq!(swal.title, "Hello");
        assert_eq!(swal.text, "");
        swal.to_basic("World");
        assert_eq!(swal.title, "World");
        assert_eq!(swal.text, "");
    }

    #[test]
    fn test_close() {
        let mut swal: Swal<&str> = Swal::default();
        assert!(!swal.is_open());
        swal.reopen();
        assert!(swal.is_open());
        swal.close();
        assert!(!swal.is_open());
    }
}
