use leptos::*;
use leptos::html::AnyElement;
use leptos_dom::HtmlElement;

/// Defines the methods that a struct must have
/// to be considered as an Icon in the Swal.
///
/// Use it to create your own icons.
///
/// # Example
///
/// ```
/// # use leptos_sweetalert::*;
/// # use leptos::*;
/// use leptos_dom::html::{HtmlElement, AnyElement};
///
/// // It has to derive from Clone, Copy,
/// // and PartialEq, but not necessarily
/// // from Debug.
/// #[derive(Debug, PartialEq, Clone, Copy)]
/// pub struct CustomIcon(&'static str);
///
/// impl CustomIcon {
///     pub const MY_ICON: Self = Self("custom");
///
///     // The built-in icons return an svg,
///     // but you're free to return whatever you want,
///     // however you should take a look at the CSS.
///     fn get_custom_icon_html() -> HtmlElement<AnyElement> {
///         (view! { <div /> })
///         .into_view()
///         .into_html_element()
///         .unwrap()
///     }
/// }
///
/// impl SwalIconLike for CustomIcon {
///     fn get_icon_element(&self) -> HtmlElement<AnyElement> {
///         CustomIcon::get_custom_icon_html()
///         // If you had multiple icons, you'd match all of them.
///         // If you match all values but have to provide a default branch,
///         // then just return a div like I did above in the function.
///         // Note that `SwalIcon::none_icon()` is public.
///     }
///     
///     // Since an icon must have a default value,
///     // we need a way to know if a given value
///     // is the default one. Knowing that the default
///     // means "don't display an icon", then if this function
///     // returns "false", it means no icon should be displayed.
///     //
///     // Note: you don't have to implement it, since `true`
///     // is the default return value for this method.
///     //
///     // SwalIcon implements it and checks if `self` is
///     // `SwalIcon::NONE`. If it is, it returns `false`,
///     // otherwise it returns `true`.
///     fn is_defined(&self) -> bool {
///         true
///     }
/// }
///
/// // Since an Icon is not mandatory, but Rust needs a value,
/// // then we need to give Rust a default one in case none is
/// // provided manually.
/// impl Default for CustomIcon {
///     fn default() -> Self {
///         // It's quite useful to set your own default value,
///         // because in the case where you just have one icon
///         // in your struct, then if you just specify your
///         // struct type as the generic parameter `I`,
///         // then this icon here will be chosen if
///         // none is manually provided.
///         Self::MY_ICON
///     }
/// }
/// ```
pub trait SwalIconLike {
    /// The HTML Element of the icon to add into
    /// the view when building the Swal.
    fn get_icon_element(&self) -> HtmlElement<AnyElement>;

    /// Whether or not an icon should be displayed.
    /// If `self` corresponds to the default value,
    /// then it means no icon should be displayed.
    ///
    /// You don't have to implement this function,
    /// since it returns `true` by default.
    ///
    /// [SwalIcon](`#SwalIcon`) implements it and check
    /// if `self` is the same as `SwalIcon::NONE`.
    /// If it is, then don't display an icon (returning `false`).
    fn is_defined(&self) -> bool {
        true
    }
}

/// Defines an icon to be displayed in the Swal.
/// Use the pre-built ones to make sure you don't
/// accidentially make a mistake in the name of an icon.
#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn success_icon() -> HtmlElement<AnyElement> {
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

    fn warning_icon() -> HtmlElement<AnyElement> {
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

    fn error_icon() -> HtmlElement<AnyElement> {
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

    fn info_icon() -> HtmlElement<AnyElement> {
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

    fn question_icon() -> HtmlElement<AnyElement> {
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

    /// Displays nothing when no icon is needed.
    pub fn none_icon() -> HtmlElement<AnyElement> {
        (view! { <div /> })
            .into_view()
            .into_html_element()
            .expect("Could not display empty icon")
    }
}

impl std::fmt::Display for SwalIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SwalIconLike for SwalIcon {
    fn get_icon_element(&self) -> HtmlElement<AnyElement> {
        match self {
            &SwalIcon::SUCCESS => SwalIcon::success_icon(),
            &SwalIcon::WARNING => SwalIcon::warning_icon(),
            &SwalIcon::QUESTION => SwalIcon::question_icon(),
            &SwalIcon::ERROR => SwalIcon::error_icon(),
            &SwalIcon::INFO => SwalIcon::info_icon(),
            _ => SwalIcon::none_icon(),
        }
    }

    fn is_defined(&self) -> bool {
        self != &SwalIcon::NONE
    }
}

impl Default for SwalIcon {
    fn default() -> Self {
        SwalIcon::NONE
    }
}
