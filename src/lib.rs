mod swal_dismiss_reason;
mod swal_icon;
mod swal_options;
mod swal_result;

pub use swal_dismiss_reason::SwalDismissReason;
pub use swal_icon::SwalIcon;
pub use swal_icon::SwalIconLike;
pub use swal_options::SwalOptions;
pub use swal_result::SwalResult;

#[allow(non_snake_case)]
pub mod Swal {
    use std::cell::RefCell;
    use std::time::Duration;

    use crate::{SwalDismissReason, SwalIconLike, SwalResult};

    use super::SwalOptions;
    use leptos::html::{AnyElement, Div};
    use leptos::{set_timeout, *};
    use leptos_dom::HtmlElement;

    use web_sys::wasm_bindgen::JsCast;
    use web_sys::{window, Element, HtmlCollection, MouseEvent};

    #[allow(unused)]
    use log::info;

    thread_local! {
        /// The duration of the transition that opens and closes the swal.
        /// It is stored this way to avoid re-doing the calculations
        /// every time that a swal is being opened or closed.
        static TRANSITION_DURATION: RefCell<f32> = const { RefCell::new(-1.0) };

        /// This is a copy of the "then" callback that was given to the current alert.
        /// The point of this variable is to be able to execute the callback when the alert
        /// gets closed by the Escape key or by clicking on the backdrop.
        static THEN_CALLBACK: RefCell<Option<fn(SwalResult)>> = const { RefCell::new(None) };

        /// The "auto_close" parameter of the current options.
        static AUTO_CLOSE: RefCell<bool> = const { RefCell::new(true) };

        /// The element that had the focus before opening the Swal.
        static PREVIOUSLY_FOCUSED: RefCell<Option<web_sys::HtmlElement>> = const { RefCell::new(None) };
    }

