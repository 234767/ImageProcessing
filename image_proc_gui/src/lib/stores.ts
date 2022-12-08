import {writable} from "svelte/store";
import type {Writable} from "svelte/store";

export const activeImagePath : Writable<string> = writable(null)
export const activeHistogramPath: Writable<string> = writable(null);