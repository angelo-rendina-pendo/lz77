import init, { encode as wasmEncode } from '../pkg/lz77.js';
import { jsEncode } from './js77.js';

function writeOutput(line) {
    const output = document.getElementById('output');
    output.innerHTML += `<p>${line}</p>`;
}

(async function () {
    const wasm77 = await init();
    
    writeOutput('Fetching file...');
    const fileBytes = await fetch('https://www.pendo.io/wp-content/uploads/2019/07/pendo-main-logo-pink-black.svg')
        .then(response => response.arrayBuffer())
        .then(buffer => new Uint8Array(buffer, 0, buffer.byteLength));
    writeOutput(`Done. Got ${fileBytes.length} bytes.`);

    writeOutput('Encoding using WASM...');
    const wasmStart = Date.now();
    const wasmEncodingData = wasmEncode(fileBytes);
    const wasmTime = Date.now() - wasmStart;
    writeOutput(`Done. It took ${wasmTime}ms.`);

    writeOutput('Encoding using JS...');
    const jsStart = Date.now();
    const jsEncoding = jsEncode(fileBytes);
    const jsTime = Date.now() - jsStart;
    writeOutput(`Done. It took ${jsTime}ms.`);

    writeOutput('Comparing encoding...');
    writeOutput(`WASM returned ${wasmEncodingData.length} bytes.`);
    writeOutput(`JS returned ${jsEncoding.length} bytes.`);
    if (wasmEncodingData.length !== jsEncoding.length) {
        writeOutput(`ERROR: Different length.`);
        return;
    }
    writeOutput('Length matches.');

    writeOutput('Comparing bytes...');
    const wasmEncoding = new Uint8Array(wasm77.memory.buffer, wasmEncodingData.address, wasmEncodingData.length);
    for (let i = 0; i < jsEncoding.length; i++) {
        if (jsEncoding[i] !== wasmEncoding[i]) {
            writeOutput(`ERROR: Encoding mismatch.`);
            return;
        }
    }
    writeOutput('Done!');
})();