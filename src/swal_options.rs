use crate::SwalIcon;
use crate::SwalIconLike;
use crate::SwalResult;
use leptos::*;

/// Defines the parameters of a Sweet Alert.
/// It uses generic parameters to simplify the use
/// of the crate. The first one (`S`) defines the type for all
/// text fields and the second one (`I`) defines the type for the icon.
///
/// # Example
///
/// ```
/// # use leptos_sweetalert::*;
///
/// // Use methods to reduce the amount
/// // of fields you have to manually assign
/// //
/// // The default types of the generic parameters
/// // cannot be used if at least one of them isn't
/// // explicitly defined. Therefore, if you do not
/// // specify an icon, then you have to use the syntax below.
/// // The default icon is `SwalIcon::NONE` (which displays no icon).
/// let opt = SwalOptions::<&str>::basic("This is a title");
///
/// // Look at the methods for a better developer experience.
/// // However, if you want to fully customize the parameters
/// // in a way that the methods doesn't allow you to, then use:
/// let opt = SwalOptions::<&str> {
///     // This is the exact same as using the `basic` method.
///     title: "This is a title",
///     ..SwalOptions::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SwalOptions<S = &'static str, I = SwalIcon>
where
    S: AsRef<str> + Clone + Default + leptos::IntoView,
    I: SwalIconLike + Default + Clone + Copy,
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
    /// the value of this property is `SwalIcon::NONE`.
    pub icon: I,

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
    /// It will not get called if no reason was given to the
    /// `Swal::close()` method (which allows you to close the popup programmatically).
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

    /// A custom view to be added into the generated HTML of the popup.
    /// This view is inserted below the description and above the buttons.
    pub body: View,
}

impl<S, I> Default for SwalOptions<S, I>
where
    S: AsRef<str> + Clone + Default + leptos::IntoView,
    I: SwalIconLike + Default + Clone + Copy,
{
    fn default() -> Self {
        Self {
            title: S::default(),
            text: S::default(),
            icon: I::default(),
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
            body: View::default(),
        }
    }
}

impl<S, I> SwalOptions<S, I>
where
    S: AsRef<str> + Clone + Default + leptos::IntoView,
    I: SwalIconLike + Default + Clone + Copy,
{
    /// Creates Swal options for a simple alert with just a title.
    /// All other parameters are set to their default values.
    ///
    /// # Example
    ///
    /// ```
    /// # use leptos_sweetalert::*;
    ///
    /// let opts = SwalOptions::<&str>::basic("This is a title");
    /// assert_eq!(opts.title, "This is a title");
    /// assert_eq!(opts.text, ""); // see default values.
    /// assert_eq!(opts.icon, SwalIcon::NONE);
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
    /// let opts = SwalOptions::basic_icon("This is a title", SwalIcon::SUCCESS);
    /// assert_eq!(opts.title, "This is a title");
    /// assert_eq!(opts.icon, SwalIcon::SUCCESS);
    /// ```
    pub fn basic_icon(title: S, icon: I) -> Self {
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
    /// let opts = SwalOptions::common("This is a title", "This is text", SwalIcon::INFO);
    /// assert_eq!(opts.title, "This is a title");
    /// assert_eq!(opts.text, "This is text");
    /// assert_eq!(opts.icon, SwalIcon::INFO);
    /// ```
    pub fn common(title: S, text: S, icon: I) -> Self {
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
}
