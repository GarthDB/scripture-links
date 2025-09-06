// Scripture Links Web Application
import init, { parse_reference, parse_reference_json, process_text, get_supported_formats } from './pkg/scripture_links_lib.js';

// Global state
let wasmModule = null;
let stats = {
    referencesProcessed: 0,
    textBlocksProcessed: 0
};

// Initialize WASM module
async function initializeWasm() {
    try {
        wasmModule = await init();
        console.log('Scripture Links WASM module loaded successfully!');
        
        // Enable buttons once WASM is loaded
        document.querySelectorAll('button').forEach(btn => {
            btn.disabled = false;
        });
        
        // Load supported formats info
        try {
            const formatsInfo = get_supported_formats();
            console.log('Supported formats:', JSON.parse(formatsInfo));
        } catch (e) {
            console.warn('Could not load format info:', e);
        }
        
    } catch (error) {
        console.error('Failed to initialize WASM module:', error);
        showError('Failed to load the application. Please refresh the page.');
    }
}

// Convert single scripture reference
function convertSingleReference() {
    const input = document.getElementById('single-reference');
    const resultArea = document.getElementById('single-result');
    const reference = input.value.trim();
    
    if (!reference) {
        showError('Please enter a scripture reference');
        return;
    }
    
    if (!wasmModule) {
        showError('Application is still loading. Please try again in a moment.');
        return;
    }
    
    try {
        setLoading(true);
        
        const result = parse_reference_json(reference);
        
        if (result.success) {
            const url = result.url;
            resultArea.textContent = url;
            resultArea.className = 'result-area has-content success';
            
            // Show action buttons
            document.getElementById('single-buttons').classList.remove('hidden');
            
            // Update browser URL with normalized reference
            updateBrowserUrl(reference);
            
            // Update stats
            stats.referencesProcessed++;
            updateStats();
            
            showSuccess('Scripture reference converted successfully!');
            
            // Auto-select the URL for easy copying
            selectText(resultArea);
            
        } else {
            const error = result.error || 'Unknown error occurred';
            
            // If we have structured error info, enhance the display
            if (typeof error === 'object' && error.message) {
                // Clear the result area and build it with HTML for interactive suggestions
                resultArea.innerHTML = '';
                resultArea.className = 'result-area has-content error';
                
                // Add the error message
                const errorText = document.createElement('div');
                errorText.textContent = `Error: ${error.message}`;
                resultArea.appendChild(errorText);
                
                // Add interactive suggestions if available
                if (error.suggestions && error.suggestions.length > 0) {
                    const suggestionDiv = document.createElement('div');
                    suggestionDiv.style.marginTop = '12px';
                    suggestionDiv.style.fontSize = '0.9em';
                    
                    const suggestionLabel = document.createElement('div');
                    suggestionLabel.textContent = '💡 Did you mean:';
                    suggestionLabel.style.marginBottom = '6px';
                    suggestionDiv.appendChild(suggestionLabel);
                    
                    const suggestionLinks = document.createElement('div');
                    suggestionLinks.style.display = 'flex';
                    suggestionLinks.style.gap = '8px';
                    suggestionLinks.style.flexWrap = 'wrap';
                    
                    error.suggestions.forEach((suggestion, index) => {
                        const link = document.createElement('button');
                        link.textContent = suggestion;
                        link.style.background = 'none';
                        link.style.border = '1px solid var(--primary-color)';
                        link.style.color = 'var(--primary-color)';
                        link.style.padding = '4px 8px';
                        link.style.borderRadius = '4px';
                        link.style.cursor = 'pointer';
                        link.style.fontSize = '0.85em';
                        link.style.transition = 'all 0.2s ease';
                        
                        link.addEventListener('mouseenter', () => {
                            link.style.backgroundColor = 'var(--primary-color)';
                            link.style.color = 'white';
                        });
                        
                        link.addEventListener('mouseleave', () => {
                            link.style.backgroundColor = 'transparent';
                            link.style.color = 'var(--primary-color)';
                        });
                        
                        link.addEventListener('click', () => {
                            // Replace the input with the suggestion and the rest of the reference
                            const currentInput = input.value.trim();
                            const parts = currentInput.split(/\s+/);
                            if (parts.length > 0) {
                                // Replace the first part (book name) with the suggestion
                                parts[0] = suggestion;
                                const newReference = parts.join(' ');
                                input.value = newReference;
                                input.focus();
                                
                                // Automatically convert the corrected reference
                                setTimeout(() => {
                                    convertSingleReference();
                                }, 100);
                            }
                        });
                        
                        suggestionLinks.appendChild(link);
                    });
                    
                    suggestionDiv.appendChild(suggestionLinks);
                    resultArea.appendChild(suggestionDiv);
                }
            } else {
                // Fallback for simple error messages
                resultArea.textContent = `Error: ${error}`;
                resultArea.className = 'result-area has-content error';
            }
            
            // Hide action buttons on error
            document.getElementById('single-buttons').classList.add('hidden');
            // Clear URL parameter on error
            clearBrowserUrl();
            showError(typeof error === 'object' && error.message ? error.message : error);
        }
        
    } catch (error) {
        console.error('Error converting reference:', error);
        resultArea.textContent = `Error: ${error.message}`;
        resultArea.className = 'result-area has-content error';
        // Hide action buttons on error
        document.getElementById('single-buttons').classList.add('hidden');
        // Clear URL parameter on error
        clearBrowserUrl();
        showError('An unexpected error occurred');
    } finally {
        setLoading(false);
    }
}

