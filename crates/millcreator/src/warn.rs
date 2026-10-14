//! A warning of the program: an English text with `{}` for the details, translated when it is shown.
use base_ui::i18n::{fill, tr};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Warn {
    msg: &'static str,
    args: Vec<String>,
}

impl Warn {
    pub fn new(msg: &'static str, args: &[&dyn Display]) -> Warn {
        Warn { msg, args: args.iter().map(|a| a.to_string()).collect() }
    }

    /// The warning in the language of the program.
    pub fn text(&self) -> String {
        let args: Vec<&dyn Display> = self.args.iter().map(|a| a as &dyn Display).collect();
        fill(tr(self.msg), &args)
    }
}

/// The English text, for tests and logs.
impl Display for Warn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args: Vec<&dyn Display> = self.args.iter().map(|a| a as &dyn Display).collect();
        f.write_str(&fill(self.msg, &args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_details_are_put_in_and_the_english_text_is_kept() {
        let w = Warn::new("{}: no bit chosen", &[&"Cut out"]);
        assert_eq!(w.to_string(), "Cut out: no bit chosen");
        assert_eq!(w.text(), "Cut out: no bit chosen");
    }
}
