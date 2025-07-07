import { Buffer } from "buffer";
import { Client as ContractClient, Spec as ContractSpec, } from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk';
export * as contract from '@stellar/stellar-sdk/contract';
export * as rpc from '@stellar/stellar-sdk/rpc';
if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}
export const networks = {
    testnet: {
        networkPassphrase: "Test SDF Network ; September 2015",
        contractId: "CDTB5Y33QE76MLB5Q5FF3EYDMZKK2OXOTBKXJOTSH2YEABPZP3NNPB5D",
    }
};
export class Client extends ContractClient {
    options;
    static async deploy(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options) {
        return ContractClient.deploy(null, options);
    }
    constructor(options) {
        super(new ContractSpec(["AAAAAAAAAAAAAAAFaGVsbG8AAAAAAAABAAAAAAAAAAJ0bwAAAAAAEAAAAAEAAAPqAAAAEA=="]), options);
        this.options = options;
    }
    fromJSON = {
        hello: (this.txFromJSON)
    };
}
