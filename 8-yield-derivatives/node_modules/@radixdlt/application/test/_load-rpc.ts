// A little hacky solution so that we can load the RPC spec asynchronously
// and generate tests from it, since 'describe()' callback
// isn't allowed to be async in Jest. 💩💩💩
// https://github.com/facebook/jest/issues/2235

const NodeEnvironment = require('jest-environment-node')
const RPC_SPEC = require('@radixdlt/open-rpc-spec')
const parseOpenRPCDocument = require('@open-rpc/schema-utils-js')
	.parseOpenRPCDocument

class TestEnvironment extends NodeEnvironment {
	constructor(config) {
		super(config)
	}

	async setup() {
		await super.setup()
		this.global.rpcSpec = await parseOpenRPCDocument(
			JSON.stringify(RPC_SPEC),
		)
	}

	async teardown() {
		this.global.rpcSpec = null
		await super.teardown()
	}

	runScript(script) {
		return super.runScript(script)
	}
}

module.exports = TestEnvironment
