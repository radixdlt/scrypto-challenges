<!--suppress ES6UnusedImports -->
<script lang="ts">
    import accountManager from "../lib/stores/accountManager";
    import flashyfiRepo from "../lib/flashyfiRepo";
    import messageManager from "../lib/stores/messageManager";
    import Snackbar, {Label} from "@smui/snackbar";
    import {base} from "$app/paths";
    import {page} from '$app/stores';
    import type {Page} from "@sveltejs/kit";

    accountManager.setAllFlashyfiAccounts(flashyfiRepo.getAllFlashyfiedAccounts())

    let {message, messageType} = messageManager
    let snackbar: Snackbar
    $: {
        if ($message && snackbar) {
            // noinspection TypeScriptUnresolvedFunction
            snackbar.open()
        }
    }

    function isCurrentPage(pagePathname: string, currentPage: Page): boolean {
        return pagePathname === currentPage.url.pathname || pagePathname === currentPage.url.pathname + "/"
    }
</script>

<svelte:head>
    <!-- SMUI Styles -->
    <link rel="stylesheet" href="{base}/smui.css"/>
</svelte:head>

<header>
    <div class="header-content">
        <a href="{base}/" class="title">FlashyFi</a>
        <a class="nav-link" class:current={isCurrentPage(base+"/", $page)} href="{base}/"
           style="margin-left: auto;">Home
        </a>
        <a class="nav-link" class:current={isCurrentPage(base+"/accounts",$page)} href="{base}/accounts">
            Manage Accounts
        </a>
        <a class="nav-link" class:current={isCurrentPage(base+"/about",$page)} href="{base}/about">
            About
        </a>
        <radix-connect-button id="radix-connect-button"></radix-connect-button>
    </div>
</header>
<main style="background-image: url('{base}/background.jpg')">
    <slot/>
</main>
<footer>
    <div class="footer-content">
        <a href="https://www.vecteezy.com/free-vector/background-pattern">Background Pattern Vectors by Vecteezy</a>
    </div>
</footer>
<!--</div>-->


<Snackbar bind:this={snackbar}
          class={$messageType==="Success" ? "message-success" : $messageType==="Error" ? "message-error" : ""}>
    <Label><span style="font-weight: bold">{$message}</span></Label>
</Snackbar>

<style lang="scss">
  :global(.message-success > .mdc-snackbar__surface) {
    background-color: #2abfa1;
    margin-bottom: 1.5rem;
  }

  :global(.message-error > .mdc-snackbar__surface) {
    background-color: var(--mdc-theme-error);
    margin-bottom: 1.5rem;
  }

  * :global(.page-heading) {
    font-weight: bold;
    font-size: 3rem;
    margin-top: 5rem;
    margin-bottom: 3rem;
    color: #F5F6FB;
  }

  * :global(.content-container) {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  * :global(.warning) {
    margin-top: 1rem;
    color: var(--mdc-theme-on-error);
    background-color: var(--mdc-theme-error);
    border: 1px solid var(--mdc-theme-error);
    border-radius: var(--mdc-shape-medium, 4px);
    padding: 1rem;
  }

  * :global(.info) {
    margin-top: 1rem;
    color: var(--mdc-theme-on-secondary);
    background-color: var(--mdc-theme-secondary);
    border: 1px solid var(--mdc-theme-secondary);
    border-radius: var(--mdc-shape-medium, 4px);
    padding: 1rem;
  }

  header {
    position: fixed;
    top: 0;
    width: 100%;
    height: 80px;
    box-shadow: 0 10px 15px rgba(0, 0, 0, 0.2);
    background-color: white;
    z-index: 1;
  }

  .header-content {
    max-width: 70vw;
    margin: 0 auto;
    height: 100%;
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .title {
    //color: #4b5563;
    color: var(--mdc-theme-primary);
    font-weight: bolder;
    font-size: 2.5rem;
    text-shadow: 0 1px 1px #333;
    text-decoration: none;
  }

  .nav-link {
    //color: #4b5563;
    color: var(--mdc-theme-primary);
    font-size: 20px;
    font-weight: bolder;
    text-decoration: none;
  }

  .nav-link.current {
    text-decoration: underline;
  }

  footer {
    height: 25px;
    display: flex;
    align-items: center;
    //background-color: #9f3046;
  }

  .footer-content {
    max-width: 70vw;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .footer-content > a {
    text-decoration: none;
    margin: 0;
    padding: 0;
    color: var(--mdc-theme-secondary);
    font-size: 0.9rem;
  }

  main {
    //scroll-margin-top: 80px;
    padding-top: 80px;
    padding-bottom: 1rem;
    background-repeat: repeat;
    min-height: calc(100vh - 80px - 25px - 1rem); // View height - header height - footer height
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    box-shadow: 0 10px 15px rgba(0, 0, 0, 0.2);
  }
</style>