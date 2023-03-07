use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("IO Errors:: {0}")]
    IoError(#[from] io::Error),
    #[error(transparent)]
    RunTimeError(#[from] anyhow::Error),
    #[error("unknown error")]
    UnKnown,
}