import {
    RadixDappToolkit,
} from '@radixdlt/radix-dapp-toolkit'
import { dAppId } from "./general/constants";

const rdt = RadixDappToolkit(
    { dAppDefinitionAddress: dAppId, dAppName: 'stoichiometric.fi' },
    () => null,
    { networkId: 11 }
);

function resetRdt() {
    localStorage.clear();
}

export { rdt, resetRdt }

