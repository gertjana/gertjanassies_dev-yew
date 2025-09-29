// Initialize KaTeX auto-rendering when the page loads
document.addEventListener("DOMContentLoaded", function() {
    renderMathInElement(document.body, {
        // Delimiters for inline and display math
        delimiters: [
            {left: '$$', right: '$$', display: true},
            {left: '$', right: '$', display: false},
            {left: '\\(', right: '\\)', display: false},
            {left: '\\[', right: '\\]', display: true}
        ],
        // KaTeX options
        throwOnError: false,
        errorColor: '#cc0000'
    });
});

// Function to re-render math when content is dynamically loaded
window.renderMath = function() {
    if (window.renderMathInElement) {
        renderMathInElement(document.body, {
            delimiters: [
                {left: '$$', right: '$$', display: true},
                {left: '$', right: '$', display: false},
                {left: '\\(', right: '\\)', display: false},
                {left: '\\[', right: '\\]', display: true}
            ],
            throwOnError: false,
            errorColor: '#cc0000'
        });
    }
};
