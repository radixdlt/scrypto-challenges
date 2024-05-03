/// <reference types="jest" />
declare const Fetch: jest.Mock<Promise<any>, [string, any]>;
export default Fetch;
