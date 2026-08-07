/* tslint:disable */
/* eslint-disable */

export function add_friend(nickname: string | null | undefined, invite_key: string, storage_json: string): string;

export function copy_to_clipboard(storage_json: string, item: string): Promise<string>;

export function create_identity(username?: string | null): string;

export function decrypt(my_public_b64: string, my_private_b64: string, friend_index_json: string, message_key: string): string;

export function encrypt(my_public_b64: string, my_private_b64: string, their_public_b64: string, message: string): string;

export function load_display_data(storage_json: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly add_friend: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly copy_to_clipboard: (a: number, b: number, c: number, d: number) => any;
    readonly create_identity: (a: number, b: number) => [number, number];
    readonly decrypt: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
    readonly encrypt: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
    readonly load_display_data: (a: number, b: number) => [number, number];
    readonly wasm_bindgen_7815ba1f1746f71b___convert__closures_____invoke___wasm_bindgen_7815ba1f1746f71b___JsValue__core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_7815ba1f1746f71b___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_7815ba1f1746f71b___convert__closures_____invoke___js_sys_2c04299f1b2b182f___Function_fn_wasm_bindgen_7815ba1f1746f71b___JsValue_____wasm_bindgen_7815ba1f1746f71b___sys__Undefined___js_sys_2c04299f1b2b182f___Function_fn_wasm_bindgen_7815ba1f1746f71b___JsValue_____wasm_bindgen_7815ba1f1746f71b___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
