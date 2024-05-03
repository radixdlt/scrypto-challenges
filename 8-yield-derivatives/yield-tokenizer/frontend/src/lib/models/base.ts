import { rdt } from "$lib";
import EntityStateFetcher from "$lib/utils/state_fetcher";
import type { StateApi } from "@radixdlt/radix-dapp-toolkit";

export class BaseModel {
  protected stateApi!: StateApi;
  protected stateFetcher: EntityStateFetcher;
  constructor() {
    this.stateApi = rdt.gatewayApi.state.innerClient;
    this.stateFetcher = new EntityStateFetcher(this.stateApi, {
      useDecimals: true
    });
  }
}


