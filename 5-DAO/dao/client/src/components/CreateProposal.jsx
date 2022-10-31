import { useState, useEffect } from 'react';
// Import Radix Wallet and Gateway SDKs
import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import {
  StateApi,
  TransactionApi,
  StatusApi,
} from '@radixdlt/alphanet-gateway-api-v0-sdk';

const CreateProposal = () => {
  return (
    <div>
      <h2>Create a New Contributor Proposal</h2>
      <p>
        Create contribution proposals to allow community members to earn DAO
        member tokens by helping build your DAO
      </p>
      <p>Add Field</p>
      <input type="text" />
      <p>Proposed Token Reward for completed contribution.</p>
      <input type="text" />
      <button>Create Proposal</button>
      <p>Comments: </p>
      <input type="text" />
    </div>
  );
};

export default CreateProposal;
