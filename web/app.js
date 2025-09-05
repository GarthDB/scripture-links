// Scripture Links Web Application
import init, { parse_reference, process_text, get_supported_formats } from './pkg/scripture_links.js';

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
        
        const result = parse_reference(reference);
        
        if (result.success) {
            const url = result.result;
            resultArea.textContent = url;
            resultArea.className = 'result-area has-content success';
            
            // Show action buttons
            document.getElementById('single-buttons').classList.remove('hidden');
            
            // Update stats
            stats.referencesProcessed++;
            updateStats();
            
            showSuccess('Scripture reference converted successfully!');
            
            // Auto-select the URL for easy copying
            selectText(resultArea);
            
        } else {
            const error = result.error || 'Unknown error occurred';
            resultArea.textContent = `Error: ${error}`;
            resultArea.className = 'result-area has-content error';
            // Hide action buttons on error
            document.getElementById('single-buttons').classList.add('hidden');
            showError(error);
        }
        
    } catch (error) {
        console.error('Error converting reference:', error);
        resultArea.textContent = `Error: ${error.message}`;
        resultArea.className = 'result-area has-content error';
        // Hide action buttons on error
        document.getElementById('single-buttons').classList.add('hidden');
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
    const toastId = type === 'error' ? 'error-toast' : 'success-toast';
    const messageId = type === 'error' ? 'error-message' : 'success-message';
    
    const toast = document.getElementById(toastId);
    const messageElement = document.getElementById(messageId);
    
    messageElement.textContent = message;
    toast.classList.remove('hidden');
    
    // Auto-hide after 5 seconds
    setTimeout(() => {
        toast.classList.add('hidden');
    }, 5000);
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
