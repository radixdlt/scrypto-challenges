import { writable } from 'svelte/store';
import type { Participant } from './participant.ts';

export const walletAddress: Writable<string> = writable(undefined);
export const userNfid: Writable<string> = writable(undefined);
export const participantNftCount: Writable<number> = writable(0);
export const promiseParticipants: Writable<Map<string, Promise>> = writable(new Map());
export const allParticipants: Writable<Map<string, Participant>> = writable(new Map());
export const loadedParticipantCount: Writable<number> = writable(0);
export const tourist: Writable<bool> = writable(false);
