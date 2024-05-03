// eslint-disable-next-line
export const pipe = (...fns: Function[]) => (x: any) =>
	// eslint-disable-next-line
	fns.reduce((y, f) => f(y), x)
