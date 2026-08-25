# Heading Level 1
## Heading Level 2
### Heading Level 3
#### Heading Level 4
##### Heading Level 5
###### Heading Level 6

Setext Heading 1
================

Setext Heading 2
----------------

---

## Typography & Inline Styles

This paragraph demonstrates standard text formatting: **bold text**, *italicized text*, ***bold and italicized***, ~~strikethrough~~, and ==highlighted text==. You can also represent inline code like `const x = 42;` or keystrokes using <kbd>Ctrl</kbd> + <kbd>C</kbd>.

This is a <sub>subscript</sub> text

This is a <sup>superscript</sup> text

This is an <ins>underlined</ins> text

Subscript: H~2~O  
Superscript: X^2^

---

## Blockquotes

> This is a single-line blockquote.

> This is a multi-line blockquote.
>
> > This is a nested blockquote containing a `code snippet` and **bold text**.

### Alerts
> [!NOTE]
> Useful information that users should know, even when skimming content.

> [!TIP]
> Helpful advice for doing things better or more easily.

> [!IMPORTANT]
> Key information users need to know to achieve their goal.

> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems.

> [!CAUTION]
> Advises about risks or negative outcomes of certain actions.

---

## Lists

### Unordered List
* Item 1
* Item 2
  * Nested Item 2.1
  * Nested Item 2.2
    * Deep Nested Item 2.2.1
* Item 3

### Ordered List
1. First step
2. Second step
   1. Sub-step A
   2. Sub-step B
3. Third step

### Task List (GFM)
- [x] Completed feature
- [ ] In-progress feature
- [ ] Pending task

---

## Code Blocks

### Syntax Highlighting (Rust)
```rust
fn extract_headers(text: &str) -> Vec<(usize, String)> {
    // Parser implementation
    println!("Testing syntax highlighting for Rust");
    vec![(1, "Title".to_string())]
}
```

### Syntax Highlighting (JavaScript)
```javascript
const initSearch = () => {
  console.log("Search initialized...");
};
```

### Indented Code Block
    // Indented by 4 spaces
    function fallback() {
        return true;
    }

---

## Tables

| Left-Aligned | Centered | Right-Aligned | Default |
| :--- | :---: | ---: | --- |
| Header 1 | `code` | $100.00 | Data |
| Header 2 | **bold** | $25.50 | Data |
| Header 3 | *italic* | $3.00 | Data |

---

## Links & Media
* **Autolink**: <https://github.com/kyncl/vault>
* **Reference Link**: [Markdown Guide][1]
* **Anchor Link**: [Jump to Code Blocks](#code-blocks)

[1]: https://www.markdownguide.org

---

## Extended Elements

### Footnotes
Here is a sentence with a footnote reference.[^1]

[^1]: This is the corresponding footnote text at the bottom.

### Collapsible Section (HTML)
<details>
<summary>Click to expand additional details</summary>

This hidden content tests raw HTML rendering support within Markdown.

</details>

### Supported color models
The background color is `#ffffff` for light mode and `#000000` for dark mode.
`rgb(9, 105, 218)`
`hsl(212, 92%, 45%)`
