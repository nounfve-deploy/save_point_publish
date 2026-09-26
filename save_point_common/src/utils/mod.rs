pub mod path_impl;

/// re export
pub mod external {
    pub use sutils::boilerplates::tokio_rt_singleton;
    pub use sutils::macros::command;
}