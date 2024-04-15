/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Cell {
  Dead = 0,
  Alive = 1,
}
/**
*/
export class LifeOfGame {
  free(): void;
/**
* @param {number} size
* @returns {LifeOfGame}
*/
  static new(size: number): LifeOfGame;
/**
* @param {number} i
* @param {number} j
*/
  toggle(i: number, j: number): void;
/**
*/
  draw(): void;
/**
*/
  step(): void;
/**
* @returns {string}
*/
  render(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_lifeofgame_free: (a: number) => void;
  readonly lifeofgame_new: (a: number) => number;
  readonly lifeofgame_toggle: (a: number, b: number, c: number) => void;
  readonly lifeofgame_draw: (a: number) => void;
  readonly lifeofgame_step: (a: number) => void;
  readonly lifeofgame_render: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
