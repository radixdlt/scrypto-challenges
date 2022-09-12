import { writable } from 'svelte/store';
import { PfpSource } from './participant.ts'

export const unendorseStore: Writable<Set<string>> = writable(new Set<string>());
export const endorseStore: Writable<Set<string>> = writable(new Set<string>());
export const unsponsorStore: Writable<Set<string>> = writable(new Set<string>());
export const sponsorStore: Writable<Set<string>> = writable(new Set<string>());
//export const unexpectSponsorStore: Writable<string> = writable(undefined);
export const expectSponsorStore: Writable<string> = writable(undefined);
export const editedPfpSeries: Writable<PfpSource> = writable(undefined);
export const editedPfpId: Writable<string> = writable(undefined);
export const editedUrl: Writable<string> = writable(undefined);

export const participantsDirty: Writable<bool> = writable(false);
export const isPfpDirty: Writable<bool> = writable(false);
