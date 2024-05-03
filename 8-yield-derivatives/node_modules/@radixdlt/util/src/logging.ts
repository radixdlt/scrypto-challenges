import log from 'loglevel'
import chalk from 'chalk'
import prefix from 'loglevel-plugin-prefix'

enum LogLevel {
	SILENT = 'silent',
	TRACE = 'trace',
	DEBUG = 'debug',
	INFO = 'info',
	WARN = 'warn',
	ERROR = 'error',
}

const defaultLogLevel = LogLevel.WARN

const restoreDefaultLogLevel = (): void => {
	log.setLevel(defaultLogLevel)
}

restoreDefaultLogLevel()

const logDecorations = {
	trace: {
		color: chalk.italic.cyan,
		emoji: '💜',
	},
	debug: {
		color: chalk.italic.cyan,
		emoji: '💚',
	},
	info: {
		color: chalk.blue,
		emoji: '💙',
	},
	warn: {
		color: chalk.yellow,
		emoji: '💛',
	},
	error: {
		color: chalk.red,
		emoji: '❤️',
	},
}

prefix.reg(log)

prefix.apply(log, {
	format: (level, name, timestamp) =>
		`${chalk.gray(`[${timestamp.toString()}]`)} ${
			logDecorations[level.toLowerCase() as Exclude<LogLevel, 'silent'>]
				.emoji
		} ${logDecorations[
			level.toLowerCase() as Exclude<LogLevel, 'silent'>
		].color(level)}`,
})

export { log, restoreDefaultLogLevel, LogLevel }
