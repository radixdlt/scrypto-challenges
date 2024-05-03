const fs = require('fs')
const path = require('path')

const text = fs.readFileSync(
	path.join(__dirname, 'src', 'open-api', 'api.ts'),
	{ encoding: 'utf-8' },
)
const version = text.match(/The version of the OpenAPI document: (\d.\d.\d)/)[1]

fs.writeFileSync(
	path.join(__dirname, 'src', 'open-api', 'api-version.ts'),
	`export const apiVersion = '${version}'`,
)
