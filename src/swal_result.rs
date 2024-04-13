use crate::SwalDismissReason;

/// The data that is returned when an alert is closed.
#[derive(Debug)]
pub struct SwalResult {
    /// The "Confirm" button was clicked, the value will contain the result.
    pub is_confirmed: bool,

    /// The "Deny" button was clicked, the value will be false.
    pub is_denied: bool,

    /// The "Cancel" button was clicked, the dismiss will be
    /// [`SwalDismissReason.Cancel`]
    pub is_dismissed: bool,

    /// The value from the popup, possible values:
    /// - `true` for simple confirmed dialogs
    /// - `false` for denied popups
    pub value: bool,

    /// The dismissal reason, see [`SwalDismissReason`].
    /// It's optional because if the popup is confirmed or denied, then it wasn't dismissed,
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
