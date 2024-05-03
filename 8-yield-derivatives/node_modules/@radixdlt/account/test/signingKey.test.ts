import {
	ECPointOnCurveT,
	HDMasterSeed,
	HDPathRadix,
	HDPathRadixT,
	MessageEncryption,
	Mnemonic,
	PrivateKey,
	PublicKey,
	PublicKeyT,
	sha256Twice,
	Signature,
	SignatureT,
} from '@radixdlt/crypto'
import { UInt256 } from '@radixdlt/uint256'
import {
	AccountAddress,
	HDSigningKeyTypeIdentifier,
	SigningKey,
	SigningKeyT,
	SigningKeyTypeHDT,
} from '../src'
import { Observable, Subscription, throwError } from 'rxjs'
import { toObservable } from '@radixdlt/util'
import { HardwareSigningKeyT } from '@radixdlt/hardware-wallet'
import { Network } from '@radixdlt/primitives'

const privateKeyFromNum = (privateKeyScalar: number) =>
	PrivateKey.fromScalar(UInt256.valueOf(privateKeyScalar))._unsafeUnwrap()

describe('signingKey_type', () => {
	it('works', () => {
		const mnemonic = Mnemonic.fromEnglishPhrase(
			'equip will roof matter pink blind book anxiety banner elbow sun young',
		)._unsafeUnwrap()
		const hdMasterSeed = HDMasterSeed.fromMnemonic({ mnemonic })
		const hdPath = HDPathRadix.fromString(
			`m/44'/1022'/2'/1/3'`,
		)._unsafeUnwrap()

		const signingKey = SigningKey.fromHDPathWithHDMasterSeed({
			hdPath,
			hdMasterSeed,
		})

		expect(signingKey.isHDSigningKey).toBe(true)
		expect(signingKey.isLocalHDSigningKey).toBe(true)
		expect(signingKey.isHardwareSigningKey).toBe(false)

		expect(signingKey.hdPath!.equals(hdPath)).toBe(true)

		expect(signingKey.publicKey.toString(true)).toBe(
			'034d2fd914bb6045f58b239d6949dd35e73bc67a67fe9668ed0d9c05affe1c122b',
		)
	})

	it('radix_hd_path_hardened', async done => {
		const mnemonic = Mnemonic.fromEnglishPhrase(
			'equip will roof matter pink blind book anxiety banner elbow sun young',
		)._unsafeUnwrap()
		const hdMasterSeed = HDMasterSeed.fromMnemonic({ mnemonic })
		const hdPath = HDPathRadix.fromString(
			`m/44'/1022'/2'/1/3`,
		)._unsafeUnwrap()

		const signingKey = SigningKey.fromHDPathWithHDMasterSeed({
			hdPath,
			hdMasterSeed,
		})

		expect(signingKey.publicKey.toString(true)).toBe(
			'03d79039c428a6b835e136fbb582e9259df23f8660f928367c3f0d6912728a8444',
		)

		const accountAddress = AccountAddress.fromPublicKeyAndNetwork({
			publicKey: signingKey.publicKey,
			network: Network.MAINNET,
		})
		expect(accountAddress.toString()).toBe(
			'rdx1qspa0ypecs52dwp4uym0hdvzayjemu3lses0j2pk0sls66gjw29gg3q0cpfrg',
		)

		const otherPubKey = PublicKey.fromBuffer(
			Buffer.from(
				'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
				'hex',
			),
		)._unsafeUnwrap()
		const keyExchange = (
			await signingKey.__diffieHellman(otherPubKey)
		)._unsafeUnwrap()
		expect(keyExchange.toString()).toBe(
			'd79039c428a6b835e136fbb582e9259df23f8660f928367c3f0d6912728a8444a87d3a07191942666ca3d0396374531fe669e451bae6eeb79fb0884ef78a2f9d',
		)

		const message = `I'm testing Radix awesome hardware wallet!`
		const hashedMessage = sha256Twice(message)
		expect(hashedMessage.toString('hex')).toBe(
			'be7515569e05daffc71bffe2a30365b74450c017a56184ee26699340a324d402',
		)

		const subs = new Subscription()

		subs.add(
			signingKey.signHash(hashedMessage).subscribe(sig => {
				expect(sig.toDER()).toBe(
					'3045022100de5f8c5a92cc5bea386d7a5321d0aa1b46fc7d90c5d07098346252aacd59e52302202bbaeef1256d0185b550a7b661557eea11bb98b99ccc7e01d19fd931e617e824',
				)

				expect(sig.r.toString(16)).toBe(
					'de5f8c5a92cc5bea386d7a5321d0aa1b46fc7d90c5d07098346252aacd59e523',
				)
				expect(sig.s.toString(16)).toBe(
					'2bbaeef1256d0185b550a7b661557eea11bb98b99ccc7e01d19fd931e617e824',
				)

				const rsBufHex =
					'de5f8c5a92cc5bea386d7a5321d0aa1b46fc7d90c5d07098346252aacd59e5232bbaeef1256d0185b550a7b661557eea11bb98b99ccc7e01d19fd931e617e824'

				const sigFromRS = Signature.fromRSBuffer(
					Buffer.from(rsBufHex, 'hex'),
				)._unsafeUnwrap()

				expect(sigFromRS.toDER()).toBe(
					'3045022100de5f8c5a92cc5bea386d7a5321d0aa1b46fc7d90c5d07098346252aacd59e52302202bbaeef1256d0185b550a7b661557eea11bb98b99ccc7e01d19fd931e617e824',
				)
				expect(sigFromRS.equals(sig)).toBe(true)
				expect(sig.equals(sigFromRS)).toBe(true)

				done()
			}),
		)
	})

	it('radix_hd_path_0H00', () => {
		const mnemonic = Mnemonic.fromEnglishPhrase(
			'equip will roof matter pink blind book anxiety banner elbow sun young',
		)._unsafeUnwrap()
		const hdMasterSeed = HDMasterSeed.fromMnemonic({ mnemonic })
		const hdPath0H00 = HDPathRadix.fromString(
			`m/44'/1022'/0'/0/0`,
		)._unsafeUnwrap()

		const signingKey0H00 = SigningKey.fromHDPathWithHDMasterSeed({
			hdPath: hdPath0H00,
			hdMasterSeed,
		})

		expect(signingKey0H00.publicKey.toString(true)).toBe(
			'03bc2ec8f3668c869577bf66b7b48f8dee57b833916aa70966fa4a5029b63bb18f',
		)

		const hdPath0H00H = HDPathRadix.fromString(
			`m/44'/1022'/0'/0/0'`,
		)._unsafeUnwrap()

		const signingKey0H00H = SigningKey.fromHDPathWithHDMasterSeed({
			hdPath: hdPath0H00H,
			hdMasterSeed,
		})

		expect(signingKey0H00H.publicKey.toString(true)).toBe(
			'03f43fba6541031ef2195f5ba96677354d28147e45b40cde4662bec9162c361f55',
		)
	})

	it('can create accounts from private key', () => {
		const signingKey = SigningKey.fromPrivateKey({
			privateKey: privateKeyFromNum(1),
		})

		const expPubKey =
			'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'
		expect(signingKey.publicKey.toString(true)).toBe(expPubKey)
		expect(signingKey.uniqueIdentifier).toBe(`Non_hd_pubKey${expPubKey}`)
		expect(signingKey.isHDSigningKey).toBe(false)
		expect(signingKey.isLocalHDSigningKey).toBe(false)
		expect(signingKey.isHardwareSigningKey).toBe(false)
	})

	it('hw signingKey', done => {
		const subs = new Subscription()

		const mockHardwareSigningKey = (
			hdPath: HDPathRadixT,
			hardwareMnemonic?: string,
		): HardwareSigningKeyT => {
			const mnemonic = Mnemonic.fromEnglishPhrase(
				hardwareMnemonic ??
					'equip will roof matter pink blind book anxiety banner elbow sun young',
			)._unsafeUnwrap()
			const masterSeed = HDMasterSeed.fromMnemonic({ mnemonic })

			const signingKeyLocalHD = SigningKey.byDerivingNodeAtPath({
				hdPath,
				deriveNodeAtPath: () => masterSeed.masterNode().derive(hdPath),
			})
			return {
				keyExchange: (
					publicKeyOfOtherParty: PublicKeyT,
				): Observable<ECPointOnCurveT> => {
					return toObservable(
						signingKeyLocalHD.__diffieHellman(
							publicKeyOfOtherParty,
						),
					)
				},
				getPublicKeyDisplayOnlyAddress: () =>
					throwError(new Error('Not impl')),
				publicKey: signingKeyLocalHD.publicKey,
				signHash: (hashedMessage: Buffer): Observable<SignatureT> => {
					return signingKeyLocalHD.signHash(hashedMessage)
				},
				sign: _ => {
					throw new Error('not implemented')
				},
			}
		}

		const path = `m/44'/1022'/2'/1/3`

		const hdPath = HDPathRadix.fromString(path)._unsafeUnwrap()

		const bobPrivateKey = PrivateKey.fromScalar(
			UInt256.valueOf(1),
		)._unsafeUnwrap()
		const bobPubKey = bobPrivateKey.publicKey()

		const expectedPublicKey =
			'03d79039c428a6b835e136fbb582e9259df23f8660f928367c3f0d6912728a8444'

		const hwSigningKey = SigningKey.fromHDPathWithHWSigningKey({
			hdPath,
			hardwareSigningKey: mockHardwareSigningKey(hdPath),
		})
		expect(hwSigningKey.isHDSigningKey).toBe(true)
		expect(hwSigningKey.isHardwareSigningKey).toBe(true)
		expect(hwSigningKey.isLocalHDSigningKey).toBe(false)
		expect(hwSigningKey.hdPath!.toString()).toBe(path)
		expect(hwSigningKey.publicKey.toString(true)).toBe(expectedPublicKey)
		expect((hwSigningKey.type as SigningKeyTypeHDT).hdSigningKeyType).toBe(
			HDSigningKeyTypeIdentifier.HARDWARE_OR_REMOTE,
		)
		expect(hwSigningKey.uniqueIdentifier).toBe(
			`Hardware_HD_signingKey_at_path_m/44'/1022'/2'/1/3`,
		)
		expect(
			hwSigningKey.equals(<SigningKeyT>{
				publicKey: PublicKey.fromBuffer(
					Buffer.from(expectedPublicKey, 'hex'),
				)._unsafeUnwrap(),
			}),
		).toBe(true)

		const plaintext = 'Hello Bob!'

		subs.add(
			hwSigningKey
				.encrypt({
					plaintext,
					publicKeyOfOtherParty: bobPubKey,
				})
				.subscribe(encryptedMessage => {
					MessageEncryption.decrypt({
						encryptedMessage,
						diffieHellmanPoint: bobPrivateKey.diffieHellman.bind(
							null,
							hwSigningKey.publicKey,
						),
					}).match(
						decrypted => {
							expect(decrypted.toString('utf8')).toBe(plaintext)
							done()
						},
						error => {
							done(error)
						},
					)
				}),
		)
	})
})
