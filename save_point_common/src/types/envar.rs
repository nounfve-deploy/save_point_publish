use std::borrow::Cow;

use sutils::{Singleton, env_or};

#[Singleton]
pub struct Envar {
    pub alt_main: Cow<'static, str>,
    pub identity: Cow<'static, str>,
}

const ALT_MAIN: &str = "main";
const SP_IDENTITY: &str = "";

impl Default for Envar {
    fn default() -> Self {
        let alt_main = env_or!(ALT_MAIN);
        let identity = env_or!(SP_IDENTITY);
        Self { alt_main, identity }
    }
}
