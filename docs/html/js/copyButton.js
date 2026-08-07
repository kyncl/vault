// 1. Code Block Copy Buttons
function initCodeBlocks() {
    document.querySelectorAll('pre').forEach((block) => {
        if (getComputedStyle(block).position === 'static') {
            block.style.position = 'relative';
        }

        const copyButton = document.createElement('button');
        copyButton.type = 'button';
        copyButton.innerText = 'Copy';
        Object.assign(copyButton.style, {
            position: 'absolute',
            top: '8px',
            right: '8px',
            padding: '4px 8px',
            fontSize: '12px',
            fontFamily: 'sans-serif',
            background: '#2d2d2d',
            color: '#ffffff',
            border: '1px solid #444',
            borderRadius: 'var(--radius-sm)',
            cursor: 'pointer',
            zIndex: '10',
            transition: 'background 0.2s ease'
        });

        copyButton.addEventListener('mouseenter', () => copyButton.style.background = '#444');
        copyButton.addEventListener('mouseleave', () => copyButton.style.background = '#2d2d2d');

        copyButton.addEventListener('click', async () => {
            const codeElement = block.querySelector('code');
            const textToCopy = codeElement ? codeElement.innerText : block.innerText;
            try {
                await navigator.clipboard.writeText(textToCopy);
                copyButton.innerText = 'Copied!';
            } catch (err) {
                console.error('Failed to copy text: ', err);
                copyButton.innerText = 'Error';
            } finally {
                setTimeout(() => copyButton.innerText = 'Copy', 2000);
            }
        });

        block.appendChild(copyButton);
    });
}
