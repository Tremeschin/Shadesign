#![allow(non_snake_case)]
pub use std::cmp::Ordering;
pub use std::cmp::max;
pub use std::cmp::min;
pub use std::fmt::Display;
pub use std::fs::File;
pub use std::fs::read;
pub use std::fs::read_to_string;
pub use std::io::BufRead;
pub use std::io::BufReader;
pub use std::path::Path;
pub use std::path::PathBuf;
pub use std::sync::LazyLock;
pub use std::sync::OnceLock;

pub use anyhow::Result;
pub use anyhow::bail;
pub use clap::Parser;
pub use clap::Subcommand;
pub use clap::ValueEnum;
pub use flate2::read::GzDecoder;
pub use indicatif::ParallelProgressIterator;
pub use indicatif::ProgressBar;
pub use indicatif::ProgressStyle;
pub use rayon::prelude::*;
pub use rust_embed::Embed;
pub use serde::Deserialize;
pub use serde::Serialize;
pub use smart_default::SmartDefault;

pub mod commands;
pub mod score;
pub mod utils;
pub use commands::*;
pub use score::*;
pub use utils::*;
