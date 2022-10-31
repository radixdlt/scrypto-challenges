<script>
  import { useNavigate } from "svelte-navigator";
  
  let navigator = useNavigate();
  export let text = "";
  export let dest = "";
  
  function goTo(){ 
    navigator(dest);
  }
</script>

<button type="button" class="btn btn-primary my-1 mx-1" on:click={goTo}>{text}</button>