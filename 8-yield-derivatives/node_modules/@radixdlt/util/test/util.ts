export const mockErrorMsg = (msg: string): string => {
	const testFilePath = expect.getState().testPath
	const testFileName = testFilePath.split('/').reverse()[0]
	const testName = expect.getState().currentTestName.replaceAll(' ', '_')
	return `MOCKED_ERROR_${msg}_${testName}_in_${testFileName}`
}
