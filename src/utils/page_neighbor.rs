use crate::{page::PageMetadata, vault::Vault};

impl Vault {
    pub fn set_neighbors(&mut self) -> &mut Self {
        if self.pages.is_empty() {
            eprintln!("Pages are empty. Did you chain correctly?");
        }
        let total_pages = self.pages.len();
        for i in 0..total_pages {
            let previous = if i > 0 {
                Some(PageNeighbor {
                    metadata: self.pages[i - 1].metadata.clone(),
                    rel_html_path: self.pages[i - 1].metadata.rel_html_path.clone(),
                })
            } else {
                None
            };

            let next = if i + 1 < total_pages {
                Some(PageNeighbor {
                    metadata: self.pages[i + 1].metadata.clone(),
                    rel_html_path: self.pages[i + 1].metadata.rel_html_path.clone(),
                })
            } else {
                None
            };
            self.pages[i].previous = previous;
            self.pages[i].next = next;
        }
        self
    }
}

pub struct PageNeighbor {
    pub metadata: PageMetadata,
    pub rel_html_path: String,
}
