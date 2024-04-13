//! This is a remake of SweetAlert for the Leptos web framework.
//! It was not made by SweetAlert but by someone that is not related to the project in any way.
//!
//! Many features have been implemented, but not all of them. To learn more about how to use this
//! create, then please read the documentation provided on [GitHub](https://github.com/ThomasGysemans/leptos_sweetalert).

mod swal_dismiss_reason;
mod swal_icon;
mod swal_options;
mod swal_result;

#[allow(non_snake_case)]
pub mod Swal;

pub use swal_dismiss_reason::SwalDismissReason;
pub use swal_icon::SwalIcon;
pub use swal_icon::SwalIconLike;
pub use swal_options::SwalOptions;
pub use swal_result::SwalResult;

mod tests;
