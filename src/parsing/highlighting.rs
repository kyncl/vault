use regex::Regex;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// Takes html and injects highlighting into code blocks
/// Extra await and async keyword modification
pub fn highlight_html_blocks(html_input: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let re =
        Regex::new(r#"(?s)<pre><code(?: class="language-([^"]+)")?>(.*?)</code></pre>"#).unwrap();
    let syntax_fix_re =
        Regex::new(r#"(<span class="comment">.*?</span>)|(<[^>]+>)|(\bawait\b)|(\basync\b)"#)
            .unwrap();

    let output = re.replace_all(html_input, |caps: &regex::Captures| {
        let lang = caps.get(1).map_or("txt", |m| m.as_str());
        let raw_code = caps.get(2).unwrap().as_str();

        let unescaped_code = raw_code
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#x27;", "'")
            .replace("&#x60;", "`");

        let syntax = ss
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| ss.find_syntax_plain_text());

        let mut html_generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::Spaced);

        for line in LinesWithEndings::from(&unescaped_code) {
            html_generator
                .parse_html_for_line_which_includes_newline(line)
                .unwrap();
        }

        let highlighted = html_generator.finalize();
        let highlighted = syntax_fix_re
            .replace_all(&highlighted, |inner_caps: &regex::Captures| {
                if let Some(comment_block) = inner_caps.get(1) {
                    comment_block.as_str().to_string()
                } else if let Some(html_tag) = inner_caps.get(2) {
                    html_tag.as_str().to_string()
                } else if inner_caps.get(3).is_some() {
                    r#"<span class="keyword control await">await</span>"#.to_string()
                } else {
                    r#"<span class="keyword storage async">async</span>"#.to_string()
                }
            })
            .to_string();

        format!(
            r#"<pre><code class="language-{}">{}</code></pre>"#,
            lang, highlighted
        )
    });

    output.to_string()
}
