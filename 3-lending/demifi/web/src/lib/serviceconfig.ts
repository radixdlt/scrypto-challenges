import { readable } from 'svelte/store';

// Package Address: 01e270219686302a2f17359021a039396b5c80ccbf52c33d9c40a0
// Participants Component Address: 0292f7cc7546827367f3d442b4b67503d9f4d681c6d09f5e95cb87
// Participants NFT Resource Address: 03049bfec3b33134e638b569de9d4e15c9fff63ef8676c83b7bbe2
// Catalog owner NFT id: 1dcfad7cd5ebd724437dc639cc079afb

export const gatewayNodeUrl: Readable<string> = readable('https://pte01.radixdlt.com');
export const participantsComponentAddress: Readable<string> = readable('026b3faf9670ffbb652991959e4a96d2025745d3cc629272e2a468');
export const participantsNftResourceAddress: Readable<string> = readable('03c1e25c8f2e27a3def3b97290fd10ee3141b84ac2982fe5a12b7f');
