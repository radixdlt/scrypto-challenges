import { useState, useEffect } from 'react';
// Import Radix Wallet and Gateway SDKs
import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import {
  StateApi,
  TransactionApi,
  StatusApi,
} from '@radixdlt/alphanet-gateway-api-v0-sdk';

const CreateBallot = () => {
  return (
    <div className="">
      <h2>CreateBallot</h2>
      <p>Add Option Name</p>
      <input type="text" />
      <p>Add Option Description</p>
      <input type="text" />
      <button>Create</button>
    </div>
  );
};

export default CreateBallot;
