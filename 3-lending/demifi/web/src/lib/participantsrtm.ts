import { gatewayNodeUrl,
	 participantsComponentAddress,
	 participantsNftResourceAddress } from './serviceconfig.ts';
import { walletAddress, allParticipants, promiseParticipants, userNfid } from './login.ts';
import { editedPfpSeries, editedPfpId, editedUrl,
	 endorseStore, unendorseStore,
	 sponsorStore, unsponsorStore,
	 expectSponsorStore, 
	 isPfpDirty } from './participantstores.ts'
import { ManifestBuilder } from 'pte-sdk';
import { signTransaction } from 'pte-browser-extension-sdk';
import { get } from 'svelte/store';
import { createIdrefForPfp, loadParticipant } from './participant.ts';
import { promiseWithState } from './util.ts';


export async function commitParticipants(callback) {
  let proofCount: number = 0;
  let txs: Array<[string, ManifestBuilder]> = new Array();
  let user: Participant = get(allParticipants).get(get(userNfid));
  
  if (get(isPfpDirty)) {
    let newIdRef: string = createIdrefForPfp(get(editedPfpSeries), get(editedPfpId));
    if (newIdRef !== user.idRef) {
      txs.push( [user.nfid, buildChangeIdRefManifest(user.nfid, newIdRef, proofCount)] );
      proofCount += 1;
    }
  }
  editedPfpSeries.set(undefined);
  editedPfpId.set(undefined);

  if (get(editedUrl)) {
    const newUrl: string = get(editedUrl);
    if (newUrl !== user.url) {
      txs.push( [user.nfid, buildChangeUrlManifest(user.nfid, newUrl, proofCount)] );
      proofCount += 1;
    }
  }
  editedUrl.set(undefined);

  for (let nfid: string of get(unendorseStore)) {
    txs.push( [user.nfid, buildUnendorseManifest(user.nfid, nfid, proofCount)] );
    proofCount += 1;
  }
  get(unendorseStore).clear();

  for (let nfid: string of get(endorseStore)) {
    txs.push( [user.nfid, buildEndorseManifest(user.nfid, nfid, proofCount)] );
    proofCount += 1;
  }
  get(endorseStore).clear();

  for (let nfid: string of get(unsponsorStore)) {
    txs.push( [nfid, buildUnsponsorManifest(user.nfid, nfid, proofCount)] );
    proofCount += 1;
  }
  get(unsponsorStore).clear();

  for (let nfid: string of get(sponsorStore)) {
    txs.push( [nfid, buildSponsorManifest(user.nfid, nfid, proofCount)] );
    proofCount += 1;
  }
  get(sponsorStore).clear();

//  for (let nfid: string of get(unexpectSponsorStore)) {
//    txs.push( [nfid, buildUnexpectSponsorManifest(user.nfid, nfid, proofCount)] );
//    proofCount += 1;
//  }
//  get(unexpectSponsorStore).clear();

  if (get(expectSponsorStore)) {
    txs.push( [user.nfid, buildExpectSponsorManifest(user.nfid, get(expectSponsorStore), proofCount)] );
    proofCount += 1;
  }
  expectSponsorStore.set(undefined);

  callback();

  if (txs.length === 0) return;
  
  let superManifest: string = '';
  let updateNfids: Set<string> = new Set();

  for (var [nfid, manifest] of txs) {
    superManifest = superManifest + manifest.build().toString() + '\n';
    updateNfids.add(nfid);
  }

  const promise = signTransaction(superManifest)
    .then(async ok => {
      for (let nfid: string of updateNfids) {
	const participant = await loadParticipant(nfid);
	get(allParticipants).set(nfid, participant);
      }
      callback();
    },
	  err => {console.log("ERR: " + err.status)});
}

function buildChangeIdRefManifest(nfid: string, idRef: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ nfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ nfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'change_id_ref',
		[ `Proof("proof${proofId}")`,
		  `"${idRef}"` ]
	       );
}

function buildChangeUrlManifest(nfid: string, url: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ nfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ nfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'change_url',
		[ `Proof("proof${proofId}")`,
		  `"${url}"` ]
	       );
}

function buildUnendorseManifest(subjectNfid: string, objectNfid: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ subjectNfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ subjectNfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'unendorse',
		[ `Proof("proof${proofId}")`,
		  `NonFungibleId("${objectNfid}")` ]
	       );
}

function buildEndorseManifest(subjectNfid: string, objectNfid: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ subjectNfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ subjectNfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'endorse',
		[ `Proof("proof${proofId}")`,
		  `NonFungibleId("${objectNfid}")` ]
	       );
}

function buildUnsponsorManifest(subjectNfid: string, objectNfid: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ subjectNfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ subjectNfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'unsponsor',
		[ `Proof("proof${proofId}")`,
		  `NonFungibleId("${objectNfid}")` ]
	       );
}

function buildSponsorManifest(subjectNfid: string, objectNfid: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ subjectNfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ subjectNfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'sponsor',
		[ `Proof("proof${proofId}")`,
		  `NonFungibleId("${objectNfid}")` ]
	       );
}

//function buildUnExpectSponsorManifest(subjectNfid: string, objectNfid: string, proofId: number) {
//  return new ManifestBuilder()
//    .createProofFromAccountByIds(
//      get(walletAddress),
//	  [ subjectNfid  ],
//      get(participantsNftResourceAddress))
//    .createProofFromAuthZoneByIds(
//	  [ subjectNfid  ],
//      get(participantsNftResourceAddress),
//      `proof${proofId}`)
//    .callMethod(get(participantsComponentAddress), 'unexpect_sponsor',
//		[ `Proof("proof${proofId}")`,
//		  `NonFungibleId("${objectNfid}")` ]
//	       );
//}

function buildExpectSponsorManifest(subjectNfid: string, objectNfid: string, proofId: number) {
  return new ManifestBuilder()
    .createProofFromAccountByIds(
      get(walletAddress),
	  [ subjectNfid  ],
      get(participantsNftResourceAddress))
    .createProofFromAuthZoneByIds(
	  [ subjectNfid  ],
      get(participantsNftResourceAddress),
      `proof${proofId}`)
    .callMethod(get(participantsComponentAddress), 'expect_sponsor',
		[ `Proof("proof${proofId}")`,
		  `NonFungibleId("${objectNfid}")` ]
	       );
}
