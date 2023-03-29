<script>
    /**
     * @type {string}
     */
     export let dest;
    
    /**
     * @type {string}
     */
     export let title;

    /**
     * @type {string}
     */
     export let current;

    /**
     * @type {(arg0: string) => void}
     */
     export let setCurrent;
</script>

<a href={dest}  class=" w-full text-center px-5 py-1 rounded-xl  transition-colors ease-in-out bg-gradient-to-b {current !== title ? "from-transparent" : "from-cyan-800"} to-slate-700 hover:bg-cyan-700 transition-colors ease-in-out duration-300" on:click={() => {
    setCurrent(title);
}}>
    {title}
</a>