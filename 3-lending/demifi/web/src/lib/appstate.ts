import { writable } from 'svelte/store';

export const appStarted: Writable<bool> = writable(false);
export const commitButton: Writable = writable(undefined);
export const mainParticipantsFilter: Writable = writable(undefined);
export const mainParticipantsFilterTitle: Writable<string> = writable(undefined);
export const pollCatalog: Writable<bool> = writable(false);
export const pollParticipantData: Writable<number> = writable(0);
export const disconnectProgress: Writable<number> = writable(0);
export const viewportHeight: Writable<number> = writable(0);

export const userManualKeyListener: Writable = writable(undefined);

