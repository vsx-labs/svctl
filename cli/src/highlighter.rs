use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use syntect::util::LinesWithEndings;

///
/// themes: [
///     "InspiredGitHub",
///     "Solarized (dark)",
///     "Solarized (light)",
///     "base16-eighties.dark",
///     "base16-mocha.dark",
///     "base16-ocean.dark",
///     "base16-ocean.light",
/// ]
///
pub struct SyntaxHighlighter {
    ps: SyntaxSet,
    ts: ThemeSet,
    highlighting_theme: String,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            ts: ThemeSet::load_defaults(),
            ps: SyntaxSet::load_defaults_newlines(),
            highlighting_theme: "base16-ocean.dark".to_string(),
        }
    }

    pub fn format(&self, filename: &str, code: &String) -> String {
        return self.format_lines(filename, code).join("");
    }

    pub fn format_lines(&self, filename: &str, code: &String) -> Vec<String> {
        let syntax = self
            .ps
            .find_syntax_by_extension(&filename)
            // .unwrap_or_else(|| self.ps.find_syntax_plain_text());
            .unwrap();
        let theme = &self.ts.themes[&self.highlighting_theme];
        let mut h = HighlightLines::new(syntax, theme);
        let lines = LinesWithEndings::from(&code)
            .map(|line| h.highlight_line(line, &self.ps))
            .filter_map(Result::ok)
            .map(|ranges| as_24_bit_terminal_escaped(&ranges, true))
            .collect::<Vec<_>>();

        lines
    }
}
