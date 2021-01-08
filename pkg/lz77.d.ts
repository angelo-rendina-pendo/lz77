/* tslint:disable */
/* eslint-disable */
/**
*/
export function run(): void;
/**
* @param {Uint8Array} bytes
* @returns {Data}
*/
export function encode(bytes: Uint8Array): Data;
/**
* @param {Uint8Array} bytes
* @returns {Data}
*/
export function decode(bytes: Uint8Array): Data;
/**
*/
export class Data {
  free(): void;
/**
* @returns {number}
*/
  address: number;
/**
* @returns {number}
*/
  length: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: () => void;
  readonly __wbg_data_free: (a: number) => void;
  readonly __wbg_get_data_address: (a: number) => number;
  readonly __wbg_set_data_address: (a: number, b: number) => void;
  readonly __wbg_get_data_length: (a: number) => number;
  readonly __wbg_set_data_length: (a: number, b: number) => void;
  readonly encode: (a: number, b: number) => number;
  readonly decode: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        