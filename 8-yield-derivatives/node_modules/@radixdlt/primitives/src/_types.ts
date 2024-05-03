import { UInt256 } from '@radixdlt/uint256'

export type AmountT = UInt256

export enum Network {
	MAINNET = 'mainnet',
	STOKENET = 'stokenet',
	LOCALNET = 'localnet',
	MILESTONENET = 'milestonenet',
	RELEASENET = 'releasenet',
	RCNET = 'rcnet',
	TESTNET6 = 'testnet6',
	SANDPITNET = 'sandpitnet',
}

export const NetworkId = {
	1: Network.MAINNET,
	2: Network.STOKENET,
	3: Network.RELEASENET,
	4: Network.RCNET,
	5: Network.MILESTONENET,
	6: Network.TESTNET6,
	7: Network.SANDPITNET,
	99: Network.LOCALNET,
}

export const hrpFullSuffixLength = 3

export const HRP = {
	[Network.MAINNET]: {
		account: 'rdx',
		validator: 'rv',
		RRI_suffix: '_rr',
	},
	[Network.STOKENET]: {
		account: 'tdx',
		validator: 'tv',
		RRI_suffix: '_tr',
	},
	[Network.LOCALNET]: {
		account: 'ddx',
		validator: 'dv',
		RRI_suffix: '_dr',
	},
	[Network.RELEASENET]: {
		account: 'tdx3',
		validator: 'tv3',
		RRI_suffix: '_tr3',
	},
	[Network.RCNET]: {
		account: 'tdx4',
		validator: 'tv4',
		RRI_suffix: '_tr4',
	},
	[Network.MILESTONENET]: {
		account: 'tdx5',
		validator: 'tv5',
		RRI_suffix: '_tr5',
	},
	[Network.TESTNET6]: {
		account: 'tdx6',
		validator: 'tv6',
		RRI_suffix: '_tr6',
	},
	[Network.SANDPITNET]: {
		account: 'tdx7',
		validator: 'tv7',
		RRI_suffix: '_tr7',
	},
}

export type BuiltTransactionReadyToSign = Readonly<{
	// Bytes on hex format
	blob: string
	hashOfBlobToSign: string
}>