// Process text with scripture references
function processText() {
    const input = document.getElementById('text-input');
    const resultArea = document.getElementById('text-result');
    const text = input.value.trim();
    
    if (!text) {
        showError('Please enter some text to process');
        return;
    }
    
    if (!wasmModule) {
        showError('Application is still loading. Please try again in a moment.');
        return;
    }
    
    try {
        setLoading(true);
        
        const processedText = process_text(text);
        
        if (processedText && processedText !== text) {
            resultArea.textContent = processedText;
            resultArea.className = 'result-area has-content success';
            
            // Show action buttons
            document.getElementById('text-buttons').classList.remove('hidden');
            
            // Update stats
            stats.textBlocksProcessed++;
            
            // Count how many references were found (rough estimate)
            const linkCount = (processedText.match(/\[.*?\]\(https:\/\/www\.churchofjesuschrist\.org/g) || []).length;
            if (linkCount > 0) {
                stats.referencesProcessed += linkCount;
            }
            
            updateStats();
            
            showSuccess(`Text processed successfully! Found ${linkCount} scripture reference${linkCount !== 1 ? 's' : ''}.`);
            
            // Auto-select the result for easy copying
            selectText(resultArea);
            
        } else {
            resultArea.textContent = 'No scripture references found in the text.';
            resultArea.className = 'result-area has-content';
            // Hide action buttons when no references found
            document.getElementById('text-buttons').classList.add('hidden');
            showInfo('No scripture references were found in the provided text.');
        }
        
    } catch (error) {
        console.error('Error processing text:', error);
        resultArea.textContent = `Error: ${error.message}`;
        resultArea.className = 'result-area has-content error';
        // Hide action buttons on error
        document.getElementById('text-buttons').classList.add('hidden');
        showError('An unexpected error occurred while processing text');
    } finally {
        setLoading(false);
    }
}

// Copy and open functions
function copyResult(resultElementId) {
    const resultElement = document.getElementById(resultElementId);
    const text = resultElement.textContent.trim();
    
    if (!text) {
        showError('No content to copy');
        return;
    }
    
    navigator.clipboard.writeText(text).then(() => {
        showSuccess('Copied to clipboard!');
    }).catch(err => {
        console.error('Failed to copy text: ', err);
        showError('Failed to copy to clipboard');
    });
}

function openResult(resultElementId) {
    const resultElement = document.getElementById(resultElementId);
    const text = resultElement.textContent.trim();
    
    if (!text) {
        showError('No URL to open');
        return;
    }
    
    // Check if it's a URL
    if (text.startsWith('https://www.churchofjesuschrist.org')) {
        window.open(text, '_blank', 'noopener,noreferrer');
        showSuccess('Opened in new tab');
    } else {
        showError('Result is not a valid URL');
    }
}

// Normalize reference for URL (remove periods and spaces)
function normalizeReferenceForUrl(reference) {
    return reference
        .toLowerCase()
        .replace(/\./g, '')  // Remove periods
        .replace(/\s+/g, '') // Remove all spaces
        .trim();
}

// Update browser URL with current reference
function updateBrowserUrl(reference) {
    const normalizedRef = normalizeReferenceForUrl(reference);
    const newUrl = new URL(window.location);
    newUrl.searchParams.set('ref', normalizedRef);
    
    // Update URL without reloading the page
    window.history.replaceState({}, '', newUrl);
}

// Clear the ref parameter from browser URL
function clearBrowserUrl() {
    const newUrl = new URL(window.location);
    newUrl.searchParams.delete('ref');
    
    // Update URL without reloading the page
    window.history.replaceState({}, '', newUrl);
}

// Utility functions
function setLoading(isLoading) {
    const buttons = document.querySelectorAll('button');
    const inputs = document.querySelectorAll('input, textarea');
    
    buttons.forEach(btn => {
        btn.disabled = isLoading;
    });
    
    if (isLoading) {
        document.body.classList.add('loading');
    } else {
        document.body.classList.remove('loading');
    }
}

function selectText(element) {
    if (window.getSelection && document.createRange) {
        const selection = window.getSelection();
        const range = document.createRange();
        range.selectNodeContents(element);
        selection.removeAllRanges();
        selection.addRange(range);
    }
}

function updateStats() {
    document.getElementById('references-processed').textContent = stats.referencesProcessed;
    document.getElementById('text-blocks-processed').textContent = stats.textBlocksProcessed;
}

// Toast notification functions
function showError(message) {
    showToast(message, 'error');
}

function showSuccess(message) {
    showToast(message, 'success');
}

function showInfo(message) {
    showToast(message, 'info');
}

function showToast(message, type = 'info') {
    // Hide all existing toasts first
    hideAllToasts();
    
    // On mobile, don't show error toasts if we're already showing error in result area
    // This reduces redundancy and clutter
    if (type === 'error' && window.innerWidth <= 768) {
        const resultAreas = document.querySelectorAll('.result-area.error');
        if (resultAreas.length > 0) {
            // Error is already visible in result area, skip toast
            return;
        }
    }
    
    const toastId = type === 'error' ? 'error-toast' : 'success-toast';
    const messageId = type === 'error' ? 'error-message' : 'success-message';
    
    const toast = document.getElementById(toastId);
    const messageElement = document.getElementById(messageId);
    
    messageElement.textContent = message;
    toast.classList.remove('hidden');
    
    // Auto-hide after 4 seconds on mobile (shorter), 5 seconds on desktop
    const hideDelay = window.innerWidth <= 768 ? 4000 : 5000;
    setTimeout(() => {
        toast.classList.add('hidden');
    }, hideDelay);
}

function hideAllToasts() {
    document.getElementById('error-toast').classList.add('hidden');
    document.getElementById('success-toast').classList.add('hidden');
}

function hideError() {
    document.getElementById('error-toast').classList.add('hidden');
}

function hideSuccess() {
    document.getElementById('success-toast').classList.add('hidden');
}

// Keyboard shortcuts
function handleKeyboardShortcuts(event) {
    // Ctrl/Cmd + Enter to process
    if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
        const activeElement = document.activeElement;
        
        if (activeElement.id === 'single-reference') {
            convertSingleReference();
        } else if (activeElement.id === 'text-input') {
            processText();
        }
    }
    
    // Escape to clear results
    if (event.key === 'Escape') {
        document.querySelectorAll('.result-area').forEach(area => {
            area.textContent = '';
            area.className = 'result-area';
        });
        // Hide all button groups
        document.querySelectorAll('.button-group').forEach(group => {
            group.classList.add('hidden');
        });
        // Clear URL parameter
        clearBrowserUrl();
    }
}

