use minify_html::{Cfg, minify};

pub fn minify_html(html: &str) -> String {
    let cfg = Cfg {
        keep_comments: false,
        minify_css: true,
        minify_js: true,
        keep_ssi_comments: false,
        ..Default::default()
    };

    let minified = minify(html.as_bytes(), &cfg);
    String::from_utf8(minified).unwrap_or_else(|_| html.to_string())
}
