import {
	EncryptedMessageT,
	EncryptionScheme,
	ENCRYPTION_SCHEME_BYTES,
	MessageType,
	MESSAGE_TYPE_BYTES,
	PlaintextMessageT,
	SealedMessageT,
} from './_types'
import { combine, err, ok, Result } from 'neverthrow'
import { isString, readBuffer } from '@radixdlt/util'
import { SealedMessage } from './sealedMessage'
import { validateMaxLength, validateMinLength } from '../utils'
import { PublicKey } from '../elliptic-curve'
import { MessageEncryption } from './messageEncryption'

const maxLengthEncryptedMessage = 255

const minLengthEncryptedMessage =
	SealedMessage.authTagByteCount +
	SealedMessage.nonceByteCount +
	PublicKey.compressedByteCount +
	ENCRYPTION_SCHEME_BYTES +
	MESSAGE_TYPE_BYTES

const maxLengthOfCipherTextOfSealedMsg =
	maxLengthEncryptedMessage - minLengthEncryptedMessage

const isPlaintext = (rawHex: string) =>
	parseInt(rawHex.slice(0, 2)) === MessageType.PLAINTEXT

const isEncrypted = (rawHex: string) =>
	parseInt(rawHex.slice(0, 2)) === MessageType.ENCRYPTED

const isHexEncoded = (rawHex: string) =>
	parseInt(rawHex.slice(0, 2)) === MessageType.HEX

const __validateEncryptedMessageMaxLength: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateMaxLength.bind(
	null,
	maxLengthEncryptedMessage,
	'encryptedMessage',
)

const __validateEncryptedMessageMinLength: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateMinLength.bind(
	null,
	minLengthEncryptedMessage,
	'encryptedMessage',
)

export const __validateEncryptedMessageLength = (
	buffer: Buffer,
): Result<Buffer, Error> =>
	combine([
		__validateEncryptedMessageMaxLength(buffer),
		__validateEncryptedMessageMinLength(buffer),
	]).map(_ => buffer)

const createEncrypted = (
	encryptionScheme: EncryptionScheme,
	sealedMessage: SealedMessageT,
): Result<EncryptedMessageT, Error> =>
	__validateEncryptedMessageLength(
		Buffer.concat([
			Buffer.from([MessageType.ENCRYPTED]),
			Buffer.from([encryptionScheme]),
			sealedMessage.combined(),
		]),
	).map(combinedBuffer => ({
		kind: 'ENCRYPTED',
		messageType: MessageType.ENCRYPTED,
		encryptionScheme,
		sealedMessage,
		combined: (): Buffer => combinedBuffer,
	}))

const createPlaintext = (message: string | Buffer): PlaintextMessageT => ({
	kind: 'PLAINTEXT',
	plaintext: isString(message) ? message : message.toString('utf8'),
	bytes: Buffer.concat([
		Buffer.from([MessageType.PLAINTEXT]),
		Buffer.from([EncryptionScheme.NONE]),
		MessageEncryption.encodePlaintext(message),
	]),
})

const plaintextToString = (plaintext: Buffer, startAt = 2) =>
	Buffer.from(plaintext.slice(startAt).toString('hex'), 'hex').toString(
		'utf-8',
	)

const fromBuffer = (
	buf: Buffer,
): Result<EncryptedMessageT | PlaintextMessageT, Error> =>
	__validateEncryptedMessageLength(buf).andThen(
		(buffer): Result<EncryptedMessageT | PlaintextMessageT, Error> => {
			const readNextBuffer = readBuffer(buf)

			const messageTypeResult = readNextBuffer(MESSAGE_TYPE_BYTES)
			if (messageTypeResult.isErr()) return err(messageTypeResult.error)

			const messageType = messageTypeResult.value.readUIntBE(0, 1)
			if (!(messageType in MessageType))
				return err(Error(`Unknown message type: ${messageType}`))

			const schemeResult = readNextBuffer(ENCRYPTION_SCHEME_BYTES)
			if (schemeResult.isErr()) return err(schemeResult.error)

			const scheme = schemeResult.value.readUIntBE(0, 1)
			if (!(scheme in EncryptionScheme))
				return err(Error(`Unknown encryption scheme: ${scheme}`))

			const payloadResult = readNextBuffer(
				buffer.length - ENCRYPTION_SCHEME_BYTES - MESSAGE_TYPE_BYTES,
			)
			if (payloadResult.isErr()) return err(payloadResult.error)

			const payload = payloadResult.value

			if (
				messageType === MessageType.ENCRYPTED &&
				scheme !== EncryptionScheme.NONE
			) {
				const sealedMessageResult = SealedMessage.fromBuffer(payload)
				if (sealedMessageResult.isErr())
					return err(sealedMessageResult.error)

				return createEncrypted(scheme, sealedMessageResult.value)
			}

			if (
				messageType === MessageType.PLAINTEXT &&
				scheme === EncryptionScheme.NONE
			) {
				return ok(createPlaintext(payload))
			}

			return err(
				Error(
					`Invalid combination of message type ${messageType} and encryption scheme ${scheme}.`,
				),
			)
		},
	)

export const Message = {
	maxLength: maxLengthEncryptedMessage,
	maxLengthOfCipherTextOfSealedMsg,
	minLengthEncryptedMessage,
	createEncrypted,
	createPlaintext,
	fromBuffer,
	plaintextToString,
	isPlaintext,
	isEncrypted,
	isHexEncoded,
}
