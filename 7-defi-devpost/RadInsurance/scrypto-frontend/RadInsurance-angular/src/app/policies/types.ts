import {
  ComponentAddressString,
  ResourceAddressString,
} from '@radixdlt/radix-dapp-toolkit';

export type CreatePolicy = {
  radInsuranceComponentAddress: ComponentAddressString;
  radixResourcesAddress: ResourceAddressString;
  adminAddress: ComponentAddressString;
  adminBadgeResourceAddress: ResourceAddressString;
};

export type CreatePolicies = CreatePolicy[];
