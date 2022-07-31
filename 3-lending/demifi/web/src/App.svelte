<script lang="ts">
  import StartScreen from './lib/StartScreen.svelte'
  import LendingApp from './lib/LendingApp.svelte'
  import Modal from 'svelte-simple-modal';
  import { appStarted,
	   pollCatalog, pollParticipantData } from './lib/appstate.ts';
  import { modal } from './lib/modal.ts';
  import { allParticipants, promiseParticipants,
	   loadedParticipantCount, participantNftCount } from './lib/login.ts';
  import { getHighestParticipantNfid, numberToNfid,
	   loadParticipant, equalParticipantData  } from './lib/participant.ts';
  import { promiseWithState } from './lib/util.ts';
  
  $: {
    let loaded: number = 0;
    for (let [nfid, part] of $allParticipants) {
      if (part !== undefined) loaded += 1;
    }
    $loadedParticipantCount = loaded;
  }

  const PARTICIPANT_DATA_POLL_INTERVAL: number = 10000;
  let participantDataTimeoutRef = undefined;

  async function updateParticipantData() {
    const nfid = numberToNfid($pollParticipantData);
    let promise = loadParticipant(nfid);
    let participant = await promise;
    let oldParticipant = $allParticipants.get(nfid);
    if (oldParticipant !== undefined  &&
	!equalParticipantData(oldParticipant, participant)) {
      $allParticipants.set(nfid, participant);
      $allParticipants = $allParticipants;
      $promiseParticipants.set(nfid,
			       promiseWithState(promise));
      $promiseParticipants = $promiseParticipants;
    }
    $pollParticipantData = ($pollParticipantData + 1) % ($participantNftCount + 1);
    participantDataTimeoutRef = undefined;
    if ($pollCatalog) {
      participantDataTimeoutRef = setTimeout(updateParticipantData,
					    PARTICIPANT_DATA_POLL_INTERVAL);
    }
  }

  $: {
    if ($pollCatalog) {
      if (participantDataTimeoutRef === undefined)
        participantDataTimeoutRef = setTimeout(updateParticipantData,
					      PARTICIPANT_DATA_POLL_INTERVAL);
    } else {
      clearTimeout(participantDataTimeoutRef);
      participantDataTimeoutRef = undefined;
    }
  }
  

  const PARTICIPANT_COUNT_POLL_INTERVAL: number = 60000;
  let participantCountTimeoutRef = undefined;
  
  async function updateParticipantMaxCount() {
    let result = await getHighestParticipantNfid();
    if (result > $participantNftCount) {
      let oldCount = $participantNftCount;
      $participantNftCount = result;
  
      for (let ndx = oldCount + 1; ndx <= result; ++ndx) {
	const nfid = numberToNfid(ndx);
	let promise = loadParticipant(nfid);
	let participant = await promise;
	$allParticipants.set(nfid, participant);
	$allParticipants = $allParticipants;
	$promiseParticipants.set(nfid,
				 promiseWithState(promise));
	$promiseParticipants = $promiseParticipants;
	
      }
    }
    participantCountTimeoutRef = undefined;
    if ($pollCatalog) {
      participantCountTimeoutRef = setTimeout(updateParticipantMaxCount,
					     PARTICIPANT_COUNT_POLL_INTERVAL);
    }
  }

  $: {
    if ($pollCatalog) {
      if (participantCountTimeoutRef === undefined)
        participantCountTimeoutRef = setTimeout(updateParticipantMaxCount,
					       PARTICIPANT_COUNT_POLL_INTERVAL);
    } else {
      clearTimeout(participantCountTimeoutRef);
      participantCountTimeoutRef = undefined;
    }
  }

</script>


<main>
  <Modal show={$modal} >
    {#if (!$appStarted)}
      <StartScreen />
    {:else}
      <LendingApp />
    {/if}
  </Modal>
</main>
