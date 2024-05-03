import { BIP32T, BIP32PathComponentT } from '../_types'

export type BIP44T = BIP32T &
	Readonly<{
		purpose: BIP32PathComponentT
		coinType: BIP32PathComponentT
		account: BIP32PathComponentT
		change: BIP32PathComponentT
		addressIndex: BIP32PathComponentT
	}>

export type BIP44ChangeIndex = 0 | 1

export type HDPathRadixT = BIP44T &
	Readonly<{
		purpose: {
			index: 0x8000002c
			isHardened: true
			toString: () => `44'`
			level: 1
			name: 'purpose'
		}
		coinType: {
			index: 0x80000218 // <=> 1022' dec
			isHardened: true
			toString: () => `1022'`
			level: 2
			name: 'coin type'
		}
		account: {
			isHardened: true
			level: 3
			name: 'account'
		}
		change: BIP32PathComponentT & {
			isHardened: true
			level: 4
			name: 'change'
		}
		addressIndex: BIP32PathComponentT & {
			level: 5
			name: 'address index'
		}
	}>
