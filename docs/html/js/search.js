// 4. Search Functionality & Indexing
function initSearch() {
    const searchBtn = document.querySelector('.search-btn');
    const searchBox = document.querySelector('.search-box');
    const searchInput = document.getElementById('search');
    const suggestionsContainer = document.querySelector('.search-suggestions');
    const currentPath = window.location.pathname;

    if (!searchBtn || !searchBox || !searchInput || !suggestionsContainer) return;

    let currentMatches = [];
    let selectedIndex = 0;
    const searchIndex = [];

    // --- NEW: Calculate root path dynamically and add Home ---
    // Look at any sidebar link to figure out how many "../" we need to get back to root
    let rootPath = 'index.html';
    const sampleLink = document.querySelector('.sidebar-link, .sidebar-title-link');
    if (sampleLink) {
        const href = sampleLink.getAttribute('href') || '';
        // Extract the leading "../" sequences if they exist
        const match = href.match(/^(?:\.\.\/)+/);
        rootPath = match ? `${match[0]}index.html` : 'index.html';
    }

    // Inject Home page into search index
    const rootResolver = document.createElement('a');
    rootResolver.href = rootPath;

    if (rootResolver.pathname !== currentPath) {
        searchIndex.push({
            title: 'Home',
            searchableText: 'home homepage root main',
            href: rootPath,
            type: 'Page'
        });
    }
    // ---------------------------------------------------------

    // Parse structured sidebar sections
    document.querySelectorAll('.sidebar-section').forEach(section => {
        const titleLinkEl = section.querySelector('.sidebar-title-link');
        const titleEl = section.querySelector('.sidebar-title');

        let categoryName = 'Documentation';
        if (titleLinkEl) {
            categoryName = titleLinkEl.textContent.trim();
            // NEW: Check if the category index is the current page
            if (titleLinkEl.pathname !== currentPath) {
                searchIndex.push({
                    title: `${categoryName} (Index)`,
                    searchableText: `${categoryName.toLowerCase()} index directory`,
                    href: titleLinkEl.getAttribute('href'),
                    type: 'Category Index'
                });
            }
        } else if (titleEl) {
            categoryName = titleEl.textContent.trim();
        }

        section.querySelectorAll('.sidebar-link').forEach(link => {
            // NEW: Check if the link is the current page
            if (link.pathname !== currentPath) {
                const title = link.textContent.trim();
                searchIndex.push({
                    title,
                    searchableText: `${title} ${categoryName}`.toLowerCase(),
                    href: link.getAttribute('href'),
                    type: categoryName
                });
            }
        });
    });

    // Parse root-level sidebar lists
    document.querySelectorAll('.sidebar > .sidebar-list .sidebar-link').forEach(link => {
        if (link.pathname !== currentPath) {
            const title = link.textContent.trim();
            searchIndex.push({
                title,
                searchableText: title.toLowerCase(),
                href: link.getAttribute('href'),
                type: 'Page'
            });
        }
    });

    // Parse content headers
    document.querySelectorAll('.doc-content h1, .doc-content h2, .doc-content h3').forEach(header => {
        let id = header.id;
        if (!id) {
            id = header.textContent.toLowerCase().replace(/[^\w]+/g, '-');
            header.id = id;
        }

        let sectionText = header.textContent;
        let nextElement = header.nextElementSibling;
        while (nextElement && !['H1', 'H2', 'H3'].includes(nextElement.tagName)) {
            sectionText += ` ${nextElement.textContent}`;
            nextElement = nextElement.nextElementSibling;
        }

        const currentPage = window.location.pathname.split('/').pop() || 'index.html';
        const title = header.textContent.trim();

        searchIndex.push({
            title,
            searchableText: sectionText.toLowerCase(),
            href: `${currentPage}#${id}`,
            type: 'Section'
        });
    });

    const closeSearch = () => {
        searchBox.classList.remove('active');
        searchInput.value = '';
        suggestionsContainer.innerHTML = '';
        currentMatches = [];
        selectedIndex = 0;
    };

    const updateHighlight = () => {
        suggestionsContainer.querySelectorAll('.suggestion-item').forEach((btn, index) => {
            btn.classList.toggle('selected', index === selectedIndex);
        });
    };

    const updateSuggestions = (query) => {
        suggestionsContainer.innerHTML = '';
        const trimmedQuery = query.trim().toLowerCase();

        if (!trimmedQuery) {
            currentMatches = searchIndex.slice(0, 8);
        } else {
            currentMatches = searchIndex.map(item => {
                const titleLower = item.title.toLowerCase();
                let score = 0;

                if (titleLower === trimmedQuery) score = 100;
                else if (titleLower.startsWith(trimmedQuery)) score = 75;
                else if (titleLower.includes(trimmedQuery)) score = 50;
                else if (item.searchableText.includes(trimmedQuery)) score = 25;

                return { ...item, score };
            })
                .filter(item => item.score > 0)
                .sort((a, b) => b.score - a.score)
                .slice(0, 5);
        }

        if (currentMatches.length === 0) {
            suggestionsContainer.innerHTML = `<div style="padding: 0.6rem 0.75rem; color: var(--muted); font-size: 0.9rem; font-family: var(--font-code);">No results found</div>`;
            selectedIndex = -1;
            return;
        }

        selectedIndex = 0;

        currentMatches.forEach((match, index) => {
            const btn = document.createElement('button');
            btn.type = 'button';
            btn.className = `suggestion-item ${index === 0 ? 'selected' : ''}`;
            btn.innerHTML = `<span>${match.title}</span> <small style="opacity: 0.4; font-size: 0.75rem; font-family: var(--font-code);">${match.type}</small>`;

            btn.addEventListener('click', () => {
                window.location.href = match.href;
                closeSearch();
            });

            suggestionsContainer.appendChild(btn);
        });
    };

    searchInput.addEventListener('input', (e) => updateSuggestions(e.target.value));

    searchInput.addEventListener('keydown', (e) => {
        if (currentMatches.length === 0) return;

        if (e.key === 'ArrowDown') {
            e.preventDefault();
            selectedIndex = (selectedIndex + 1) % currentMatches.length;
            updateHighlight();
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            selectedIndex = (selectedIndex - 1 + currentMatches.length) % currentMatches.length;
            updateHighlight();
        } else if (e.key === 'Enter') {
            e.preventDefault();
            if (selectedIndex >= 0 && currentMatches[selectedIndex]) {
                window.location.href = currentMatches[selectedIndex].href;
                closeSearch();
            }
        }
    });

    document.addEventListener('keydown', (e) => {
        if (document.body.classList.contains('sidebar-locked')) return;

        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
            e.preventDefault();
            searchBox.classList.add('active');
            searchInput.focus();
            updateSuggestions(searchInput.value);
        } else if (e.key === '/' && document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA') {
            e.preventDefault();
            searchBox.classList.add('active');
            searchInput.focus();
            updateSuggestions(searchInput.value);
        } else if (e.key === 'Escape') {
            closeSearch();
        }
    });

    searchBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        if (document.body.classList.contains('sidebar-locked')) return;

        if (searchBox.classList.contains('active')) {
            closeSearch();
        } else {
            searchBox.classList.add('active');
            searchInput.focus();
            updateSuggestions(searchInput.value);
        }
    });

    searchBox.addEventListener('click', (e) => e.stopPropagation());
    document.addEventListener('click', closeSearch);
}
