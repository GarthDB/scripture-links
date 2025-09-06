#!/usr/bin/env node

/**
 * WASM Integration Test
 * 
 * This test verifies that the WASM module can be loaded and that
 * the exported functions work correctly. This helps catch issues
 * like library name changes that break the web interface.
 */

import { readFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function testWasmModule() {
    console.log('🧪 Testing WASM module integration...');
    
    try {
        // Import the WASM module
        const wasmPath = join(__dirname, '../web/pkg/scripture_links_lib.js');
        const { default: init, parse_reference_json, process_text, get_supported_formats } = await import(wasmPath);
        
        console.log('✅ WASM module imported successfully');
        
        // Initialize the WASM module with the binary
        const wasmBinaryPath = join(__dirname, '../web/pkg/scripture_links_lib_bg.wasm');
        const wasmBinary = await readFile(wasmBinaryPath);
        await init(wasmBinary);
        console.log('✅ WASM module initialized');
        
        // Test parse_reference_json function
        const testRef = "Genesis 1:1";
        const result = parse_reference_json(testRef);
        
        // Handle both string and object returns
        let parsed;
        if (typeof result === 'string') {
            parsed = JSON.parse(result);
        } else {
            parsed = result;
        }
        
        if (parsed.success && parsed.url && parsed.url.includes('churchofjesuschrist.org')) {
            console.log('✅ parse_reference_json works correctly');
        } else {
            throw new Error(`parse_reference_json failed: ${JSON.stringify(parsed)}`);
        }
        
        // Test process_text function
        const testText = "See Genesis 1:1 for creation.";
        const processedText = process_text(testText);
        
        if (processedText.includes('[Genesis 1:1](') && processedText.includes('churchofjesuschrist.org')) {
            console.log('✅ process_text works correctly');
        } else {
            throw new Error(`process_text failed: ${processedText}`);
        }
        
        // Test get_supported_formats function
        const formats = get_supported_formats();
        
        // Handle both string and object returns
        let formatsList;
        if (typeof formats === 'string') {
            formatsList = JSON.parse(formats);
        } else {
            formatsList = formats;
        }
        
        if (formatsList.supported_works && Array.isArray(formatsList.supported_works) && formatsList.supported_works.length > 0) {
            console.log('✅ get_supported_formats works correctly');
        } else {
            throw new Error(`get_supported_formats failed: ${JSON.stringify(formatsList)}`);
        }
        
        console.log('🎉 All WASM integration tests passed!');
        return true;
        
    } catch (error) {
        console.error('❌ WASM integration test failed:', error.message);
        console.error('Stack trace:', error.stack);
        return false;
    }
}

// Run the test
testWasmModule().then(success => {
    process.exit(success ? 0 : 1);
});
