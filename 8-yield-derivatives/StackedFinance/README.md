# Table of Contents
- [Overview](#overview)
- [How To Build](#dapp)

## Overview
This is Vue 3, Nuxt 3 and Nuxt UI project for the Radix Yield Derivatives Competition

You can check out the project hosted here:  
[Project Link](https://radixyield.web.app/)

## How to Build

### Dapp
```
yarn install
yarn dev
```
1. Build the scrypto packages like normal and deploy them to stokenet.
1. Update the data/liquidity.json file with the new package ids for the yieldPackageAddress and aamPackageAddress.

#### Data
These would generally be placed on a server somehwhere, but for the competition I wanted to have a static site more or less  
index.json: just some filler data  
liquidity.json: contains the address of components and validators  
validators.json: this is just a cached copy in time of the validtors to avoid pulling from gateway  

## Decisions
Pulling data from the gateway is a temporary solution, as it would start to cause problems as the number of tokenized yieds grew.

## Disclaimer
Everything built here is for educational purposes only, it is not intended to be used in production.