// Copy code block functionality
function initCopyButtons() {
    document.querySelectorAll('.copy-code-button:not(.initialized)').forEach(button => {
        button.classList.add('initialized');
        button.addEventListener('click', async function(e) {
            e.preventDefault();
            e.stopPropagation();

            // The button is inside .code-block-wrapper, and the pre is the next sibling of the button
            const pre = this.nextElementSibling;

            if (!pre) {
                console.error('Pre element not found');
                return;
            }

            // Try to find code element, or use pre's text content directly
            const codeBlock = pre.querySelector('code');
            const code = codeBlock ? codeBlock.textContent : pre.textContent;

            if (!code) {
                console.error('No code content found');
                return;
            }

            try {
                await navigator.clipboard.writeText(code);
                showSuccess(this);
            } catch (err) {
                console.error('Failed to copy code:', err);

                // Fallback for older browsers
                const textArea = document.createElement('textarea');
                textArea.value = code;
                textArea.style.position = 'fixed';
                textArea.style.left = '-999999px';
                textArea.style.top = '0';
                document.body.appendChild(textArea);
                textArea.focus();
                textArea.select();

                try {
                    document.execCommand('copy');
                    showSuccess(this);
                } catch (err) {
                    console.error('Fallback copy failed:', err);
                }

                document.body.removeChild(textArea);
            }
        });
    });
}

function showSuccess(button) {
    const originalHTML = button.innerHTML;
    button.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>';
    button.classList.add('copied');

    setTimeout(() => {
        button.innerHTML = originalHTML;
        button.classList.remove('copied');
    }, 2000);
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', initCopyButtons);

// Re-initialize when content changes (for Yew/WASM dynamic content)
const observer = new MutationObserver(function(mutations) {
    mutations.forEach(function(mutation) {
        if (mutation.addedNodes.length) {
            initCopyButtons();
        }
    });
});

// Start observing the document body for changes
if (document.body) {
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });
} else {
    // If body doesn't exist yet, wait for it
    document.addEventListener('DOMContentLoaded', function() {
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    });
}
