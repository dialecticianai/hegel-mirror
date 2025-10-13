use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn get_syntax(&self, lang: &str) -> &SyntaxReference {
        self.syntax_set
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme_set.themes["base16-ocean.dark"]
    }

    pub fn highlight_code(&self, code: &str, lang: &str) -> Vec<Vec<(Style, String)>> {
        let syntax = self.get_syntax(lang);
        let theme = self.get_theme();
        let mut highlighter = HighlightLines::new(syntax, theme);

        LinesWithEndings::from(code)
            .map(|line| {
                highlighter
                    .highlight_line(line, &self.syntax_set)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(style, text)| (style, text.to_string()))
                    .collect()
            })
            .collect()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
