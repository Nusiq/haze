use miette::Diagnostic;
use std::{io, path::PathBuf};
use thiserror::Error;

/// The library error type.
#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("cannot read the config file at `{1}`")]
    #[diagnostic(help("{0}"))]
    CannotReadConfig(io::ErrorKind, String),

    #[error("the config file at `{1}` was not parsed")]
    #[diagnostic(help("{0}"))]
    CannotParseConfig(serde_json::Error, String),

    #[error("the `worlds` config property is empty")]
    #[diagnostic(help("the property must include at least one pattern"))]
    EmptyWorldsProperty,

    #[error("invalid glob pattern in `worlds` property `{1}`")]
    #[diagnostic(help("{0}"))]
    InvalidWorldsGlobPattern(glob::PatternError, String),

    #[error("cannot find the world `{1}` in local worlds directory")]
    #[diagnostic(help("available worlds: {0:?}"))]
    WorldNotFound(Vec<PathBuf>, String),

    #[error("unable to find the local appdata directory")]
    CannotFindLocalAppData(),

    #[error("cannot copy the world `{1}`")]
    #[diagnostic(help("{0}"))]
    CannotCopyWorld(io::ErrorKind, String),

    #[error("cannot overwrite the world `{0}` as it already exists in `minecraftWorlds`")]
    #[diagnostic(help("do \"haze test --overwrite {0}\" if you want to overwrite it"))]
    CannotOverwriteWorld(String),

    #[error("cannot delete directory `{0}` because permission to access some of its contents was denied")]
    #[diagnostic(help("Make sure that the directory is not in use by another program"))]
    CannotDeleteDirectory(String),

    #[error(
        "failed while trying to delete directory `{0}` and some of its contents may still exist"
    )]
    #[diagnostic(help("Make sure that the directory is not in use by another program"))]
    CriticalDeleteDirectory(String),

    #[error("cannot access all of the files in the directory `{0}`")]
    #[diagnostic(help("Make sure that the directory is not in use by another program"))]
    CannotAccessDirectory(String),
}
