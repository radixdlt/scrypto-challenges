<script lang="ts">
  import UserIcon from './UserIcon.svelte'
  import { ManifestBuilder } from 'pte-sdk';
  import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';
  import { gatewayNodeUrl, participantsComponentAddress, participantsNftResourceAddress } from './serviceconfig.ts';
  import { walletAddress, userNfid, tourist,
	   allParticipants, promiseParticipants } from './login.ts';
  import type { Participant } from './participant.ts';
  import { establishPfp, numberToNfid, createIdrefForPfp, loadParticipant } from './participant.ts';
  import { appStarted, pollCatalog } from './appstate.ts';
  import ChoosePfp from './ChoosePfp.svelte';
  import { promiseWithState } from './util.ts';
  
  // JSON.stringify(parsed, null, 2);
  
  enum ConnectionStatus {
    NotConnected,
    Connecting,
    Connected,
  }

  let myNfids: string[] = new Array(0);
  let newUserName: string = '';
  let selectedUser: string = undefined;
  let selectedPfpSeries: string = "";
  let selectedPfpId: string = "";
  let errorText: string = '';


  // createDemoParticipantsForAlice(); 

  let promise = promiseWithState(fetchParticipants());
  $pollCatalog = true;

  async function fetchParticipant(nfid: string): Participant {
    let participant = await loadParticipant(nfid);
    $allParticipants.set(nfid, participant);
    $allParticipants = $allParticipants;

    return participant;
  }
  
  async function fetchParticipants() {
    myNfids = new Array();
    if ($tourist) {
      myNfids.push('000000000000001f'); // This is or should be Twoflower
    } else {
      const accountResp = await fetch(`${$gatewayNodeUrl}/component/${$walletAddress}`);
      const accountParsed = await accountResp.json();
      const participants = accountParsed.owned_resources.filter(
	res => res.resource_address === $participantsNftResourceAddress);
      if (participants.length > 0) {
        myNfids = participants[0].non_fungible_ids;
      }
    }
    // This function only promises the Participants that our user owns
    let promises = myNfids.map((nfid) => {
      let singlePromise = promiseWithState(fetchParticipant(nfid));
      $promiseParticipants.set(nfid, singlePromise);
      $promiseParticipants = $promiseParticipants;
      return singlePromise;
    });

    if (myNfids.length === 1) {
      $userNfid = myNfids[0];
    }

    // But we also start loading all the other Participants
    for (let [key, value] of $allParticipants) {
      if (!$promiseParticipants.get(key)) {
	$promiseParticipants.set(key,
				 promiseWithState(fetchParticipant(key)));
	$promiseParticipants = $promiseParticipants;
      }
    }

    return promises ? Promise.all(promises) : undefined;
  }

  async function createDemoParticipantsForAlice() {
    let manifest = new ManifestBuilder();

    const nfidCreator: string = '0000000000000000';
    const nfidAlice: string = '0000000000000001';
    const nfidMimir: string = '0000000000000002';
    
    const users: Array<Participant> = [
      {userName: "Bob",                       idRef: "scrop 5235",  sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Carla",                     idRef: "",            sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Debbie",                    idRef: "radfly 68",   sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Fiona",                     idRef: "cerber 160",  sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "George",                    idRef: "cerber 276",  sponsor: "",          expectSponsor: `None` },
      {userName: "Harriet",                   idRef: "cerber 427",  sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Indy",                      idRef: "cerber 431",  sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Juliet",                    idRef: "cerber 805",  sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Karl",                      idRef: "",            sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Lily",                      idRef: "",            sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Mort",                      idRef: "cerber 1010", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Nellie",                    idRef: "cerber 1188", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Oppenheimer",               idRef: "cerber 1202", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Papandreyo",                idRef: "cerber 1978", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Queen Christine the First", idRef: "cerber 2875", sponsor: "",          expectSponsor: `None` },
      {userName: "Romeo",                     idRef: "cerber 2187", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Sierra",                    idRef: "",            sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Thor",                      idRef: "",            sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Ulysses",                   idRef: "",            sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Victoria",                  idRef: "cerber 3455", sponsor: "",          expectSponsor: `None` },
      {userName: "William",                   idRef: "cerber 4418", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Xavier",                    idRef: "cerber 5060", sponsor: "",          expectSponsor: `None` },
      {userName: "Yolonda",                   idRef: "cerber 5271", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Zed",                       idRef: "cerber 5571", sponsor: nfidCreator, expectSponsor: `Some(NonFungibleId("${nfidCreator}"))` },
      {userName: "Charlie",                   idRef: "cerber 6387", sponsor: nfidMimir,   expectSponsor: `Some(NonFungibleId("${nfidMimir}"))` },
      {userName: "Charlie",                   idRef: "cerber 7515", sponsor: nfidMimir,   expectSponsor: `Some(NonFungibleId("${nfidMimir}"))` },
      {userName: "Charlie",                   idRef: "",            sponsor: nfidMimir,   expectSponsor: `Some(NonFungibleId("${nfidMimir}"))` },
    ];

    for (let user: Participant of users) {
      manifest = manifest.callMethod(
	$participantsComponentAddress, 'new_participant',
	[
	  `"${user.userName}"`,
	  '""',
	  `"${user.idRef}"`,
	  `${user.expectSponsor}`,
	]
      );
    }


    for (let i in users) {
      let nfid: string = numberToNfid(Number(i) + 3);
      if (users[i].sponsor !== "") {
	let sponsor: string = users[i].sponsor;
	manifest = manifest.createProofFromAccountByIds(
	  $walletAddress,
	  [ sponsor  ],
	  $participantsNftResourceAddress)
	manifest = manifest.createProofFromAuthZoneByIds(
	  [ sponsor  ],
	  $participantsNftResourceAddress,
	  `proof${i}`);
	manifest = manifest.callMethod(
	  $participantsComponentAddress, 'sponsor',
	  [
	    `Proof("proof${i}")`,
	    `NonFungibleId("${nfid}")`,
	  ]
	);
      }
    }

    for (let i: number = 0; i < 10; ++i) {
      let nfid: string = numberToNfid(Number(i) + 3);
      manifest = manifest.createProofFromAccountByIds(
	$walletAddress,
	[ nfid  ],
	$participantsNftResourceAddress)
      manifest = manifest.createProofFromAuthZoneByIds(
	[ nfid  ],
	$participantsNftResourceAddress,
	`nuproof${i}`);
      manifest = manifest.callMethod(
	$participantsComponentAddress, 'endorse',
	[
	  `Proof("nuproof${i}")`,
	  `NonFungibleId("${nfidMimir}")`,
	]
      );
    }

    let finalManifest = manifest
	.callMethodWithAllResources($walletAddress, 'deposit_batch')
	.build()
	.toString();
    console.log(finalManifest);
    const receipt = await signTransaction(finalManifest);
    if (receipt.status === 'Success') {

    } else {
      errorText = 'Error: ' + receipt.status;
    }
  }
  
  async function createNewParticipant() {
    let newIdref: string = createIdrefForPfp(selectedPfpSeries, selectedPfpId);
    const manifest = new ManifestBuilder()
	  .callMethod($participantsComponentAddress, 'new_participant',
		      [
			`"${newUserName}"`,
			'""',
			`"${newIdref}"`,
			'None',
		      ]
		     )
	  .callMethodWithAllResources($walletAddress, 'deposit_batch')
	  .build()
	  .toString();
    const receipt = await signTransaction(manifest);
    if (receipt.status === 'Success') {
      promise = promiseWithState(fetchParticipants());
    } else {
      errorText = 'Error: ' + receipt.status;
    }
  }
  
  const newUser = () => {
    createNewParticipant();
  }

  const startApp = () => {
    $appStarted = true;
  }

  const startAppFromSelect = () => {
    $userNfid = selectedUser;
    $appStarted = true;
  }
  
</script>

{#await promise}
  loading ...
{:then}
  {#if myNfids.length === 0}
    <div>
      <button on:click={newUser} disabled="{newUserName.length === 0}">Create New</button>
    </div>
    <div>
      <!-- svelte-ignore a11y-autofocus -->
      <input type="text" autofocus bind:value={newUserName}>
    </div>
    <ChoosePfp bind:selectedPfpSeries={selectedPfpSeries} bind:selectedPfpId={selectedPfpId}/>
    
    <div innerHTML={errorText}> </div>
  {:else if myNfids.length === 1}
    <div
      style:display="flex"
      style:align="center"
      style:justify-content="center"
      style:align-items="center"
      >
      <UserIcon nfid="{$userNfid}"/>
      <div style:margin="0 5px 0 5px"><b>{$allParticipants.get($userNfid).userName}</b></div>
      <div><button on:click="{startApp}">Start</button></div>
    </div>
  {:else}
    <div style:display="flex"
	 style:justify-content="center"
	 style:align-items="center"
	 >
      <UserIcon nfid="{selectedUser}"/>
      <div style:margin="0 5px 0 5px"><select bind:value="{selectedUser}" >
	  {#each myNfids as nfid}
	    <option value="{nfid}" >
	      {$allParticipants.get(nfid).userName}
	    </option>
	  {/each}
	</select></div>
      <div><button on:click="{startAppFromSelect}">Start</button></div>
    </div>
  {/if}
{:catch error}
  ERROR: {error}
{/await}
