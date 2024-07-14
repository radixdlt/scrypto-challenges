import { Buffer } from 'buffer';

export const createChallenge = () => Buffer.from(crypto.getRandomValues(new Uint8Array(32))).toString('hex');
