import { err, ok, Result } from 'neverthrow'
import { LanguageT, MnemomicT, MnemonicProps, StrengthT } from './_types'
import {
	entropyToMnemonic,
	mnemonicToEntropy,
	validateMnemonic,
	wordlists,
} from 'bip39'
import {
	log,
	buffersEquals,
	SecureRandom,
	secureRandomGenerator,
	ValidationWitness,
	msgFromError,
} from '@radixdlt/util'

export const wordlistFromLanguage = (language: LanguageT): string[] => {
	const key = LanguageT[language].toLowerCase()
	return wordlists[key]
}

export const languagesSupportedByBIP39: LanguageT[] = [
	LanguageT.CZECH,
	LanguageT.CHINESE_SIMPLIFIED,
	LanguageT.CHINESE_TRADITIONAL,
	LanguageT.KOREAN,
	LanguageT.FRENCH,
	LanguageT.ITALIAN,
	LanguageT.SPANISH,
	LanguageT.JAPANESE,
	LanguageT.ENGLISH,
]

export const mnemonicStrengthSupportedByBIP39: StrengthT[] = [
	StrengthT.WORD_COUNT_12,
	StrengthT.WORD_COUNT_15,
	StrengthT.WORD_COUNT_18,
	StrengthT.WORD_COUNT_21,
	StrengthT.WORD_COUNT_24,
]

const separator = ' '

export const strengthFromWordCount = (
	wordCount: number,
): Result<StrengthT, Error> =>
	wordCount === 24
		? ok(StrengthT.WORD_COUNT_24)
		: wordCount === 21
		? ok(StrengthT.WORD_COUNT_21)
		: wordCount === 18
		? ok(StrengthT.WORD_COUNT_18)
		: wordCount === 15
		? ok(StrengthT.WORD_COUNT_15)
		: wordCount === 12
		? ok(StrengthT.WORD_COUNT_12)
		: err(Error(`Unsupported wordcount ${wordCount}`))

export const entropyInBitsFromWordCount = (wordCount: number): number => {
	const checksumBitsPerWord = 3
	return (wordCount / checksumBitsPerWord) * 32
}

export const byteCountFromEntropyStrength = (strenght: StrengthT): number =>
	strenght.valueOf() / 8

const create = (input: MnemonicProps): Result<MnemomicT, Error> => {
	const wordCount = input.words.length
	return strengthFromWordCount(wordCount)
		.andThen(
			(strengthFromWC: StrengthT): Result<ValidationWitness, Error> => {
				if (strengthFromWC !== input.strength) {
					const errMsg = `Mismatch between 'words' and 'strenght'.`
					log.error(errMsg)
					return err(new Error(errMsg))
				}
				if (
					entropyInBitsFromWordCount(wordCount) !==
					input.entropy.length * 8
				) {
					const errMsg = `Mismatch 'words' and 'entropy'.`
					log.error(errMsg)
					return err(new Error(errMsg))
				}
				if (
					byteCountFromEntropyStrength(input.strength) !==
					input.entropy.length
				) {
					const errMsg = `Mismatch 'strength' and 'entropy'.`
					log.error(errMsg)
					return err(new Error(errMsg))
				}

				const wordlist = wordlistFromLanguage(input.language)
				const languageName: string = LanguageT[input.language]

				if (input.words.join(separator) !== input.phrase) {
					const errMsg = `Mismatch between 'words' and 'phrase' ('phrase' possible non normalized (NFKD).).`
					log.error(errMsg)
					return err(new Error(errMsg))
				}

				for (const word of input.words) {
					if (!wordlist.includes(word)) {
						const errMsg = `Mismatch between 'words' and 'language'`
						log.error(errMsg)
						log.debug(
							`The word '${word}' was not found in mnemonic word list for language '${languageName}'`,
						)
						return err(new Error(errMsg))
					}
				}

				return ok({ witness: 'valid input' })
			},
		)
		.map(
			(_): MnemomicT => ({
				...input,
				toString: () => input.phrase,
				equals: (other: MnemomicT): boolean =>
					buffersEquals(input.entropy, other.entropy),
			}),
		)
		.map(mnemonic => {
			log.debug(`Successfully created mnemonic.`)
			return mnemonic
		})
}

