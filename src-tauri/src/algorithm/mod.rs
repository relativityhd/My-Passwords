mod legacy;
mod secure;
mod supersecure;

pub use legacy::gen_legacy_pw;
pub use secure::gen_pw;
pub use supersecure::gen_super_pw;
