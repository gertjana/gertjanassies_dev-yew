/* Enhanced Prism.js for syntax highlighting */
(function() {
    'use strict';

    var Prism = {
        languages: {},
        highlightAll: function() {
            var elements = document.querySelectorAll('code[class*="language-"], pre[class*="language-"]');
            for (var i = 0; i < elements.length; i++) {
                Prism.highlightElement(elements[i]);
            }
        },
        highlightElement: function(element) {
            var language = getLanguage(element);
            
            var code = element.textContent;
            var highlighted = Prism.highlight(code, language);
            element.innerHTML = highlighted;
        },
        highlight: function(text, language) {
            if (!text) return text;
            return tokenize(text, language);
        }
    };

    function getLanguage(element) {
        var className = element.className;
        var match = className.match(/language-(\w+)/);
        return match ? match[1] : 'text';
    }

    function tokenize(text, language) {
        if (!text) return text;
        
        // Create tokens with positions to avoid overlapping
        var tokens = [];
        var patterns = [
            { type: 'comment', regex: /(\/\*[\s\S]*?\*\/|\/\/.*$|#.*$|<!--[\s\S]*?-->)/gm },
            { type: 'string', regex: /(["'`])((?:\\.|(?!\1)[^\\])*?)\1|r#"[^"]*"#|r"[^"]*"/g },
            { type: 'keyword', regex: getKeywordPattern(language) },
            { type: 'number', regex: /\b(?:0x[a-fA-F0-9]+|0b[01]+|\d+\.?\d*(?:[eE][+-]?\d+)?[fFlL]?)\b/g },
            { type: 'function', regex: /\b\w+(?=\s*\()/g },
            { type: 'operator', regex: /[+\-*/%=!<>&|^~?:]+|->|=>|\.\.|\.\.\./g },
            { type: 'punctuation', regex: /[{}[\];(),.:]/g }
        ];
        
        // Add language-specific patterns
        if (language === 'rust') {
            patterns.push({ type: 'macro', regex: /\b\w+!/g });
            patterns.push({ type: 'attribute', regex: /#\[[\s\S]*?\]/g });
        } else if (language === 'python') {
            patterns.push({ type: 'decorator', regex: /@\w+/g });
        }
        
        // Find all matches
        patterns.forEach(function(pattern) {
            if (!pattern.regex) return;
            
            var match;
            pattern.regex.lastIndex = 0; // Reset regex
            while ((match = pattern.regex.exec(text)) !== null) {
                tokens.push({
                    type: pattern.type,
                    start: match.index,
                    end: match.index + match[0].length,
                    text: match[0]
                });
                
                // Prevent infinite loop
                if (match.index === pattern.regex.lastIndex) {
                    pattern.regex.lastIndex++;
                }
            }
        });
        
        // Sort tokens by position
        tokens.sort(function(a, b) { return a.start - b.start; });
        
        // Remove overlapping tokens (keep first one)
        var filteredTokens = [];
        for (var i = 0; i < tokens.length; i++) {
            var token = tokens[i];
            var overlaps = false;
            
            for (var j = 0; j < filteredTokens.length; j++) {
                var existing = filteredTokens[j];
                if (token.start < existing.end && token.end > existing.start) {
                    overlaps = true;
                    break;
                }
            }
            
            if (!overlaps) {
                filteredTokens.push(token);
            }
        }
        
        // Build highlighted string
        var result = '';
        var lastIndex = 0;
        
        filteredTokens.forEach(function(token) {
            // Add text before token
            result += escapeHtml(text.substring(lastIndex, token.start));
            // Add highlighted token
            result += '<span class="token ' + token.type + '">' + 
                     escapeHtml(token.text) + '</span>';
            lastIndex = token.end;
        });
        
        // Add remaining text
        result += escapeHtml(text.substring(lastIndex));
        
        return result;
    }

    function getKeywordPattern(language) {
        var keywords = {
            rust: 'as|async|await|box|break|const|continue|crate|dyn|else|enum|extern|false|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|self|Self|static|struct|super|trait|true|type|unsafe|use|where|while',
            javascript: 'async|await|break|case|catch|class|const|continue|debugger|default|delete|do|else|enum|export|extends|false|finally|for|function|if|import|in|instanceof|let|new|null|of|return|super|switch|this|throw|true|try|typeof|undefined|var|void|while|with|yield',
            python: 'and|as|assert|break|class|continue|def|del|elif|else|except|exec|finally|for|from|global|if|import|in|is|lambda|not|or|pass|print|raise|return|try|while|with|yield|True|False|None',
            typescript: 'abstract|any|as|async|await|boolean|break|case|catch|class|const|constructor|continue|declare|default|delete|do|else|enum|export|extends|false|finally|for|function|get|if|implements|import|in|instanceof|interface|let|module|namespace|new|null|number|of|package|private|protected|public|readonly|return|set|static|string|super|switch|this|throw|true|try|type|typeof|undefined|var|void|while|with|yield',
            go: 'break|case|chan|const|continue|default|defer|else|fallthrough|for|func|go|goto|if|import|interface|map|package|range|return|select|struct|switch|type|var',
            bash: 'if|then|else|elif|fi|for|do|done|while|until|case|esac|function|select|time|coproc',
            shell: 'if|then|else|elif|fi|for|do|done|while|until|case|esac|function|select|time|coproc'
        };
        
        var keywordList = keywords[language] || keywords.javascript;
        return new RegExp('\\b(' + keywordList + ')\\b', 'g');
    }

    function escapeHtml(text) {
        var div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    // Make Prism global
    window.Prism = Prism;

    // Highlight function that can be called repeatedly
    function highlightCode() {
        setTimeout(function() {
            Prism.highlightAll();
        }, 100);
    }

    // Initial highlighting
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', highlightCode);
    } else {
        highlightCode();
    }

    // Re-highlight when new content is added (for SPA)
    var observer = new MutationObserver(function(mutations) {
        var hasNewCode = false;
        mutations.forEach(function(mutation) {
            if (mutation.type === 'childList') {
                mutation.addedNodes.forEach(function(node) {
                    if (node.nodeType === 1) {
                        var hasCode = node.querySelectorAll && 
                            node.querySelectorAll('code[class*="language-"], pre[class*="language-"]').length > 0;
                        if (hasCode) {
                            hasNewCode = true;
                        }
                    }
                });
            }
        });
        
        if (hasNewCode) {
            highlightCode();
        }
    });
    
    if (document.body) {
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    } else {
        document.addEventListener('DOMContentLoaded', function() {
            observer.observe(document.body, {
                childList: true,
                subtree: true
            });
        });
    }

    // Also expose a global function for manual triggering
    window.highlightCode = highlightCode;
})();