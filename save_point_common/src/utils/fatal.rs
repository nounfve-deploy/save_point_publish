pub trait TracingUnwarp<T> {
    fn or_fatal(self) -> T;
}

impl<T, E: Error> TracingUnwarp<T> for Result<T, E> {
    #[track_caller]
    fn or_fatal(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => fatal!("[unwrap] {err:?}\n  at: {}", std::panic::Location::caller()),
        }
    }
}

impl<T> TracingUnwarp<T> for Option<T> {
    #[track_caller]
    fn or_fatal(self) -> T {
        self.ok_or(io::Error::new(io::ErrorKind::Other, "option is none"))
            .or_fatal()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! fatal {
    ($($Any:tt)*) => {{

        tracing::error!($($Any)*);
        panic!("---------------fatal-error------------------");
    }};
}
#[doc(inline)]
pub use fatal;

use std::{error::Error, io};
