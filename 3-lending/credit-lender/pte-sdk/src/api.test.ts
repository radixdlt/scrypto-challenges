
import "isomorphic-fetch";
import { DefaultApi } from './api';
import { ManifestBuilder } from './manifest';

const systemComponent = '020000000000000000000000000000000000000000000000000002';
const radixToken = '030000000000000000000000000000000000000000000000000004';
const testManifest = 'CLEAR_AUTH_ZONE;';
const testNonce = 3880673815;
const testPublicKey = '043faf148413e171e2089012faacc0577d7a29aeddf25207bcf04a8768f5fe834f66585958e6ff091d1df7505c18c900e6a58b0631f98d1aa7e28630481e28e617';
const testSignature = 'aab1f69d997c5c3f27c30d9b8a97aa74c0797f41b0c5ee4e5c2ab1edfe1e9ce8114ad35f84c9bd7de1a4a0370a1bca5dea668ae554482232fd885d3f13458085';

describe('PTE API tests', function () {
    it('Test /component', async function () {
        const api = new DefaultApi();
        const component = await api.getComponent({
            address: systemComponent
        });
        console.log(component);
    })

    it('Test /resource', async function () {
        const api = new DefaultApi();
        const resource = await api.getResource({ address: radixToken });
        console.log(resource);
    })

    it('Test /nonce', async function () {
        const api = new DefaultApi();
        const nonce = await api.getNonce({ signers: [testPublicKey] });
        console.log(nonce);
    })

    it('Test /transaction', async function () {
        const api = new DefaultApi();
        const receipt = await api.submitTransaction({
            transaction: {
                manifest: testManifest,
                nonce: {
                    value: testNonce
                },
                signatures: [
                    {
                        publicKey: testPublicKey,
                        signature: testSignature
                    }
                ]
            }
        });
        console.log(receipt);

        const tx = await api.getTransaction({ hash: receipt.transactionHash });
        console.log(tx);
        const re = await api.getReceipt({ hash: receipt.transactionHash });
        console.log(receipt);
    })

    it('Test account creation', async function () {
        const manifest = new ManifestBuilder()
            .newAccount(testPublicKey)
            .build();

        const api = new DefaultApi();
        const nonce = await api.getNonce({ signers: [testPublicKey] });
        const receipt = await api.submitTransaction({
            transaction: {
                manifest: manifest.toString(),
                nonce,
                signatures: [
                ]
            }
        });
        console.log(receipt);
    })


})