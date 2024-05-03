import {
	EncryptedMessageT,
	Message,
	MessageEncryption,
	KeyPair,
	PublicKeyT,
	DiffieHellman,
	PrivateKey,
} from '../src'
import { UInt256 } from '@radixdlt/uint256'
import { PrivateKeyT } from '../src'
import * as c from "@radixdlt/crypto";
import * as a from "@radixdlt/application";
import { PrivateKey as PrivKey } from "@radixdlt/radix-engine-toolkit"
import { signatureFromHexStrings } from './utils';
import { TextEncoder, TextDecoder } from 'util'

global.TextEncoder = TextEncoder
// @ts-ignore
global.TextDecoder = TextDecoder

describe('message encryption', () => {
	describe('can decrypt newly encrypted message', () => {
		const alice = KeyPair.generateNew()
		const bob = KeyPair.generateNew()

		const aliceEncryptTo = async (
			to: PublicKeyT,
			plaintext: string,
		): Promise<EncryptedMessageT> => {
			const res = await MessageEncryption.encrypt({
				plaintext,
				diffieHellmanPoint: alice.privateKey.diffieHellman.bind(
					null,
					to,
				),
			})
			return res._unsafeUnwrap()
		}

		const aliceEncryptToBob = aliceEncryptTo.bind(null, bob.publicKey)
		const aliceEncryptToSelf = aliceEncryptTo.bind(null, alice.publicKey)

		const decrypt = async (
			diffieHellman: DiffieHellman,
			publicKeyOfOtherParty: PublicKeyT,
			encryptedMessage: EncryptedMessageT,
		): Promise<string> => {
			const res = await MessageEncryption.decrypt({
				encryptedMessage,
				diffieHellmanPoint: diffieHellman.bind(
					null,
					publicKeyOfOtherParty,
				),
			}).map(b => b.toString('utf-8'))
			return res._unsafeUnwrap()
		}

		const aliceDecryptWithBobsPubKey = decrypt.bind(
			null,
			alice.privateKey.diffieHellman,
			bob.publicKey,
		)

		const aliceDecryptWithHerOwnPubKey = decrypt.bind(
			null,
			alice.privateKey.diffieHellman,
			alice.publicKey,
		)

		const bobDecrypt = decrypt.bind(
			null,
			bob.privateKey.diffieHellman,
			alice.publicKey,
		)

		it('encrypted message can be decrypted by both sender and receiver', async () => {
			const plaintext = 'Hey Bob!'
			const encryptedMessage = await aliceEncryptToBob(plaintext)
			expect(encryptedMessage.combined().length).toBeLessThanOrEqual(
				Message.maxLength,
			)
			const decryptedByAlice = await aliceDecryptWithBobsPubKey(
				encryptedMessage,
			)
			expect(decryptedByAlice).toBe(plaintext)

			const decryptedByBob = await bobDecrypt(encryptedMessage)
			expect(decryptedByBob).toBe(plaintext)

			expect(decryptedByBob).toBe(decryptedByAlice)
		})

		it('encrypted message to self can be decrypted by self', async () => {
			const plaintext = 'Hey Bob!'
			const encryptedMessage = await aliceEncryptToSelf(plaintext)
			expect(encryptedMessage.combined().length).toBeLessThanOrEqual(
				Message.maxLength,
			)
			const decryptedByAlice = await aliceDecryptWithHerOwnPubKey(
				encryptedMessage,
			)
			expect(decryptedByAlice).toBe(plaintext)
		})
	})

	describe('plaintext to encrypt cannot be too long', () => {
		const alicePrivateKey = PrivateKey.fromScalar(
			UInt256.valueOf(1),
		)._unsafeUnwrap()
		const bobPrivateKey = PrivateKey.fromScalar(
			UInt256.valueOf(2),
		)._unsafeUnwrap()
		const bob = bobPrivateKey.publicKey()

		it('throws error if plaintext is too long', done => {
			const tooLongMsg =
				'too long message that is too long because it is over characters limit which is too long to encrypt because it cannot fit because it is too long indeed. Which is why it cannot be encrypted. So expect an error to be thrown.'

			expect(tooLongMsg.length).toBeGreaterThan(
				Message.maxLengthOfCipherTextOfSealedMsg,
			)

			MessageEncryption.encrypt({
				plaintext: tooLongMsg,
				diffieHellmanPoint: alicePrivateKey.diffieHellman.bind(
					null,
					bob,
				),
			}).match(
				_ => {
					done(new Error('Expected failure.'))
				},
				error => {
					expect(error.message).toBe(
						`Plaintext is too long, expected max #${Message.maxLengthOfCipherTextOfSealedMsg}, but got: #${tooLongMsg.length}`,
					)
					done()
				},
			)
		})
	})

	describe('can decrypt message from buffer that was encrypted earlier', () => {
		const alicePrivateKey = PrivateKey.fromScalar(
			UInt256.valueOf(1),
		)._unsafeUnwrap()
		const alice = alicePrivateKey.publicKey()
		const bobPrivateKey = PrivateKey.fromScalar(
			UInt256.valueOf(2),
		)._unsafeUnwrap()
		const bob = bobPrivateKey.publicKey()

		// encrypted by outcommented test: 'alice can encrypt msg to bob' below
		const encryptedMessageByAliceToBobBuf = Buffer.from(
			'01ff02663a6aaf4d5ec607330b9b74a840bf5c13b0a7357202fa85be56b1326065561657d6ee46d4d84e94ec615b425a472dd8c813bad125335a097d29b64b72319357406b2b04491b4ca1a5a05fe8772b0c05f4633b399914348c5b03af58445d42c2f740f8407e572775a571805e582c6b96ffd4ccca764f2002510abddaab735ee4fb0b18c26d',
			'hex',
		)

		const plaintext =
			'Hey Bob, this is Alice, you and I can read this message, but no one else.'

		type Decryptor = 'alice' | 'bob'

		const doTestDecrypt = async (
			decryptor: Decryptor,
			done: jest.DoneCallback,
		): Promise<void> => {
			const publicKeyOfOtherParty: PublicKeyT =
				decryptor === 'bob' ? alice : bob
			const privKey: PrivateKeyT =
				decryptor === 'bob' ? bobPrivateKey : alicePrivateKey
			const diffieHellman = privKey.diffieHellman

			await MessageEncryption.decrypt({
				encryptedMessage: encryptedMessageByAliceToBobBuf,
				diffieHellmanPoint: diffieHellman.bind(
					null,
					publicKeyOfOtherParty,
				),
			}).match(
				decrypted => {
					expect(decrypted.toString('utf8')).toBe(plaintext)
					done()
				},
				error => {
					done(
						new Error(
							`Failed to decrypted, but expected to be able to decrypt, got error: ${error}`,
						),
					)
				},
			)
		}

		it('alice can decrypt msg encrypted by herself', done => {
			doTestDecrypt('alice', done)
		})

		it('bob can decrypt msg encrypted by alice', done => {
			doTestDecrypt('bob', done)
		})

		// PLEASE SAVE, convenient to have.
		//it('alice can encrypt msg to bob', async (done) => {
		//	await MessageEncryption.encrypt({
		//		plaintext,
		//		diffieHellmanPoint: alicePrivateKey.diffieHellman.bind(
		//			null,
		//			bob,
		//		)
		//	}).match(
		//		(msg) => {
		//			console.log(`🔮 msg combined: ${msg.combined().toString('hex')}`)
		//			expect(msg.combined().toString('hex')).toBe(encryptedMessageByAliceToBobBuf.toString('hex'))
		//			//expect(msg.encryptionScheme.equals(EncryptionScheme.current)).toBe(true)
		//			done()
		//		},
		//		(error) => {
		//			done(new Error(`Expected to be able to encrypt message, but got error: ${error}`))
		//		}
		//	)
		//})
	})

	it('should convert to a valid babylon private key', async () => {
		const HD_PATH = "m/44'/1022'/0'/0/0";

		const mnemonic = c.Mnemonic.fromEnglishPhrase('empty only laundry young sure sadness family beef rotate local web unfair')._unsafeUnwrap()

		const hdMasterSeed = c.HDMasterSeed.fromMnemonic({ mnemonic })

		const olympiaSigningKey = a.SigningKey.fromHDPathWithHDMasterSeed({
			hdPath: c.HDPathRadix.fromString(HD_PATH)._unsafeUnwrap(),
			hdMasterSeed
		})

		const olympiaKeyPair = hdMasterSeed.masterNode().derive(c.HDPathRadix.fromString(HD_PATH)._unsafeUnwrap())

		const babylonPrivateKey = new PrivKey.Secp256k1(olympiaKeyPair.privateKey.toString())

		const secret = c.sha256('secret');

		const _signature = babylonPrivateKey.signToSignature(secret).bytes

		const r = Buffer.from(_signature.slice(1, 33)).toString('hex')
		const s = Buffer.from(_signature.slice(33, 65)).toString('hex')

		const signature = signatureFromHexStrings({ r, s })

		const isValid = olympiaSigningKey.publicKey.isValidSignature(
			{
				signature,
				hashedMessage: secret
			}
		)

		expect(isValid).toEqual(true)
	})
})
