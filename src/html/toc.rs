use regex::Regex;

use crate::utils::slugify::strip_tags;

pub fn generate_toc(html: &str) -> String {
    let re = Regex::new(r#"(?i)<(h[1-6]) id="([^"]+)">(.*?)</h[1-6]>"#).unwrap();
    let mut toc_html =
        String::from(r#"<div class="toc-title">On this page</div><ul class="toc-list">"#);

    for caps in re.captures_iter(html) {
        let tag = &caps[1];
        let id = &caps[2];
        let text = strip_tags(&caps[3]);
        let level_class = match tag {
            "h3" => "toc-item-h3",
            "h4" => "toc-item-h4",
            _ => "toc-item",
        };

        toc_html.push_str(&format!(
            "<li class='{}'><a href='#{}' class='toc-link'>{}</a></li>",
            level_class, id, text
        ));
    }
    toc_html.push_str("</ul>");
    toc_html
}
