# Vault
Vault is a lightweight, blazing-fast documentation generator written in Rust. 
It automatically transforms a directory of Markdown files into a fully responsive, 
styled documentation site complete with automatic sidebar navigation, category grouping, 
syntax highlighting, and dynamic previous/next page buttons.

---

## Features

- **Zero-Config Structure:** Simply drop your `.md` files into a folder structure, 
and Vault handles the hierarchy, categories, and links automatically.
- **Smart Navigation:** Automatically generates Previous and Next page buttons at 
the bottom of every page with full title mapping.
- **Auto-Generated Sidebar:** Intelligently organizes files into sidebar sections 
based on your folder structure, prioritizing key files like `index` or `overview` 
(index files are turned into clickable category header).
- **Syntax Highlighting:** Built-in code block styling and syntax 
highlighting for technical documentation.
- **Fully Customizable Styling Engine:** Change the look and feel of your docs using custom CLI:
  - **20 Color Themes:** Choose from Crimson, Orange, Emerald, Teal, Cyan, Blue, Violet, Fuchsia, Pink, and more.
  Still not enough? You can custom pick your own HEX code, which
  will be used as main theme color.
  - **Responsive Background Profiles:** Automatically adapt to system light/dark mode with options
  like **Standard**, **Comfy**, **Deep Black**, **Zen**, and more.
  - **Border-Radius Styles:** Switch between **Standard** (8px–2px gradient),
  **Brutalist** (sharp 0px edges), and **Rounded** (extra smooth 16px–4px gradient).
- **Responsive & Theme-Aware:** Mobile-friendly layout featuring a toggleable 
sidebar/burger menu and clean dark/light mode CSS variables.
- **Searching:** Search feature for users. Can be invoked by `ctrl+k` or `/`. Looks 
through current page and name of the files and their category.
- **Asset Optimization (Injected vs. Lazy):** Smart asset pipeline that handles CSS and JS files in two modes:
  - **Injected:** Bundled and embedded directly inside the HTML for maximum performance and zero extra requests.
  - **Lazy:** Linked efficiently inside the headers to load asynchronously. (Fonts are always loaded lazily).
  To make lazy file you must put `lazy__` prefix into file's name.

---

## Project Structure
While you can specify custom paths for your files, it is highly recommended 
to keep everything inside a dedicated `docs` directory.

Vault expects a clean directory structure separating your 
source Markdown files from the generated HTML output:
```text
your-project/
├── docs/
│   ├── md/                 # Your source Markdown files
│   │   └── index.md
│   └── html/               # Generated output site
│       ├── css/            # Vault's CSS files (custom styles supported)
│       ├── js/             # Vault's JS files (custom scripts supported)
│       ├── fonts/          # Custom fonts directory (@font-face required in css/)
│       └── index.html
└── ...
```

## Getting Started
### Prerequisites
- Git
- Cargo (Rust package manager)
- Make (Optional)
### Quick Run
```bash 
git clone https://github.com/kyncl/vault.git
cd vault

# For development (fast compilation)
cargo run -- --title "Your Custom Title"
# For production builds (optimized performance)
cargo run --release -- --title "Your Custom Title"
```
Vault will read all Markdown files, parse metadata, map sequential links, generate structured sidebars, 
inject custom CSS/JS assets, and output minified HTML files 
ready for deployment inside `docs/html`.

### Global Installation
If you want to install Vault globally on your machine:
```bash
make install
```
Once installed, you can run Vault from anywhere:
```bash
vault --title <YOUR-CUSTOM-TITLE>
```

## Configuration & CLI Flags
You can customize input and output directories by passing command-line flags:
```text
Path Options:
  -m, --md-path <MD_PATH>
         Folder, where your markdowns live

         [default: ./docs/md]

  -p, --html-path <HTML_PATH>
         Folder, where will your HTML files live
         and its resources

         [default: ./docs/html]
```

## License & Commercial Use
Distributed under the MIT License. See LICENSE for more information.
