/// The reasons why an alert has been closed.
#[derive(Debug, PartialEq)]
pub enum SwalDismissReason {
    /// The user clicked the backdrop.
    Backdrop,

    /// The user clicked the cancel button.
    Cancel,

    /// The user clicked the close button.
    ///
    /// This member is actually not used by
    /// this crate, but it is provided for
    /// you if you decide to manually close
    /// the alert for some reason, and need
    /// a way to detect that you did that
    /// programmatically.
    Close,

    /// The user clicked the Escape key.
    Esc,
}
