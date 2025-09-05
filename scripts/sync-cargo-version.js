#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Read package.json version
const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
const newVersion = packageJson.version;

// Read Cargo.toml
const cargoTomlPath = 'Cargo.toml';
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');

// Update version in Cargo.toml
cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${newVersion}"`);

// Write back to Cargo.toml
fs.writeFileSync(cargoTomlPath, cargoToml);

console.log(`✅ Updated Cargo.toml version to ${newVersion}`);
