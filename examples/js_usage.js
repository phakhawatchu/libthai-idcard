#!/usr/bin/env node

/**
 * js_usage.js — Example of using libthaiidcard from Node.js via koffi.
 *
 * Usage:
 *   # Build the shared library first:
 *   make shared
 *
 *   # Install koffi:
 *   npm install koffi
 *
 *   # Run:
 *   node examples/js_usage.js [reader_name]
 *
 * koffi: https://github.com/Koromix/koffi
 */

const koffi = require('koffi');
const fs = require('fs');
const path = require('path');
const os = require('os');

// ---------------------------------------------------------------------------
// Library discovery
// ---------------------------------------------------------------------------

/**
 * Return the shared library filename for the current platform.
 */
function libraryFilename() {
    switch (os.platform()) {
        case 'win32':  return 'thaiidcard.dll';
        case 'darwin': return 'libthaiidcard.dylib';
        default:       return 'libthaiidcard.so';
    }
}

/**
 * Locate libthaiidcard on the filesystem.
 * Returns a full path or the bare name (for system search).
 */
function findLibrary() {
    const name = libraryFilename();

    // Derive project root: this script is at examples/js_usage.js
    const projectRoot = path.resolve(__dirname, '..');

    const candidates = [
        // Development build
        path.join(projectRoot, 'target', 'debug', name),
        path.join(projectRoot, 'target', 'release', name),
    ];

    // Platform-specific system paths
    switch (os.platform()) {
        case 'darwin':
            candidates.push(
                '/usr/local/lib/' + name,
                '/opt/homebrew/lib/' + name
            );
            break;
        case 'win32':
            const sysRoot = process.env.SYSTEMROOT || 'C:\\Windows';
            const pf = process.env.PROGRAMFILES || 'C:\\Program Files';
            candidates.push(
                path.join(sysRoot, 'System32', name),
                path.join(pf, 'thaiidcard', 'bin', name)
            );
            break;
        default: // Linux
            candidates.push(
                '/usr/local/lib/' + name,
                '/usr/lib/' + name,
                '/usr/lib/x86_64-linux-gnu/' + name,
                '/usr/lib/aarch64-linux-gnu/' + name
            );
            break;
    }

    for (const p of candidates) {
        if (fs.existsSync(p)) return p;
    }

    // Fallback: let the system linker search (LD_LIBRARY_PATH etc.)
    return name;
}

// ---------------------------------------------------------------------------
// Load library and define FFI bindings with koffi
// ---------------------------------------------------------------------------

const libPath = findLibrary();
const lib = koffi.load(libPath);

// --- thaiid_read(reader: str|null, face: int, nhso: int, laser: int) -> handle ---
const thaiid_read = lib.func('thaiid_read', 'void *', ['string', 'int', 'int', 'int']);

// --- thaiid_free(handle) ---
const thaiid_free = lib.func('thaiid_free', 'void', ['void *']);

// --- thaiid_get_last_error() -> str ---
const thaiid_get_last_error = lib.func('thaiid_get_last_error', 'string', []);

// --- Getters: all take handle, return string ---
const GETTERS = {
    cid:                   lib.func('thaiid_get_cid', 'string', ['void *']),
    name_thai:             lib.func('thaiid_get_name_thai', 'string', ['void *']),
    name_en:               lib.func('thaiid_get_name_en', 'string', ['void *']),
    dob:                   lib.func('thaiid_get_dob', 'string', ['void *']),
    gender:                lib.func('thaiid_get_gender', 'string', ['void *']),
    card_issuer:           lib.func('thaiid_get_card_issuer', 'string', ['void *']),
    issue_date:            lib.func('thaiid_get_issue_date', 'string', ['void *']),
    expire_date:           lib.func('thaiid_get_expire_date', 'string', ['void *']),
    address:               lib.func('thaiid_get_address', 'string', ['void *']),
    face_image:            lib.func('thaiid_get_face_image', 'string', ['void *']),
    laser_id:              lib.func('thaiid_get_laser_id', 'string', ['void *']),
    main_inscl:            lib.func('thaiid_get_main_inscl', 'string', ['void *']),
    sub_inscl:             lib.func('thaiid_get_sub_inscl', 'string', ['void *']),
    main_hospital:         lib.func('thaiid_get_main_hospital', 'string', ['void *']),
    sub_hospital:          lib.func('thaiid_get_sub_hospital', 'string', ['void *']),
    paid_type:             lib.func('thaiid_get_paid_type', 'string', ['void *']),
    nhso_issue_date:       lib.func('thaiid_get_nhso_issue_date', 'string', ['void *']),
    nhso_expire_date:      lib.func('thaiid_get_nhso_expire_date', 'string', ['void *']),
    nhso_update_date:      lib.func('thaiid_get_nhso_update_date', 'string', ['void *']),
    change_hospital_amount: lib.func('thaiid_get_change_hospital_amount', 'string', ['void *']),
};

// ---------------------------------------------------------------------------
// Read card
// ---------------------------------------------------------------------------

function readCard(readerName, showFace = true, showNhso = true, showLaser = false) {
    const handle = thaiid_read(readerName || null, +showFace, +showNhso, +showLaser);
    if (!handle) {
        const err = thaiid_get_last_error();
        throw new Error(err || 'Unknown error');
    }
    try {
        const data = {};
        for (const [key, fn] of Object.entries(GETTERS)) {
            data[key] = fn(handle) || '';
        }
        return data;
    } finally {
        thaiid_free(handle);
    }
}

// ---------------------------------------------------------------------------
// Pretty-print
// ---------------------------------------------------------------------------

function printCard(data) {
    const field = (label, val) => console.log(`  ${label.padEnd(20)} ${val}`);

    console.log('\n=== Personal Information ===');
    field('CID:',         data.cid);
    field('Name (TH):',   data.name_thai);
    field('Name (EN):',   data.name_en);
    field('DOB:',         data.dob);
    field('Gender:',      data.gender);
    field('Card Issuer:', data.card_issuer);
    field('Issue Date:',  data.issue_date);
    field('Expire Date:', data.expire_date);
    field('Address:',     data.address);

    if (data.face_image) {
        console.log(`  ${'Face Image:'.padEnd(20)} [${Buffer.byteLength(data.face_image, 'utf8')} bytes base64]`);
    }

    if (data.laser_id) {
        console.log('\n=== Card Info ===');
        field('Laser ID:', data.laser_id);
    }

    if (data.main_inscl) {
        console.log('\n=== NHSO Information ===');
        field('Main Inscl:',        data.main_inscl);
        field('Sub Inscl:',         data.sub_inscl);
        field('Main Hosp:',         data.main_hospital);
        field('Sub Hosp:',          data.sub_hospital);
        field('Paid Type:',         data.paid_type);
        field('NHSO Issue:',        data.nhso_issue_date);
        field('NHSO Expire:',       data.nhso_expire_date);
        field('NHSO Update:',       data.nhso_update_date);
        field('Change Hosp:',       data.change_hospital_amount);
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

const reader = process.argv[2] || null;

console.log(`Using library: ${libPath}`);
console.log(`Reader:        ${reader || '(auto-detect)'}`);
console.log('Waiting for card...');

try {
    const data = readCard(reader);
    printCard(data);
    console.log('\nDone.');
} catch (err) {
    console.error('Error:', err.message);
    process.exit(1);
}
