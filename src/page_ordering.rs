use std::collections::{BTreeMap, HashMap};

use crate::vault::Vault;

pub struct PageOrderManifest {
    pub ranks: HashMap<String, usize>,
}

impl PageOrderManifest {
    pub fn parse(content: &str) -> Self {
        let mut ranks = HashMap::new();
        let mut stack: Vec<(usize, String)> = Vec::new();
        let mut rank_counter = 0;

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let mut indent_len = 0;
            let mut start_idx = 0;
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                if chars[i].is_whitespace() {
                    indent_len += if chars[i] == '\t' { 4 } else { 1 };
                    i += 1;
                } else if chars[i] == '\\' && i + 1 < chars.len() && chars[i + 1] == 't' {
                    indent_len += 4;
                    i += 2;
                } else {
                    start_idx = i;
                    break;
                }
            }

            let clean_name = line[start_idx..].trim().trim_end_matches('/');
            let name_without_ext = if let Some(idx) = clean_name.rfind('.') {
                &clean_name[..idx]
            } else {
                clean_name
            };

            while let Some(&(level, _)) = stack.last() {
                if level >= indent_len {
                    stack.pop();
                } else {
                    break;
                }
            }

            // Convert path to lowercase for case-insensitive matching
            let path = if let Some((_, parent_path)) = stack.last() {
                format!("{}/{}", parent_path, name_without_ext).to_lowercase()
            } else {
                name_without_ext.to_lowercase()
            };

            ranks.insert(path.clone(), rank_counter);
            rank_counter += 1;
            stack.push((indent_len, path));
        }

        Self { ranks }
    }

    pub fn get_rank(&self, path: &str) -> Option<usize> {
        self.ranks.get(&path.to_lowercase()).copied()
    }
}

impl Vault {
    /// Generates a complete, static manifest configuration string from the current pages.
    /// This scaffolds the initial file that users will manually edit.
    pub fn generate_static_manifest(&self) -> String {
        let mut root_files: Vec<String> = Vec::new();
        let mut categories: BTreeMap<String, Vec<String>> = BTreeMap::new();

        let get_clean_name = |name: &str| -> String {
            if let Some(idx) = name.rfind('.') {
                name[..idx].to_lowercase()
            } else {
                name.to_lowercase()
            }
        };

        let get_clean_cat = |cat: &Option<String>| -> Option<String> {
            let cleaned = cat
                .as_ref()
                .map(|c| c.replace('\\', "/").trim_matches('/').to_lowercase())
                .filter(|c| !c.is_empty())?;

            if cleaned == "html" {
                None
            } else {
                Some(cleaned)
            }
        };

        for page in &self.pages {
            let clean_name = get_clean_name(&page.metadata.file_name);
            let cat = get_clean_cat(&page.metadata.category);

            if clean_name == "index" {
                if let Some(c) = cat {
                    categories.entry(c).or_default();
                }
                continue;
            }

            if let Some(c) = cat {
                categories.entry(c).or_default().push(clean_name);
            } else {
                root_files.push(clean_name);
            }
        }
        root_files.sort();

        for files in categories.values_mut() {
            files.sort_by(|a, b| {
                let a_is_special = a == "overview";
                let b_is_special = b == "overview";

                if a_is_special && !b_is_special {
                    std::cmp::Ordering::Less
                } else if !a_is_special && b_is_special {
                    std::cmp::Ordering::Greater
                } else {
                    a.cmp(b)
                }
            });
        }

        let mut manifest_str = String::new();
        for root in root_files {
            manifest_str.push_str(&format!("{}\n", root));
        }
        for (cat, files) in categories {
            manifest_str.push_str(&format!("{}\n", cat));
            for file in files {
                manifest_str.push_str(&format!("    {}\n", file));
            }
        }

        manifest_str
    }
}
