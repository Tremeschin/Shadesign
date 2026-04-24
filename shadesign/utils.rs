use crate::*;

/// Common progress bar style
pub fn progress(message: &str) -> ProgressStyle {
    ProgressStyle::default_bar().template(
        &format!("{message} ({{elapsed_precise}} • ETA {{eta_precise}}) {{wide_bar:.cyan/blue}} ({{percent_precise}}%) • {{pos}}/{{len}} ({{per_sec:0.}})")).unwrap()
        .progress_chars("##•")
}
