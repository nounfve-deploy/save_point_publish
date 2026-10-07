#![allow(nonstandard_style)]

use sutils::{PutInMacro, Singleton, env_or, lazy_const};

#[Singleton]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Envar {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub ALT_MAIN: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub SP_IDENTITY: String,
}

impl Envar {
    pub const GLOBAL: &str = "env.global.yaml";
}

#[PutInMacro(lazy_const)]
pub const ENVAR_DEFAULT: Envar = Envar {
    ALT_MAIN: "main",
    SP_IDENTITY: "anonymous",
};

impl Default for Envar {
    fn default() -> Self {
        let Envar {
            ALT_MAIN,
            SP_IDENTITY,
        } = &*ENVAR_DEFAULT;
        let ALT_MAIN = env_or!(ALT_MAIN).into_owned();
        let SP_IDENTITY = env_or!(SP_IDENTITY).into_owned();
        Self {
            ALT_MAIN,
            SP_IDENTITY,
        }
    }
}
