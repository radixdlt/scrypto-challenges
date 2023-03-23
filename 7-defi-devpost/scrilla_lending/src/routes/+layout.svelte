<script lang="ts">
    import "../app.css";
    import LinkButton from "./LinkButton.svelte";
    import RadixButton from "./RadixButton.svelte";
    
    let current = 'home';

    const setCurrent = (title: string) => {
        current = title;
    }
</script>

<svelte:head>
    <title>$SCRILLA Lending - XRD
    </title>
</svelte:head>

<nav class="flex flex-col md:flex-row w-full md:items-baseline md:justify-between px-5 py-1">
    <a href="/" class="font-extrabold text-transparent text-3xl bg-clip-text bg-gradient-to-t from-cyan-300 to-violet-600 p-3 " on:click={() => setCurrent('Home')}>$Scrilla</a>
    <LinkButton dest="/collateral" title="Collateral" {current} {setCurrent}/>
    <LinkButton dest="/borrow" title="Borrow" {current}  {setCurrent} />
    <LinkButton dest="/shield" title="Shield" {current}  {setCurrent} />
    <LinkButton dest="/earn" title="Earn" {current}  {setCurrent} />
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <RadixButton />
</nav>

<slot/>
