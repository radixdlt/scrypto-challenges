import {tokenCache} from './tokenCache.js';

export const radixResourceAddress = "030000000000000000000000000000000000000000000000000004";

let address2name = {};
let name2address = {};

[[radixResourceAddress, "RADIX"]].forEach(v => {
  address2name[v[0]] = v[1];
  name2address[v[1]] = v[0];
});

export function mapAddress2Name(addr){
  return address2name[addr] || addr;
}

export function mapName2Address(nameOrAddress){
  return name2address[nameOrAddress] || nameOrAddress;
}

export function displayToken(address, metadata){
  let name = address2name[address];
  if(name)
    return name;
  if(metadata.name && metadata.symbol)
    return "'" + metadata.name + "' (" + metadata.symbol + ") addr: " + address;
  if(metadata.name)
    return "'" + metadata.name + "' addr: " + address;
  if(metadata.symbol)
    return metadata.symbol + " addr: " + address;
  return address;
}

export function pad2(str){
  return ("00" + str).slice(-2);
}

export function assert(check, msg){
  if(!check)
    throw new Error(msg);
}

export function reduceAddress(addr){
  return addr.slice(0,5)+"..."+addr.slice(-5);
}

export function getDecimal(struct){
  assert(struct.type == "Decimal", "Expect a Decimal type, got a '"+struct.type+"'");
  let expectStart = "Decimal(\"";
  assert(struct.value.startsWith(expectStart), "the decimal value should start with: " + expectStart);
  return struct.value.slice(expectStart.length, -2);
}

export function getResourceAddress(struct){
  assert(struct.type == "ResourceAddress", "Expect a ResourceAddress type, got a '"+struct.type+"'");
  let expectStart = 'ResourceAddress("';
  assert(struct.value.startsWith(expectStart), "the resource address value should start with: " + expectStart);
  return struct.value.slice(expectStart.length, -2);
}

export function getU64(struct){
  assert(struct.type == "U64", "Expect a U64 type, got a '"+struct.type+"'");
  return struct.value;
}

export function getComponentAddress(struct){
  assert(struct.type == "ComponentAddress", "Expect a ComponentAddress type, got a '"+struct.type+"'");
  let expectStart = 'ComponentAddress("';
  assert(struct.value.startsWith(expectStart), "the component address value should start with: " + expectStart);
  return struct.value.slice(expectStart.length, -2);
}

export function readOfferFromState(stateStr){
  const state = JSON.parse(stateStr);
  let values = state.fields;
  console.log("The state is:", values);
  return {
    tokenAddress: getResourceAddress(values[0]), 
    tokenAmount: getDecimal(values[1]),
    collatAmount: getDecimal(values[2]),
    costPerHour: getDecimal(values[3]),
    maxBorrowTime: getU64(values[4]),
    unixBorrowStart: getU64(values[5]),
    collatResourceAddress: getResourceAddress(values[7]), 
    feeResourceAddress:  getResourceAddress(values[8]),
    state: values[6].name
  };
}

export async function enrichTokens(offer){
  let tokenPromise = tokenCache.getMetaInfo(offer.tokenAddress);
  let collatResourcePromise = tokenCache.getMetaInfo(offer.collatResourceAddress);
  let feeResourcePromise = tokenCache.getMetaInfo(offer.feeResourceAddress);
  
  offer.tokenMetaInfo = await tokenPromise;
  offer.collatResourceMetaInfo = await collatResourcePromise;
  offer.feeResourceMetaInfo = await feeResourcePromise;
}

export async function enrichOffers(offers){
  await Promise.all(offers.map(enrichTokens));
}

export function timeToHuman(seconds){
  if(seconds == 0)
    return "expired";
  let days = Math.floor(seconds / (60*60*24));
  seconds -= days*60*60*24;
  let hours = Math.floor(seconds / (60*60));
  seconds -= hours*60*60;
  let minutes = Math.floor(seconds / 60);
  seconds -= minutes*60;
  let timeleftHuman = pad2(seconds) + "s";
  if(!days && !hours && !minutes)
    return timeleftHuman;
  timeleftHuman = pad2(minutes) + "m" + timeleftHuman;
  if(!days && !hours)
    return timeleftHuman;
  timeleftHuman = pad2(hours) + "h" + timeleftHuman;
  if(!days)
    return timeleftHuman;
  timeleftHuman = pad2(days) + "d" + timeleftHuman;
  return timeleftHuman;
}

function searchToken(address, metainfo, search){
  if(address == search)
    return true;
  if(metainfo && metainfo.name == search)
    return true;
  if(metainfo && metainfo.symbol == search)
    return true;
}

function searchOffer(offer, search){
  if(offer.address == search)
    return true;
  if(searchToken(offer.tokenAddress, offer.tokenMetaInfo))
    return true;
  if(searchToken(offer.collatResourceAddress, offer.collatResourceMetaInfo))
    return true;
  if(searchToken(offer.feeResourceAddress, offer.feeResourceMetaInfo))
    return true;
}

export function searchOffers(offers, search){
  if(!search)
    return offers;
  return offers.filter(o => searchOffer(o, search));
}

export function checkReceipt(receipt){
  console.log("Receipt", receipt);
  if(receipt.status == "InvokeError")
    throw new Error(receipt.logs[0]);
}
