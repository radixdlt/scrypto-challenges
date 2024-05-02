import bip39 from 'bip39'
import { ok } from 'neverthrow'
import { derivePath, getPublicKey } from 'ed25519-hd-key'
import { secureRandom } from './secure-random'

export const generateMnemonic = () => bip39.entropyToMnemonic(secureRandom(32))

const mnemonicToSeed = (mnemonic: string) =>
  ok(bip39.mnemonicToSeedSync(mnemonic).toString('hex'))

const deriveChildKey = (derivationPath: string, seedHex: string) =>
  ok(derivePath(derivationPath, seedHex))

export const mnemonicToKeyPair = (mnemonic: string, derivationPath: string) =>
  mnemonicToSeed(mnemonic)
    .andThen((seedHex: string) => deriveChildKey(derivationPath, seedHex))
    .map(({ key }) => ({
      privateKey: key.toString('hex'),
      publicKey: getPublicKey(key, false).toString('hex'),
    }))
