use regex::Regex;
use std::sync::OnceLock;

pub fn add_color_swatches(html: &str) -> String {
    static COLOR_RE: OnceLock<Regex> = OnceLock::new();
    let re = COLOR_RE.get_or_init(|| {
        Regex::new(r"(?i)(?P<open><code[^>]*>)(?P<color>#[0-9a-f]{3,8}|rgba?\([^)]+\)|hsla?\([^)]+\))(?P<close></code>)").unwrap()
    });

    re.replace_all(html, |caps: &regex::Captures| {
        let open = &caps["open"];
        let color = &caps["color"];
        let close = &caps["close"];
        format!(
            r#"{open}<span class="color-swatch" style="background-color: {color};"></span>{color}{close}"#
        )
    })
    .to_string()
}
