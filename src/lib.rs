/// Defines an icon to be displayed in the Swal.
/// Use the pre-built ones to make sure you don't
/// accidentially make a mistake in the name of an icon.
#[derive(Debug, PartialEq)]
pub struct SwalIcon(&'static str);

impl SwalIcon {
    /// Shows an information icon (the letter "i" in a circle).
    pub const INFO: Self = Self("info");

    /// Shows an error icon (a cross).
    pub const ERROR: Self = Self("error");

    /// Shows a check (for a successfull operation).
    pub const SUCCESS: Self = Self("success");

    /// Shows a warning (an exclamation mark).
    pub const WARNING: Self = Self("warning");

    /// Shows a question mark
    pub const QUESTION: Self = Self("question");

    /// Shows no icon.
    /// It is the default icon that a Swal will use.
    pub const NONE: Self = Self("NONE");
}

impl std::fmt::Display for SwalIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The reasons why an alert has been closed.
#[derive(Debug, PartialEq)]
pub enum SwalDismissReason {
    /// The user clicked the backdrop.
    Backdrop,

    /// The user clicked the cancel button.
    Cancel,

    /// The user clicked the close button.
    Close,

    /// The user clicked the Escape key.
    Esc,
}

/// The data that is returned when an alert is closed.
#[derive(Debug)]
pub struct SwalResult {
    /// The "Confirm" button was clicked, the value will contain the result.
    pub is_confirmed: bool,

    /// The "Deny" button was clicked, the value will be false.
    pub is_denied: bool,

    /// The "Cancel" button was clicked, the dismiss will be
    /// [SwalDismissReason.Cancel](`#SwalDismissReason.Cancel`)
    pub is_dismissed: bool,

    /// The value from the popup, possible values:
    /// - `true` for simple confirmed dialogs
    /// - `false` for denied popups
    pub value: bool,

    /// The dismissal reason, see [SwalDismissReason](`#SwalDismissReason`).
    /// It's optional because if the popup is confirmed, then it wasn't dismissed,
    /// so no reason to specify a dismiss reason.
    pub dismiss: Option<SwalDismissReason>,
}

impl SwalResult {
    /// Creates a response that is the result of a confirmed popup.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let r = SwalResult::confirmed();
    /// assert!(r.is_confirmed);
    /// assert!(r.value);
    /// assert!(!r.is_denied);
    /// assert!(!r.is_dismissed);
    /// assert!(r.dismiss.is_none());
    /// ```
    pub fn confirmed() -> Self {
        Self {
            is_confirmed: true,
            value: true,
            is_denied: false,
            is_dismissed: false,
            dismiss: None,
        }
    }

    /// Creates a response that is the result of a denied popup.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let r = SwalResult::denied();
    /// assert!(!r.is_confirmed);
    /// assert!(!r.value);
    /// assert!(r.is_denied);
    /// assert!(!r.is_dismissed);
    /// assert!(r.dismiss.is_none());
    /// ```
    pub fn denied() -> Self {
        Self {
            is_confirmed: false,
            value: false,
            is_denied: true,
            is_dismissed: false,
            dismiss: None,
        }
    }

    /// Creates a response that is the result of a canceled popup.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let r = SwalResult::canceled(SwalDismissReason::Backdrop);
    /// assert!(!r.is_confirmed);
    /// assert!(!r.value);
    /// assert!(!r.is_denied);
    /// assert!(r.is_dismissed);
    /// assert!(r.dismiss.is_some());
    /// assert!(r.dismiss.unwrap() == SwalDismissReason::Backdrop);
    /// ```
    pub fn canceled(reason: SwalDismissReason) -> Self {
        Self {
            is_confirmed: false,
            value: false,
            is_denied: false,
            is_dismissed: true,
            dismiss: Some(reason),
        }
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
#[derive(Debug, Clone, Copy)]
pub struct SwalOptions<S>
where
    S: AsRef<str> + Clone + Copy + Default + leptos::IntoView,
{
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

    /// Should the deny button be displayed?
    /// It defaults to `false`.
    pub show_deny_button: bool,

    /// Should the cancel button be displayed?
    /// It defaults to `false`.
    pub show_cancel_button: bool,

    /// The label of the confirmation button.
    /// Defaults to "Ok".
    pub confirm_button_text: S,

    /// The label of the cancel button.
    /// Defaults to "Cancel".
    pub cancel_button_text: S,

    /// The label of the deny button.
    /// Defaults to "Deny".
    pub deny_button_text: S,

    /// Function to execute before confirming.
    pub pre_confirm: fn(),

    /// Function to execute before denying.
    pub pre_deny: fn(),

    /// Function to execute when an alert ends.
    /// It will always get called no matter how
    /// the alert was dismissed.
    pub then: fn(SwalResult),

    /// Should the alert close itself when a button is pressed
    /// and when it is dismissed?
    /// Defaults to `true`.
    ///
    /// Use this carefully as preventing someone from dismissing
    /// a popup is considered bad practice, especially for
    /// accessibility concerns.
    pub auto_close: bool,

    /// Should animate the popup?
    /// A value of `false` will stop all animations,
    /// including the opening and closing transitions
    /// as well as the icon animations.
    /// Defaults to `true`.
    pub animation: bool,
}

impl<S> Default for SwalOptions<S>
where
    S: AsRef<str> + Clone + Copy + Default + leptos::IntoView,
{
    fn default() -> Self {
        Self {
            title: S::default(),
            text: S::default(),
            icon: &SwalIcon::NONE,
            show_confirm_button: true,
            show_deny_button: false,
            show_cancel_button: false,
            confirm_button_text: S::default(), // "Ok" is added maually
            cancel_button_text: S::default(),  // "Cancel" is added manually
            deny_button_text: S::default(),    // "Deny" is added manually
            pre_confirm: || {},
            pre_deny: || {},
            then: |_| {},
            auto_close: true,
            animation: true,
        }
    }
}

impl<S> SwalOptions<S>
where
    S: AsRef<str> + Clone + Copy + Default + leptos::IntoView,
{
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

    /// Creates Swal options for a simple alert with a title and an icon.
    /// All other parameters are set to their default values.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let opts = SwalOptions::basic_icon("This is a title", &SwalIcon::SUCCESS);
    /// assert_eq!(opts.title, "This is a title");
    /// assert_eq!(opts.icon, &SwalIcon::SUCCESS);
    /// ```
    pub fn basic_icon(title: S, icon: &'static SwalIcon) -> Self {
        Self {
            title,
            icon,
            ..Self::default()
        }
    }

    /// Creates Swal options for a simple alert with a title, a text and an icon.
    /// All other parameters are set to their default values.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let opts = SwalOptions::common("This is a title", "This is text", &SwalIcon::INFO);
    /// assert_eq!(opts.title, "This is a title");
    /// assert_eq!(opts.text, "This is text");
    /// assert_eq!(opts.icon, &SwalIcon::INFO);
    /// ```
    pub fn common(title: S, text: S, icon: &'static SwalIcon) -> Self {
        Self {
            title,
            text,
            icon,
            ..Self::default()
        }
    }

    /// Whether or not the current options have a title.
    pub fn has_title(&self) -> bool {
        !self.title.as_ref().is_empty()
    }

    /// Whether or not the current options have a text.
    pub fn has_text(&self) -> bool {
        !self.text.as_ref().is_empty()
    }

    /// Checks if the given text for the confirmation button is empty.
    /// If it's empty, it means the default value, "Ok", should be used instead.
    pub fn has_confirm_button_text(&self) -> bool {
        !self.confirm_button_text.as_ref().is_empty()
    }

    /// Checks if the given text for the deny button is empty.
    /// If it's empty, it means the default value, "Deny", should be used instead.
    pub fn has_deny_button_text(&self) -> bool {
        !self.deny_button_text.as_ref().is_empty()
    }

    /// Checks if the given text for the cancel button is empty.
    /// If it's empty, it means the default value, "Cancel", should be used instead.
    pub fn has_cancel_button_text(&self) -> bool {
        !self.cancel_button_text.as_ref().is_empty()
    }

    /// Whether or not the current options have an icon.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let opts = SwalOptions::basic_icon("This is a title", &SwalIcon::SUCCESS);
    /// assert_eq!(opts.icon, &SwalIcon::SUCCESS);
    /// assert!(opts.has_icon());
    ///
    /// let opts = SwalOptions::basic_icon("This is a title", &SwalIcon::NONE);
    /// assert!(!opts.has_icon());
    /// ```
    pub fn has_icon(&self) -> bool {
        self.icon != &SwalIcon::NONE
    }
}

#[allow(non_snake_case)]
pub mod Swal {
    use std::cell::RefCell;
    use std::time::Duration;

    use crate::{SwalDismissReason, SwalIcon, SwalResult};

    use super::SwalOptions;
    use leptos::html::{AnyElement, Div};
    use leptos::{set_timeout, *};
    use leptos_dom::HtmlElement;

    use web_sys::wasm_bindgen::JsCast;
    use web_sys::{window, Element, MouseEvent};

    #[allow(unused)]
    use log::info;

    thread_local! {
        /// The duration of the transition that opens and closes the swal.
        /// It is stored this way to avoid re-doing the calculations again and again
        /// every time that a swal is being opened or closed.
        static TRANSITION_DURATION: RefCell<f32> = const { RefCell::new(-1.0) };

        /// This is a copy of the "then" callback that was given to the current alert.
        /// The point of this variable is to be able to execute the callback when the alert
        /// gets closed by something other than the "Cancel" button.
        static THEN_CALLBACK: RefCell<Option<fn(SwalResult)>> = const { RefCell::new(None) };

        /// The "auto_close" parameter of the current options.
        /// It has to be saved because we want to stop the user
        /// from closing the popup even by pressing the Escape key
        /// or by clicking on the backdrop.
        static AUTO_CLOSE: RefCell<bool> = const { RefCell::new(true) };
    }

    /// Creates a Sweet Alert with the options defined in `opt`.
    /// See the docs for [SwalOptions](`#SwalOptions`) to know how to use it.
    pub fn fire<S>(opt: SwalOptions<S>)
    where
        S: AsRef<str> + Clone + Copy + Default + leptos::IntoView + 'static,
    {
        if let Some(swal) = get_swal() {
            // It has to be unsynced so that the current Swal can
            // finish closing and the DOM update itself.
            set_timeout(
                move || {
                    open(opt);
                },
                Duration::from_secs_f32(0.01 + get_transition_duration(&swal)),
            );
        } else {
            open(opt);
        }
    }

    /// Creates the Swal, adds it to the DOM and sets its aria-hidden
    /// attribute to "false" so that the animation can start once the
    /// DOM was updated.
    fn open<S>(opt: SwalOptions<S>)
    where
        S: AsRef<str> + Clone + Copy + Default + leptos::IntoView + 'static,
    {
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

    /// Allows the user to close the alert by pressing the Escape key.
    /// This method must be called only once, otherwise duplicated event
    /// listeners will be created and attached to the window, which is
    /// pointless and reduces performance.
    ///
    /// This method must be called in the main function of your program.
    ///
    /// It returns a handle that you can use to manually remove the event listener
    /// by calling `remove()` on the return value. You probably won't need it but it
    /// is there in case you need it.
    pub fn init_escape_key_handler() -> leptos_dom::helpers::WindowListenerHandle {
        window_event_listener(ev::keydown, |ev| {
            if is_swal_open() {
                let code = ev.code();
                if code.eq("Escape") {
                    if AUTO_CLOSE.with_borrow(|a| *a) {
                        close(Some(SwalResult::canceled(SwalDismissReason::Esc)));
                    }
                }
            }
        })
    }

    /// Checks if the Sweet Alert is currenlty open.
    pub fn is_swal_open() -> bool {
        get_swal().is_some()
    }

    /// Closes the alert and returns a boolean indicating if the action was successfull.
    /// It will return `false` if the alert isn't opened.
    /// It will trigger a copy of the current alert's "then" callback.
    ///
    /// Closing a popup without mentioning a result will not trigger the "then" callback.
    pub fn close(result: Option<SwalResult>) -> bool {
        if let Some(then) = THEN_CALLBACK.with_borrow(|t| *t) {
            if let Some(result) = result {
                (then)(result);
            }
            THEN_CALLBACK.with(|c| *c.borrow_mut() = None);
            AUTO_CLOSE.with(|a| *a.borrow_mut() = true);
        }
        if let Some(swal) = get_swal() {
            // Here the goal is to remove the swal from the DOM
            // as soon as the ending transition is over.
            // My solution is to extract the transition duration
            // from the computed styles and remove the node in a
            // delayed closure (via set_timeout from leptos).
            //
            // Initially I was going to listen to the "transitionend" event,
            // but WebAssembly's only solution in my case would leak memory,
            // as they so gently explain here:
            // https://rustwasm.github.io/wasm-bindgen/examples/closures.html#srclibrs
            // (which is awful and dumb)
            swal.set_attribute("aria-hidden", "true")
                .expect("Could not change the Swal's aria-hidden attribute.");
            set_timeout(
                || {
                    if let Some(swal) = get_swal() {
                        swal.remove()
                    }
                },
                Duration::from_secs_f32(get_transition_duration(&swal)),
            );
            true
        } else {
            false
        }
    }

    fn get_swal() -> Option<Element> {
        document().get_element_by_id("swal")
    }

    /// Gets the value of the "transition-duration" CSS property.
    /// It is used to remove the Swal from the DOM once the animation is over.
    fn get_transition_duration(el: &Element) -> f32 {
        let duration = TRANSITION_DURATION.with_borrow(|t| *t);
        if duration == -1.0 {
            let css_value = window()
                .expect("Could not get window")
                .get_computed_style(&el)
                .expect("Could not get computed style of Swal")
                .expect("Could not get computed style of Swal")
                .get_property_value("transition-duration");
            if let Ok(css_value) = css_value {
                let result = css_value
                    .get(0..css_value.len() - 1)
                    .expect("Invalid CSS value for transition duration of Swal")
                    .parse::<f32>()
                    .expect("Could not parse transition duration of Swal");
                TRANSITION_DURATION.with(|t| *t.borrow_mut() = result);
                result
            } else {
                0.0
            }
        } else {
            duration
        }
    }

    fn SwalComponent<S>(opt: SwalOptions<S>) -> HtmlElement<AnyElement>
    where
        S: AsRef<str> + Clone + Copy + Default + leptos::IntoView + 'static
    {
        let swal_container_ref = create_node_ref::<Div>();

        let on_backdrop_clicked = move |ev: MouseEvent| {
            if let Some(container) = swal_container_ref.get() {
                if let Some(target) = ev.target() {
                    let actual_target = target.dyn_ref::<web_sys::HtmlElement>();
                    if actual_target.is_some() {
                        if !container.contains(Some(actual_target.unwrap())) {
                            if AUTO_CLOSE.with_borrow(|a| *a) {
                                close(Some(SwalResult::canceled(SwalDismissReason::Backdrop)));
                            }
                        }
                    }
                }
            }
        };

        let has_icon = opt.has_icon();
        let has_text = opt.has_text();
        let then_callback = opt.then.clone();
        let auto_close = opt.auto_close.clone();

        // Here we copy the "then" callback and store it as a static variable.
        // The point of doing this is that it's the only way to detect whether or not
        // the Escape key was pressed (or if the backdrop was clicked) when closing the
        // alert. We need a way to execute this callback, and since it cannot be a
        // reference, a copy does the trick just fine.
        THEN_CALLBACK.with(move |t| *t.borrow_mut() = Some(then_callback));

        // We need to know if the developer has allowed
        // the Escape key and the backdrop to close the popup.
        AUTO_CLOSE.with(move |a| *a.borrow_mut() = auto_close);

        let on_confirm = move |_| { (opt.pre_confirm)(); if opt.auto_close { (opt.then)(SwalResult::confirmed()); close(None); }; };
        let on_deny = move |_| { (opt.pre_deny)(); if opt.auto_close { (opt.then)(SwalResult::denied()); close(None); }; };
        let on_cancel = move |_| { (opt.then)(SwalResult::canceled(SwalDismissReason::Cancel)); if opt.auto_close { close(None); }; };

        (view! {
            <div id="swal" on:click=on_backdrop_clicked class="swal-backdrop" class:swal-no-animation={!opt.animation} aria-hidden="true">
                <div _ref=swal_container_ref class="swal-container">
                    <Show when=move || has_icon>
                        <div class="swal-container-icon fade-icon">
                            {move || match opt.icon {
                                &SwalIcon::SUCCESS => SuccessIcon(),
                                &SwalIcon::WARNING => WarningIcon(),
                                &SwalIcon::ERROR => ErrorIcon(),
                                &SwalIcon::INFO => InfoIcon(),
                                &SwalIcon::QUESTION => QuestionIcon(),
                                _ => view! { <div /> }.into_any()
                            }}
                        </div>
                    </Show>
                    <strong>{opt.title}</strong>
                    <Show when=move || has_text>
                        <p>{opt.text}</p>
                    </Show>
                    <div>
                        <Show when=move || opt.show_confirm_button>
                            <button type="button" class="swal-confirm-button" on:click=on_confirm>
                                <Show when=move || { opt.has_confirm_button_text() } fallback=|| view! { "Ok" }>
                                    { opt.confirm_button_text }
                                </Show>
                             </button>
                        </Show>
                        <Show when=move || opt.show_deny_button>
                            <button type="button" class="swal-deny-button" on:click=on_deny>
                                <Show when=move || { opt.has_deny_button_text() } fallback=|| view! { "Deny" }>
                                    { opt.deny_button_text }
                                </Show>
                             </button>
                        </Show>
                        <Show when=move || opt.show_cancel_button>
                            <button type="button" class="swal-cancel-button" on:click=on_cancel>
                                <Show when=move || { opt.has_cancel_button_text() } fallback=|| view! { "Cancel" }>
                                    { opt.cancel_button_text }
                                </Show>
                             </button>
                        </Show>
                    </div>
                </div>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Swal component")
    }

    fn SuccessIcon() -> HtmlElement<AnyElement> {
        (view! {
            <svg
                class="success-icon"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg" stroke-linecap="round" fill-rule="evenodd">
              <g>
                <path vector-effect="non-scaling-stroke" d="M23.5 11.5C23.224 11.5 23 11.724 23 12C23 18.065 18.065 23 12 23C5.935 23 1 18.065 1 12C1 5.935 5.935 1 12 1C15.498 1 18.775 2.661 20.848 5.464L11.282 15.03L7.753 11.501C7.558 11.306 7.241 11.306 7.046 11.501C6.851 11.696 6.851 12.013 7.046 12.208L10.929 16.091C11.124 16.286 11.441 16.286 11.636 16.091L21.854 5.872C22.024 5.702 22.049 5.434 21.913 5.236C19.672 1.958 15.966 0 12 0C5.383 0 0 5.383 0 12C0 18.617 5.383 24 12 24C18.617 24 24 18.617 24 12C24 11.724 23.776 11.5 23.5 11.5Z"/>
              </g>
            </svg>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Success Icon")
    }

    fn WarningIcon() -> HtmlElement<AnyElement> {
        (view! {
            <div class="swal-rounded-icon swal-warning-icon">
                <svg viewBox="0 0 6 14" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M3 10C2.77157 10.0067 2.54774 9.93487 2.36579 9.79659C2.18385 9.65831 2.05475 9.46188 2 9.24L0.490001 3.18C0.399712 2.802 0.398797 2.40814 0.487327 2.02972C0.575857 1.6513 0.751395 1.29873 1 1C1.23982 0.699856 1.54411 0.457536 1.89033 0.291004C2.23655 0.124473 2.61581 0.038002 3 0.038002C3.38419 0.038002 3.76345 0.124473 4.10967 0.291004C4.45589 0.457536 4.76018 0.699856 5 1C5.24154 1.30421 5.40887 1.66052 5.48872 2.04066C5.56856 2.4208 5.55873 2.81432 5.46 3.19L4 9.24C3.94525 9.46188 3.81615 9.65831 3.63421 9.79659C3.45226 9.93487 3.22844 10.0067 3 10V10ZM3 2C2.91448 1.99921 2.82999 2.01874 2.75349 2.05699C2.677 2.09524 2.61068 2.15111 2.56 2.22C2.50515 2.28562 2.46732 2.36375 2.44988 2.44748C2.43243 2.5312 2.43591 2.61794 2.46 2.7L3 4.88L3.54 2.7C3.56409 2.61794 3.56757 2.5312 3.55013 2.44748C3.53268 2.36375 3.49486 2.28562 3.44 2.22C3.38932 2.15111 3.32301 2.09524 3.24651 2.05699C3.17001 2.01874 3.08552 1.99921 3 2V2Z" />
                    <path d="M3 14C3.82843 14 4.5 13.3284 4.5 12.5C4.5 11.6716 3.82843 11 3 11C2.17157 11 1.5 11.6716 1.5 12.5C1.5 13.3284 2.17157 14 3 14Z" />
                </svg>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Warning Icon")
    }

    fn ErrorIcon() -> HtmlElement<AnyElement> {
        (view! {
            <div class="swal-rounded-icon swal-error-icon">
                <svg viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M1.22178 16.7782C0.831183 16.3876 0.831183 15.7545 1.22178 15.3639L15.3639 1.2218C15.7544 0.8313 16.3876 0.8313 16.7781 1.2218C17.1686 1.6123 17.1686 2.2455 16.7781 2.636L2.63598 16.7782C2.24538 17.1687 1.61228 17.1687 1.22178 16.7782Z" />
                    <path d="M1.22183 1.2218C1.61243 0.8313 2.24553 0.8313 2.63613 1.2218L16.7782 15.364C17.1687 15.7545 17.1687 16.3876 16.7782 16.7782C16.3877 17.1687 15.7545 17.1687 15.364 16.7782L1.22183 2.636C0.83133 2.2455 0.83133 1.6123 1.22183 1.2218Z" />
                </svg>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Warning Icon")
    }

    fn InfoIcon() -> HtmlElement<AnyElement> {
        (view! {
            <div class="swal-rounded-icon swal-info-icon">
                <svg viewBox="0 0 4 8" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M1.3 1.69C1.20764 1.59889 1.1351 1.48969 1.08692 1.36924C1.03873 1.24878 1.01595 1.11967 1.02 0.990002C1.02 0.710002 1.11 0.470002 1.3 0.290002C1.49 0.110002 1.72 0.0100021 2 0.0100021C2.28 0.0100021 2.52 0.100002 2.7 0.290002C2.88 0.480002 2.98 0.710002 2.98 0.990002C2.98 1.27 2.89 1.51 2.7 1.69C2.51537 1.87842 2.26377 1.98624 2 1.99C1.72 1.99 1.48 1.88 1.3 1.69ZM3 3.99C2.98 3.74 2.89 3.51 2.69 3.3C2.49 3.11 2.27 3 2 2.99H1C0.73 3.01 0.52 3.12 0.31 3.3C0.11 3.5 0.01 3.74 0 3.99H1V6.99C1.02 7.26 1.11 7.49 1.31 7.68C1.51 7.88 1.73 7.99 2 7.99H3C3.27 7.99 3.48 7.88 3.69 7.68C3.89 7.49 3.99 7.26 4 6.99H3V3.98V3.99Z" />
                </svg>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Warning Icon")
    }

    fn QuestionIcon() -> HtmlElement<AnyElement> {
        (view! {
            <div class="swal-rounded-icon swal-question-icon">
                <svg viewBox="0 0 11 17" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M1 4C1 3.20435 1.36875 2.44129 2.02513 1.87868C2.6815 1.31607 3.57174 1 4.5 1H5.5C6.42826 1 7.3185 1.31607 7.97487 1.87868C8.63125 2.44129 9 3.20435 9 4C9.03682 4.64925 8.86168 5.2929 8.50096 5.83398C8.14024 6.37506 7.61347 6.78428 7 7C6.38653 7.28763 5.85976 7.83326 5.49904 8.5547C5.13832 9.27614 4.96318 10.1343 5 11" />
                    <path d="M5 15V15.01" />
                </svg>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Warning Icon")
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

    #[test]
    fn test_basic_icon() {
        let opts = SwalOptions::<&str>::basic_icon("Hello", &SwalIcon::ERROR);
        assert_eq!(opts.icon, &SwalIcon::ERROR);
    }

    #[test]
    fn test_common() {
        let opts = SwalOptions::<&str>::common("Hello", "World", &SwalIcon::ERROR);
        assert_eq!(opts.title, "Hello");
        assert_eq!(opts.text, "World");
        assert_eq!(opts.icon, &SwalIcon::ERROR);
    }

    #[test]
    fn test_has_icon() {
        let opts = SwalOptions::<&str>::basic_icon("Hello", &SwalIcon::SUCCESS);
        assert!(opts.has_icon());
        let opts = SwalOptions::<&str>::basic_icon("Hello", &SwalIcon::NONE);
        assert!(!opts.has_icon());
    }

    #[test]
    fn test_has_text() {
        let opts = SwalOptions::<&str>::common("Hello", "Some text", &SwalIcon::INFO);
        assert!(opts.has_text());
        let opts = SwalOptions::<&str>::basic("Hello");
        assert!(!opts.has_text());
    }

    #[test]
    fn test_has_title() {
        let opts = SwalOptions::<&str>::basic("Hello");
        assert!(opts.has_title());
        let opts = SwalOptions {
            title: "",
            text: "Hello",
            ..SwalOptions::default()
        };
        assert!(!opts.has_title());
    }

    // We make sure that this test works by panicking voluntarily.
    // It's the best way to know if the assert!(false) was called or not,
    // within the `pre_confirm` callback.
    #[test]
    #[should_panic]
    fn test_pre_confirm() {
        let opts = SwalOptions {
            title: "Confirm this!!",
            pre_confirm: || {
                assert!(false);
            },
            ..SwalOptions::default()
        };
        (opts.pre_confirm)();
    }
    
    #[test]
    #[should_panic]
    fn test_pre_deny() {
        let opts = SwalOptions {
            title: "Deny this!!",
            pre_deny: || {
                assert!(false);
            },
            ..SwalOptions::default()
        };
        (opts.pre_deny)();
    }
}
