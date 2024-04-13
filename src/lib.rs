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
