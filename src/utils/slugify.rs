use regex::Regex;

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

pub fn strip_tags(html: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

pub fn add_heading_ids(html: &str) -> String {
    let re = Regex::new(r#"(?i)<(h[1-6])>(.*?)</h[1-6]>"#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let tag = &caps[1];
        let content = &caps[2];
        let plain_text = strip_tags(content);
        let id = slugify(&plain_text);
        format!(r#"<{tag} id="{id}">{content}</{tag}>"#)
    })
    .to_string()
}
