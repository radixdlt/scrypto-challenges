<script>
    /**
     * @type {any}
     */
     export let stdout;
</script>

<div class="w-full max-h-32 overflow-auto bg-black text-green-400 break-all">
    {stdout ? JSON.stringify(stdout) : ""}
</div>