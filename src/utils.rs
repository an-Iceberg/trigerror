// Place most of lib.rs here.

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Default, Debug)]
pub enum OutFormat { #[default] Text, CSV, JSON }
