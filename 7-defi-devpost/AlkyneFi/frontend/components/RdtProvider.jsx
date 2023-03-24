import { RdtContext } from './rdt-context';

export const RdtProvider = (input) => <RdtContext.Provider value={input.value}>{input.children}</RdtContext.Provider>;
