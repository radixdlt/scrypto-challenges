<script lang="ts">
  import Icon from 'svelte-icon'
  import imgPrevious from '../img/zondicons/cheveron-left.svg?raw';
  import imgNext from '../img/zondicons/cheveron-right.svg?raw';
  import { viewportHeight, userManualKeyListener } from './appstate.ts';

  let pageNdx: number = 0;
  $: page = pageNdx + 1;
  let pages: number = 8;

  const previousPage = () => {
    if (pageNdx === 0) pageNdx = pages-1;
    else pageNdx -= 1;
  };
  const nextPage = () => {
    pageNdx = (pageNdx + 1) % pages;
  };

  // Ideally this should be tied to a "modal closing" event -
  // but if it works it works I guess
  if ($userManualKeyListener !== undefined) {
    document.removeEventListener("keydown", $userManualKeyListener);
  }
  
  $userManualKeyListener = function(event) {
    if(event.key === "ArrowLeft") {
      previousPage();
    }
    else if(event.key == "ArrowRight") {
      nextPage();
    }
  };
  
  document.addEventListener('keydown', $userManualKeyListener);
</script>

<div style:display="grid"
     style:justify-item="stretch"
     style:grid-template="auto auto"
     >
  <div style:grid-column="1 / span 2"
       style:grid-row="1"
       style:justify-self="center"
       >
    <h1>Manual</h1>
  </div>
  <div style:grid-column="1"
       style:grid-row="1"
       style:justify-self="start"
       >
    <em>Page {page}/{pages}</em>
  </div>
  <div style:grid-column="2"
       style:grid-row="1"
       style:justify-self="end"
       style:align-self="center"
       class="arrows unselectable">
      <span on:click={previousPage}><Icon data={imgPrevious} size="36px" /></span>
      <span on:click={nextPage}><Icon data={imgNext} size="36px"/></span >
  </div>
</div>

<div style:min-height="{$viewportHeight * 0.6}px"
     style:max-height="{$viewportHeight * 0.6}px"
     style:overflow="auto"
     >
  