    /// Creates a Sweet Alert with the options defined in `opt`.
    /// See the docs for [`SwalOptions`] to know how to use it.
    pub fn fire<S, I>(opt: SwalOptions<S, I>)
    where
        S: AsRef<str> + Clone + Default + leptos::IntoView + 'static,
        I: SwalIconLike + Default + Clone + Copy + 'static,
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
    fn open<S, I>(opt: SwalOptions<S, I>)
    where
        S: AsRef<str> + Clone + Default + leptos::IntoView + 'static,
        I: SwalIconLike + Default + Clone + Copy + 'static,
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
        if let Some(active_element) = get_active_element() {
            set_previously_focused_element(active_element);
        }
        set_timeout(
            || {
                get_swal()
                    .unwrap()
                    .set_attribute("aria-hidden", "false")
                    .expect("Could not set aria-hidden of Swal");
                let focusables = get_focusables();
                if focusables.len() > 0 {
                    focusables[0]
                        .focus()
                        .expect("Could not focus first button of Swal");
                }
            },
            Duration::from_secs_f32(0.01),
        );
    }

    /// Allows the user to close the alert by pressing the Escape key.
    /// It also holds the focus within the swal, preventing the user from
    /// focusing elements that are not inside the alert.
    ///
    /// This method must be called only once, otherwise duplicated event
    /// listeners will be created and attached to the window, which is
    /// pointless and reduces performance.
    ///
    /// This method must be called in the main function of your program.
    ///
    /// It returns a handle that you can use to manually remove the event listener
    /// by calling `remove()` on the return value. You probably won't need it but it
    /// is there in case you need it.
    pub fn init_key_handlers() -> leptos_dom::helpers::WindowListenerHandle {
        window_event_listener(ev::keydown, |ev| {
            if is_open() {
                let code = ev.code();
                if code.eq("Escape") {
                    if AUTO_CLOSE.with_borrow(|a| *a) {
                        close(Some(SwalResult::canceled(SwalDismissReason::Esc)));
                    }
                } else if code.eq("Tab") {
                    let focusables = get_focusables();
                    if focusables.len() == 0 {
                        return;
                    }

                    let mut index: usize = 0;
                    if let Some(active_element) = document().active_element() {
                        let active_element = active_element
                            .dyn_ref::<web_sys::HtmlElement>()
                            .expect("Invalid active element");
                        for i in 0..focusables.len() {
                            if focusables[i].is_same_node(Some(active_element)) {
                                index = i;
                                break;
                            }
                        }
                    }

                    ev.prevent_default();

                    if ev.shift_key() {
                        if index == 0 {
                            index = focusables.len() - 1;
                        } else {
                            index -= 1;
                        }
                    } else {
                        index = (index + 1) % focusables.len();
                    }

                    focusables[index]
                        .focus()
                        .expect("Could not focus next element");
                }
            }
        })
    }

    /// Gets the active element, meaning the element that has the focus.
    /// It returns a [`web_sys::HtmlElement`] so as to be able to focus it again.
    pub fn get_active_element() -> Option<web_sys::HtmlElement> {
        let active = document().active_element();
        if let Some(active) = active {
            match active.dyn_into::<web_sys::HtmlElement>() {
                Ok(valid) => Some(valid),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// Sets the element that should receive the focus when the Swal closes.
    pub fn set_previously_focused_element(element: web_sys::HtmlElement) {
        PREVIOUSLY_FOCUSED.with(|c| *c.borrow_mut() = Some(element));
    }

    /// Forgets the element that should receive the focus when the Swal closes.
    pub fn forget_previously_focused_element() {
        PREVIOUSLY_FOCUSED.with(|c| *c.borrow_mut() = None);
    }

    /// Checks if the Sweet Alert is currently open.
    pub fn is_open() -> bool {
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
            PREVIOUSLY_FOCUSED.with(|c| {
                let elt = c.borrow();
                if elt.is_some() {
                    let _ = elt.as_ref().unwrap().focus();
                    drop(elt); // to avoid double borrow
                    *c.borrow_mut() = None;
                }
            });
            true
        } else {
            false
        }
    }

    /// Gets the WebSys Element for the popup from the DOM.
    pub fn get_swal() -> Option<Element> {
        document().get_element_by_id("swal")
    }

    /// Gets the WebSys HtmlCollection for the confirmation button from the DOM.
    /// It returns an HtmlCollection because the button has a class, and by
    /// definition a class can be attached to several elements, therefore
    /// there is no other way that returning an HtmlCollection.
    /// If things are done right, then this should return a
    /// collection of one element, and this element should be
    /// the expected button.
    pub fn get_confirm_button() -> HtmlCollection {
        document().get_elements_by_class_name("swal-confirm-button")
    }

    /// Gets the WebSys HtmlCollection for the "deny" button from the DOM.
    /// It returns an HtmlCollection because the button has a class, and by
    /// definition a class can be attached to several elements, therefore
    /// there is no other way that returning an HtmlCollection.
    /// If things are done right, then this should return a
    /// collection of one element, and this element should be
    /// the expected button.
    pub fn get_deny_button() -> HtmlCollection {
        document().get_elements_by_class_name("swal-deny-button")
    }

    /// Gets the WebSys HtmlCollection for the "cancel" button from the DOM.
    /// It returns an HtmlCollection because the button has a class, and by
    /// definition a class can be attached to several elements, therefore
    /// there is no other way that returning an HtmlCollection.
    /// If things are done right, then this should return a
    /// collection of one element, and this element should be
    /// the expected button.
    pub fn get_cancel_button() -> HtmlCollection {
        document().get_elements_by_class_name("swal-cancel-button")
    }

    /// Gets the focusable buttons in the Swal.
    pub fn get_focusables() -> Vec<web_sys::HtmlElement> {
        let mut vec = Vec::new();
        if let Some(swal) = get_swal() {
            let all = swal
                .query_selector_all("*:is(a[href], button, input, textarea, select, details):not([disabled]):not([aria-hidden=true]):not([inert]), [tabindex]:not([tabindex='-1'])")
                .expect("Could not retrieve the focusable elements");
            for i in 0..all.length() {
                let el = all
                    .get(i)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap();
                if !has_display_none(&el) {
                    vec.push(el);
                }
            }
        }
        vec
    }

    fn has_display_none(element: &web_sys::HtmlElement) -> bool {
        element
            .style()
            .get_property_value("display")
            .unwrap_or("none".to_string())
            == "none"
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

    fn SwalComponent<S, I>(opt: SwalOptions<S, I>) -> HtmlElement<AnyElement>
    where
        S: AsRef<str> + Clone + Default + leptos::IntoView + 'static,
        I: SwalIconLike + Default + Clone + Copy + 'static,
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

        let then_callback = opt.then.clone();
        let auto_close = opt.auto_close.clone();

        let has_icon = opt.icon.is_defined();
        let has_text = opt.has_text();
        let has_confirm_btn_text = opt.has_confirm_button_text();
        let has_deny_btn_text = opt.has_deny_button_text();
        let has_cancel_btn_text = opt.has_cancel_button_text();

        // Here we copy the "then" callback and store it as a static variable.
        // The point of doing this is that it's the only way to detect whether or not
        // the Escape key was pressed (or if the backdrop was clicked) when closing the
        // alert. We need a way to execute this callback, and since it cannot be a
        // reference, a copy does the trick just fine.
        THEN_CALLBACK.with(move |t| *t.borrow_mut() = Some(then_callback));

        // We need to know if the developer has allowed
        // the Escape key and the backdrop to close the popup.
        AUTO_CLOSE.with(move |a| *a.borrow_mut() = auto_close);

        let on_confirm = move |_| {
            (opt.pre_confirm)();
            if opt.auto_close {
                (opt.then)(SwalResult::confirmed());
                close(None);
            };
        };

        let on_deny = move |_| {
            (opt.pre_deny)();
            if opt.auto_close {
                (opt.then)(SwalResult::denied());
                close(None);
            };
        };

        let on_cancel = move |_| {
            (opt.then)(SwalResult::canceled(SwalDismissReason::Cancel));
            if opt.auto_close {
                close(None);
            };
        };

        (view! {
            <div
                role="dialog"
                aria-modal="true"
                aria-labelledby="swal-title"
                id="swal"
                on:click=on_backdrop_clicked
                class="swal-backdrop"
                class:swal-no-animation={!opt.animation}
                aria-hidden="true"
            >
                <div _ref=swal_container_ref class="swal-container">
                    <Show when=move || has_icon>
                        <div class="swal-container-icon fade-icon">
                            {opt.icon.get_icon_element()}
                        </div>
                    </Show>
                    <strong id="swal-title">{opt.title}</strong>
                    <Show when=move || has_text>
                        <p>{opt.text.clone()}</p>
                    </Show>
                    {opt.body}
                    <div>
                        {match opt.show_confirm_button {
                            true => view! {
                                <button type="button" class="swal-confirm-button" on:click=on_confirm>
                                    <Show when=move || { has_confirm_btn_text } fallback=|| view! { "Ok" }>
                                        { opt.confirm_button_text.clone() }
                                    </Show>
                                 </button>
                            }.into_view(),
                            false => view! {}.into_view(),
                        }}
                        {match opt.show_deny_button {
                            true => view! {
                                <button type="button" class="swal-deny-button" on:click=on_deny>
                                    <Show when=move || { has_deny_btn_text } fallback=|| view! { "Deny" }>
                                        { opt.deny_button_text.clone() }
                                    </Show>
                                 </button>
                            }.into_view(),
                            false => view! {}.into_view(),
                        }}
                        {match opt.show_cancel_button {
                            true => view! {
                                <button type="button" class="swal-cancel-button" on:click=on_cancel>
                                    <Show when=move || { has_cancel_btn_text } fallback=|| view! { "Cancel" }>
                                        { opt.cancel_button_text.clone() }
                                    </Show>
                                 </button>
                            }.into_view(),
                            false => view! {}.into_view(),
                        }}
                    </div>
                </div>
            </div>
        })
        .into_view()
        .into_html_element()
        .expect("Could not create Swal component")
    }
}

mod tests;
