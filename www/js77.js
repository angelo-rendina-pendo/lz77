function rfind(window, search) {
    if (search.length > window.length) {
        return -1;
    }
    for (let position = window.length - search.length; position >= 0; position--) {
        const segment = window.slice(position, position + search.length);
        let segmentMatch = true;
        for (let i = 0; i < segment.length; i++) {
            if (segment[i] !== search[i]) {
                segmentMatch = false;
                break;
            }
        }
        if (segmentMatch) return position;
    }
    return -1;
}

function findCode(window, lookahead) {
    const code = {
        offset: 0,
        length: 0,
        literal: lookahead[0]
    };
    let searchLength = 1;
    while (searchLength < lookahead.length) {
        const search = lookahead.slice(0, searchLength);
        const rightmostMatch = rfind(window, search);
        if (rightmostMatch < 0) {
            break;
        }
        code.offset = window.length - rightmostMatch;
        code.length = searchLength;
        code.literal = lookahead[searchLength];
        searchLength++;
    }
    return code;
}

function encode(input) {
    const encoded = [];
    let position = 0;
    while (position < input.length) {
        const lookahead = input.slice(position);
        const windowStart = Math.max(position - 255, 0);
        const window = input.slice(windowStart, position);
        const code = findCode(window, lookahead);
        position += code.length + 1;
        encoded.push(code);
    }
    return encoded;
}

function encodingToBytes(encoded) {
    return encoded.reduce((bytes, code) => {
        return bytes.concat([
            code.offset,
            code.length,
            code.literal
        ])
    }, []);
}

export function jsEncode(input) {
    const encoded = encode(input);
    return new Uint8Array(encodingToBytes(encoded));
}