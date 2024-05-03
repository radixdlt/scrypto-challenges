import {
	PrivateKeyT,
	Secp256k1,
	sha256,
	ECPointOnCurve,
	PublicKey,
	Signature,
	PrivateKey,
	KeyPair,
} from '../src'

import { UInt256 } from '@radixdlt/uint256'
import { signatureFromHexStrings } from './utils'
import { msgFromError } from '@radixdlt/util'

describe('elliptic curve cryptography', () => {
	it('knows the order of secp256l1', () => {
		expect(Secp256k1.order.toString(10)).toBe(
			'115792089237316195423570985008687907852837564279074904382605163141518161494337',
		)
	})

	it('0202...is a valid public key', () => {
		const publicKeyCompressedHexString = '02'.repeat(
			PublicKey.compressedByteCount,
		)
		PublicKey.fromBuffer(
			Buffer.from(publicKeyCompressedHexString, 'hex'),
		).match(
			s => {
				expect(s.toString(false)).toBe(
					'040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2',
				)
				expect(s.toString(true)).toBe(publicKeyCompressedHexString)
			},
			e => {
				throw new Error(
					`0202... is not a valid public key, but we expected it to be. Underlying error: ${msgFromError(
						e,
					)}`,
				)
			},
		)
	})

	describe('failing scenarios', () => {
		beforeAll(() => {
			jest.spyOn(console, 'error').mockImplementation(() => {})
		})

		afterAll(() => {
			jest.clearAllMocks()
		})

		it('0303...is not a valid public key', () => {
			const publicKeyCompressedHexString = '03'.repeat(
				PublicKey.compressedByteCount,
			)
			PublicKey.fromBuffer(
				Buffer.from(publicKeyCompressedHexString, 'hex'),
			).match(
				_ => {
					throw new Error(
						`We expected ${publicKeyCompressedHexString} to be invalid, but it is not.`,
					)
				},
				e => {
					expect(msgFromError(e)).toBe(
						`Failed to decode bytes into public key, underlying error: invalid point. bytes: '${publicKeyCompressedHexString}'`,
					)
				},
			)
		})
	})

	it('can securely generate private keys', () => {
		const privateKeys = [...Array(1024)]
			.map(_ => PrivateKey.generateNew())
			.map((privateKey: PrivateKeyT): string => privateKey.toString())
		const uniquePrivateKeys = new Set(privateKeys)
		// Probability of collision is: 2^10/2^256 <=> 1/2^246<=> Very very very very low probability.
		expect(uniquePrivateKeys.size).toBe(privateKeys.length)
	})

	it('should be able to sign messages', async () => {
		const privateKey = PrivateKey.fromScalar(
			UInt256.valueOf(1),
		)._unsafeUnwrap()

		const signatureResult = await privateKey.signUnhashed({
			msgToHash: 'Satoshi Nakamoto',
			hasher: sha256,
		})

		const signature = signatureResult._unsafeUnwrap()

		const r = signature.r.toString(16)
		const s = signature.s.toString(16)

		const expectedRHex =
			'934b1ea10a4b3c1757e2b0c017d0b6143ce3c9a7e6a4a49860d7a6ab210ee3d8'
		expect(r).toBe(expectedRHex)

		const expectedSHex =
			'2442ce9d2b916064108014783e923ec36b49743e2ffa1c4496f01a512aafd9e5'
		expect(s).toBe(expectedSHex)

		const derString =
			'3045022100934b1ea10a4b3c1757e2b0c017d0b6143ce3c9a7e6a4a49860d7a6ab210ee3d802202442ce9d2b916064108014783e923ec36b49743e2ffa1c4496f01a512aafd9e5'
		expect(signature.toDER()).toBe(derString)

		const signatureFromDER = Signature.fromDER(derString)._unsafeUnwrap()
		expect(signatureFromDER.toDER()).toBe(derString)
		expect(signatureFromDER.r.toString(16)).toBe(expectedRHex)

		expect(signatureFromDER.s.toString(16)).toBe(expectedSHex)
		expect(signature.equals(signatureFromDER)).toBe(true)
	})

	it('should be able to derive publicKey from privateKey', () => {
		const privateKey = PrivateKey.fromScalar(
			UInt256.valueOf(1),
		)._unsafeUnwrap()

		const publicKey = privateKey.publicKey()

		const compressedPubKey = publicKey
			.asData({ compressed: true })
			.toString('hex')

		expect(compressedPubKey).toBe(
			'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
		)

		const uncompressedPubKey = publicKey
			.asData({ compressed: false })
			.toString('hex')

		expect(uncompressedPubKey).toBe(
			'0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
		)

		const signature = signatureFromHexStrings({
			r:
				'934b1ea10a4b3c1757e2b0c017d0b6143ce3c9a7e6a4a49860d7a6ab210ee3d8',
			s:
				'2442ce9d2b916064108014783e923ec36b49743e2ffa1c4496f01a512aafd9e5',
		})

		const signatureValidation = publicKey.isValidSignature({
			signature: signature,
			hashedMessage: sha256('Satoshi Nakamoto'),
		})

		expect(signatureValidation).toBeTruthy()
	})

	it('can create a publicKey from bytes', () => {
		const publicKeyResult = PublicKey.fromBuffer(
			Buffer.from(
				'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
				'hex',
			),
		)

		const publicKey = publicKeyResult._unsafeUnwrap()

		const publicKeyUncompressed = publicKey
			.asData({ compressed: false })
			.toString('hex')

		expect(publicKeyUncompressed).toBe(
			'0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
		)
	})

	it('has G', () => {
		const g = Secp256k1.generator
		expect(g.x.toString(16)).toBe(
			'79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
		)
		expect(g.y.toString(16)).toBe(
			'483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
		)
	})

	it('G can mult with self', () => {
		const g = Secp256k1.generator
		const one = UInt256.valueOf(1)
		expect(g.multiply(one).equals(g)).toBe(true)
		const pubKey = PublicKey.fromPrivateKeyScalar({
			scalar: one,
		})
		expect(pubKey.decodeToPointOnCurve().equals(g)).toBe(true)
	})

	it('can do EC multiplication', () => {
		const keyPair = KeyPair.generateNew()
		const publicKey = keyPair.publicKey
		const privateKey = keyPair.privateKey
		const pubKeyPoint = publicKey.decodeToPointOnCurve()

		expect(
			Secp256k1.generator
				.multiplyWithPrivateKey(privateKey)
				.equals(pubKeyPoint),
		).toBe(true)
	})

	it('can do EC addition', () => {
		const g = Secp256k1.generator
		const two = UInt256.valueOf(2)
		const three = UInt256.valueOf(3)
		const five = UInt256.valueOf(5)
		const point2G = g.multiply(two)
		const point3G = g.multiply(three)
		const point5GByAddition = point2G.add(point3G)
		const point5GByMultiplication = g.multiply(five)
		expect(point5GByAddition.equals(point5GByMultiplication)).toBe(true)
	})

	it('can construct ECPoint from X and Y', () => {
		const manualG = ECPointOnCurve.fromXY({
			x: new UInt256(
				'79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
				16,
			),
			y: new UInt256(
				'483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
				16,
			),
		})._unsafeUnwrap()
		expect(manualG.equals(Secp256k1.generator)).toBe(true)

		const gBuf = manualG.toBuffer()
		const gFromBuf = ECPointOnCurve.fromBuffer(gBuf)._unsafeUnwrap()
		expect(gFromBuf.equals(Secp256k1.generator)).toBe(true)

		expect(manualG.toBuffer(true).slice(0, 1).readUInt8(0)).toBe(0x04)
		expect(manualG.toString(true)).toBe(
			'0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
		)
	})

	it('cannot construct points that is not on the curve', () => {
		ECPointOnCurve.fromXY({
			x: UInt256.valueOf(1337),
			y: UInt256.valueOf(1337),
		}).match(
			() => {
				throw Error('expected error, but got none')
			},
			e => expect(e.message).toBe(`Not point on curve!`),
		)
	})
})
