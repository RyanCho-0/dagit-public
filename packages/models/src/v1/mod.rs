pub mod agit_admins;
pub mod agits;
pub mod artists;
pub mod artworks;
pub mod collections;
pub mod users;

pub mod prelude {
    pub use super::agit_admins::*;
    pub use super::agits::*;
    pub use super::artists::*;
    pub use super::artworks::*;
    pub use super::collections::*;
    pub use super::users::*;
}