const fromEntropyAndMaybeStrength = (
	input: Readonly<{
		entropy: Buffer
		strength?: StrengthT
		language?: LanguageT
	}>,
): Result<MnemomicT, Error> => {
	const language = input?.language ?? LanguageT.ENGLISH
	const wordlist = wordlistFromLanguage(language)
	const phrase = entropyToMnemonic(input.entropy, wordlist)
	if (!validateMnemonic(phrase, wordlist))
		throw new Error(
			'Incorrect implementation, should be able to always generate valid mnemonic',
		)

	const normalizedPhrase = phrase.normalize('NFKD')
	const words = normalizedPhrase.split(separator)

	const strengthOf: Result<StrengthT, Error> =
		input.strength !== undefined
			? ok(input.strength)
			: strengthFromWordCount(words.length)

	return strengthOf.andThen(strength =>
		create({
			...input,
			language,
			strength,
			phrase: normalizedPhrase,
			words,
		}),
	)
}

const fromEntropy = (
	input: Readonly<{
		entropy: Buffer
		language?: LanguageT
	}>,
): Result<MnemomicT, Error> => fromEntropyAndMaybeStrength(input)

const generateNew = (
	input?: Readonly<{
		strength?: StrengthT // defaults to 12 words (128 bits)
		language?: LanguageT // defaults to English
		secureRandom?: SecureRandom // defaults to default
	}>,
): MnemomicT => {
	const strength = input?.strength ?? StrengthT.WORD_COUNT_12
	const secureRandom = input?.secureRandom ?? secureRandomGenerator
	const entropyByteCount = byteCountFromEntropyStrength(strength)
	const entropy = Buffer.from(
		secureRandom.randomSecureBytes(entropyByteCount),
		'hex',
	)
	return fromEntropyAndMaybeStrength({
		...input,
		entropy,
		strength,
	})._unsafeUnwrap()
}

const fromPhraseInLanguage = (
	input: Readonly<{
		phrase: string
		language: LanguageT
	}>,
): Result<MnemomicT, Error> => {
	const wordlist = wordlistFromLanguage(input.language)
	const phrase = input.phrase

	let entropy: Buffer
	try {
		entropy = Buffer.from(mnemonicToEntropy(phrase, wordlist), 'hex')
	} catch (e) {
		const errMsg = msgFromError(e)
		if (errMsg === 'Invalid mnemonic checksum') {
			const notChecksummedErr = 'Invalid mnemonic, it is not checksummed.'
			log.error(notChecksummedErr)
			return err(new Error(notChecksummedErr))
		}

		return err(e as Error)
	}
	const normalizedPhrase = phrase.normalize('NFKD')
	const words = normalizedPhrase.split(separator)

	return strengthFromWordCount(words.length)
		.map(strength => ({
			...input,
			phrase: normalizedPhrase,
			words,
			entropy,
			strength,
		}))
		.andThen(create)
}

const fromWordsInLanguage = (
	input: Readonly<{
		words: string[]
		language: LanguageT
	}>,
): Result<MnemomicT, Error> =>
	fromPhraseInLanguage({
		phrase: input.words.join(separator),
		language: input.language,
	})

const fromEnglishPhrase = (phrase: string): Result<MnemomicT, Error> =>
	fromPhraseInLanguage({
		phrase,
		language: LanguageT.ENGLISH,
	})

const fromEnglishWords = (words: string[]): Result<MnemomicT, Error> =>
	fromWordsInLanguage({
		words,
		language: LanguageT.ENGLISH,
	})

export const Mnemonic = {
	generateNew,
	fromEntropy,
	fromPhraseInLanguage,
	fromWordsInLanguage,
	fromEnglishPhrase,
	fromEnglishWords,
}