// Check for URL parameters and auto-process
function handleUrlParameters() {
    const urlParams = new URLSearchParams(window.location.search);
    const ref = urlParams.get('ref');
    
    if (ref) {
        // Auto-fill the reference input
        const referenceInput = document.getElementById('single-reference');
        referenceInput.value = ref;
        
        // Auto-convert after WASM loads
        setTimeout(() => {
            if (wasmModule) {
                convertSingleReference();
            } else {
                // Wait for WASM to load, then convert
                const checkWasm = setInterval(() => {
                    if (wasmModule) {
                        clearInterval(checkWasm);
                        convertSingleReference();
                    }
                }, 100);
            }
        }, 100);
    }
}

// Event listeners
document.addEventListener('DOMContentLoaded', () => {
    // Initialize WASM
    initializeWasm();
    
    // Disable buttons until WASM loads
    document.querySelectorAll('button').forEach(btn => {
        btn.disabled = true;
    });
    
    // Handle URL parameters
    handleUrlParameters();
    
    // Add keyboard shortcuts
    document.addEventListener('keydown', handleKeyboardShortcuts);
    
    // Add enter key support for single reference input
    document.getElementById('single-reference').addEventListener('keypress', (event) => {
        if (event.key === 'Enter') {
            convertSingleReference();
        }
    });
    
    
    // Add event listeners for copy/open buttons
    document.getElementById('copy-single-btn').addEventListener('click', () => {
        copyResult('single-result');
    });
    
    document.getElementById('open-single-btn').addEventListener('click', () => {
        openResult('single-result');
    });
    
    document.getElementById('copy-text-btn').addEventListener('click', () => {
        copyResult('text-result');
    });
    
    // Load stats from localStorage if available
    const savedStats = localStorage.getItem('scripture-links-stats');
    if (savedStats) {
        try {
            stats = { ...stats, ...JSON.parse(savedStats) };
            updateStats();
        } catch (e) {
            console.warn('Could not load saved stats:', e);
        }
    }
    
    // Note: Using event listeners instead of onclick handlers for better ES6 module compatibility
});

// Save stats to localStorage when the page unloads
window.addEventListener('beforeunload', () => {
    localStorage.setItem('scripture-links-stats', JSON.stringify(stats));
});

// Make functions available globally for onclick handlers (still needed for main convert/process buttons)
window.convertSingleReference = convertSingleReference;
window.processText = processText;
window.hideError = hideError;
window.hideSuccess = hideSuccess;

// Export for potential use by other modules
export { convertSingleReference, processText, showError, showSuccess };
