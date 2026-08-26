use katex::Opts;
use regex::{Captures, Regex};

pub fn render_katex_in_html(html: &str) -> String {
    let block_re = Regex::new(r#"(?:<pre>)?<code class="language-math math-(inline|display)">([\s\S]*?)</code>(?:</pre>)?"#).unwrap();
    block_re
        .replace_all(html, |caps: &Captures| {
            let is_display = &caps[1] == "display";
            let raw_math = caps[2]
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'");

            let opts = Opts::builder().display_mode(is_display).build().unwrap();

            match katex::render_with_opts(&raw_math, &opts) {
                Ok(rendered_html) => rendered_html,
                Err(e) => {
                    eprintln!("KaTeX parsing error: {}", e);
                    caps[0].to_string()
                }
            }
        })
        .to_string()
}
