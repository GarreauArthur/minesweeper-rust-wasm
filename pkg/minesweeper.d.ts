/* tslint:disable */
/* eslint-disable */
/**
* @param {number} width
* @param {number} height
* @param {number} mines
* @returns {string}
*/
export function startGame(width: number, height: number, mines: number): string;
/**
* @param {number} x
* @param {number} y
* @returns {string}
*/
export function openField(x: number, y: number): string;
/**
* @param {number} x
* @param {number} y
* @returns {string}
*/
export function toggleFlag(x: number, y: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly startGame: (a: number, b: number, c: number, d: number) => void;
  readonly openField: (a: number, b: number, c: number) => void;
  readonly toggleFlag: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