{#if page === 1}
<h2>So what am I looking at here?</h2>

<p style:text-align="justify">
  This is the identity portion of the DeMiFi lending app. It manages
  identities, and relationships between the people holding those
  identities. The UI for the actual lending and borrowing isn't yet
  complete but you can play around with identities already.
</p>

<p style:text-align="justify">
  We will first do a quick summary of what you're looking at, then on
  the following pages we will go through each element in more detail.
</p>

<p style:text-align="justify">
  On your screen what you see in the top left corner is your own
  identity: its profile picture, its name and its id. You can click on
  the profile picture to open up its profile, and you can click on the
  numeric id to copy it to clipboard (with full precision: the UI just
  shows an abbreviated form of it). All the other identities you can
  see behave similarly.
</p>

<p style:text-align="justify">
  Then you see four lists of people: Those people you have decided to
  endorse (they're your pals), those people you are currently
  sponsoring (which is very serious business), the people who have
  decided to endorse you, and finally just a list of everyone in the
  system.
</p>
{:else if page === 2}
  <h2>Your Own Profile</h2>

<p style:text-align="justify">
  When you click on your own profile picture you will be shown your
  profile settings and also a summary of your relationships:
</p>

<img src="/twoflowerprofile.png" alt="Twoflower's profile settings"/>

<p style:text-align="justify">
  Click on the pen to change your URL, or click on the copy icon to
  copy it to clipboard. Pick an NFT to use as profile picture if you
  own a supported one. Click on the bookmark icons to filter the
  "Everyone" view to show only those involved in that particular
  relationship.
</p>
{:else if page === 3}
  <h2>Other People's Profile</h2>

<p style:text-align="justify">
  When you click on someone else's profile picture out in the main
  view their profile will be brought up. This shows slightly different
  information than your own did, focusing on your relationship with
  that person:
</p>

<img src="/xavierprofile.png" alt="Xavier's profile"/>

<p style:text-align="justify">
  The key difference here compared to your own profile is that it
  shows one line of description for each type of relationship you have
  or can have with the person. To the left of each description is an
  icon you can click to change your relationship, essentially either
  turning it on or off.
</p>

<p style:text-align="justify">
  In the example shown you could choose to respond to Xavier's
  sponsorship request by accepting it, and you could choose to start
  endorsing him.
</p>

{:else if page === 4}
  <h2>Ending Relations</h2>
<p style:text-align="justify">
  There is a shortcut out in the lists view for ending relationships:
  In the "People I Endorse" list you can click the yellow star to turn
  it off, and in the "People I Sponsor" list you can click the purple
  ribbon to turn it off.
</p>

<img src="/listshortcut.png" alt="Shows the list-based shortcuts"/>

<p style:text-align="justify">
  Here we are ending our endorsement of Bob, and we are terminating
  our sponsorship of Victoria.
</p>

{:else if page === 5}
  <h2>Committing Your Changes</h2>
<p style:text-align="justify">
  When you've made such changes as you want to, you need to commit
  them to the ledger or they will be lost.
</p>  

<img src="/commitbutton.png"
     alt="Shows the reset icon and the commit button"/>

<p style:text-align="justify">
  The commit button and the arrow icon will appear whenever you have
  uncommitted changes. Click the button to save your work. This will
  take you into the PTE extension "wallet" where you will need to Sign
  and Submit the transaction.
</p>

<p style:text-align="justify">
  <b>Note</b> that at this point the UI
  thinks the data is safe so if you now decide to not submit it you
  will need to redo your work. (You will see that in the UI it has
  already been reverted, but after you Submit the transaction the UI
  will catch back up again very shortly.)
</p>

  <h2>Resetting Your Changes</h2>
<p style:text-align="justify">
  If you click the circling arrow you will reset your changes without
  committing. Do this if you just want to start from scratch again.
</p>

{:else if page === 6}
  <h2>About Micro-finance</h2>

<p style:text-align="justify">
  Micro-finance is a method of lending money to people who have next
  to nothing but they do have a business idea worth pursuing. They
  cannot offer collateral, and if their business venture fails they
  will not be able to make further payments. This is therefore a high
  risk venture for the lender. (This author likes to think of it as
  very small-scale venture capital).
</p>

<p style:text-align="justify">
  The identity system you can try out here is one of the pillars in
  reducing this risk. It is a decentralized reputation based identity
  system in which borrowers can gradually build up relationships and
  lenders can investigate those relationships to gauge how trustworthy
  any given borrower might be.
</p>

<p style:text-align="justify">
  Note that while the identity system is used in conjuction with
  micro-finance in this instance, it is a general purpose system that
  can be used for any type of application. There could easily be a
  hundred different dApps from different teams using the same identity
  system.
</p>

{:else if page === 7}
  <h2>Relationship Types</h2>

<p style:text-align="justify">
  There are two main types of relationship in the system: endorsements
  and sponsorships.
</p>

  <h3>Endorsements</h3>
<p style:text-align="justify">
  An endorsement is another person saying they figure you're a
  dependable person. They may have dealt with you before with good
  results, or they may know you personally, or they otherwise vouch
  for your character. When investigating someone you do not know you
  may gauge them by how much you trust the judgement of those who have
  endorsed them.
</p>
<p style:text-align="justify">
  A person can have any number of other people endorsing them.
</p>

<h3>Sponsorships</h3>
<p style:text-align="justify">
  A sponsorship is someone saying that they have investigated this
  person thoroughly and decided that they are a real person, not a
  fake or an alt. This can typically be someone of some authority who
  has no personal opinion of the person in question but who is
  nevertheless aware of their real-world identity (perhaps via KYC) or
  have otherwise discovered that they're legit.
</p>
<p style:text-align="justify">
  A person can have at most one person sponsoring them.
</p>

{:else if page === 8}
  <h2>Acknowledgements</h2>
<p>
  This app was built using <a href="https://svelte.dev/">Svelte</a> and typescript.
</p>

<p>
  It uses the <a href="https://www.zondicons.com/">Zondicons</a> icon set.
</p>

  <h3>Supported NFT Collections</h3>
<p>
  <a href="https://radstrike.com/scorpions">Abandoned Scorpions</a><br>
  <a href="https://cerberrads.com">CerberRADs</a><br>
  <a href="https://www.radderflies.com">Radderflies</a><br>
  <a href="https://dapper-dachshunds.com">Dapper Dachshunds</a>
</p>

<p>
  They were selected for the very small file size of their images,
  making it possible to load large numbers of them without bogging
  down the browser.
</p>

{/if}
</div>


<style>
  .arrows {
  margin:0 0 0 1em;
  size: 36px;
  cursor: pointer;
  }
  .unselectable {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
</style>
