pub const DEFAULT_COUNT_BEFORE: u32 = 1_000;
pub const DEFAULT_COUNT_AFTER: u32 = 1_000;
/// In \[ms].
pub const DEFAULT_TIME_BEFORE: u32 = 5_000;
/// In \[ms].
pub const DEFAULT_TIME_AFTER: u32 = 5_000;
pub const DEFAULT_RETRIGGER: bool = false;
pub const DEFAULT_MAX_RETRIGGERS: u32 = 10;

/// In \[MB].
pub const DEFAULT_FILE_SIZE: u32 = 10;

pub const DEFAULT_FILE: &str =
"# Quotes are not necessary.\n\
out_dir = .\n\
# Variants: txt, text, csv, json; case insensitive\n\
out_format = txt\n\
\n\
interfaces = eth0, wlan0, lo\n\
# BPF (this filters for PTP)\n\
filter = ether proto 0x88f7\n\
\n\
# trigerror supports underscores for number group separators.
count_before = 1_000\n\
time_before = 5_000\n\
count_after = 1_000\n\
time_after = 5_000\n\
\n\
retrigger = false\n\
max_retriggers = 10\n\
";

pub const MARGIN: f64 = 0.3;
