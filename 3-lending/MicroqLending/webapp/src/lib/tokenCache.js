import {config} from './config.js';

let cachePromise = {};
let cache = {};

async function getInfoFromLedger(tokenAddress){
  const res = await fetch(config.ledger + '/resource/' + tokenAddress);
  const json = await res.json();
  let info = {};
  json.metadata.forEach(v => info[v.name] = v.value);
  return info;
}

async function getMetaInfo(tokenAddress){
  let promise = cachePromise[tokenAddress];
  if(promise)
    return await promise;
  let value = cache[tokenAddress];
  if(value)
    return value;
  promise = cachePromise[tokenAddress] = getInfoFromLedger(tokenAddress);
  cache[tokenAddress] = value = await promise;
  delete cachePromise[tokenAddress];
  return value;
}

export let tokenCache = {getMetaInfo};