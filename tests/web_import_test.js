#!/usr/bin/env node

/**
 * Web Import Test
 * 
 * This test verifies that the web app's JavaScript import path
 * matches the actual WASM files generated. This catches issues
 * like library name changes that break the import.
 */

import { readFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { existsSync } from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function testWebImports() {
    console.log('🔍 Testing web app import consistency...');
    
    try {
        // Read the web app JavaScript file
        const appJsPath = join(__dirname, '../web/app.js');
        const appJsContent = await readFile(appJsPath, 'utf-8');
        
        // Extract the import path using regex
        const importMatch = appJsContent.match(/import.*from\s+['"](.\/pkg\/[^'"]+)['"]/);
        
        if (!importMatch) {
            throw new Error('Could not find WASM import statement in web/app.js');
        }
        
        const importPath = importMatch[1];
        console.log(`📦 Found import path: ${importPath}`);
        
        // Convert relative path to absolute
        const absoluteImportPath = join(__dirname, '../web', importPath);
        
        // Check if the imported file exists
        if (!existsSync(absoluteImportPath)) {
            throw new Error(`Import file does not exist: ${absoluteImportPath}`);
        }
        
        console.log('✅ Import file exists');
        
        // Check for corresponding WASM file
        const wasmPath = absoluteImportPath.replace('.js', '_bg.wasm');
        if (!existsSync(wasmPath)) {
            throw new Error(`Corresponding WASM file does not exist: ${wasmPath}`);
        }
        
        console.log('✅ Corresponding WASM file exists');
        
        // Verify the imported JS file has the expected exports
        const jsContent = await readFile(absoluteImportPath, 'utf-8');
        const expectedExports = ['parse_reference_json', 'process_text', 'get_supported_formats'];
        
        for (const exportName of expectedExports) {
            if (!jsContent.includes(exportName)) {
                throw new Error(`Expected export '${exportName}' not found in ${importPath}`);
            }
        }
        
        console.log('✅ All expected exports found');
        console.log('🎉 Web import consistency test passed!');
        return true;
        
    } catch (error) {
        console.error('❌ Web import test failed:', error.message);
        return false;
    }
}

// Run the test
testWebImports().then(success => {
    process.exit(success ? 0 : 1);
});
