<script lang="ts">
 import { PfpSource } from './participant.ts';

 export let selectedPfpSeries: PfpSource = undefined;
 export let selectedPfpId: string = undefined;

</script>

<select bind:value="{selectedPfpSeries}">
    <option value={undefined}>Choose a Pfp</option>
    <option value={PfpSource.Scrops}>Scrop</option>
    <option value={PfpSource.Radderflies}>Radderfly</option>
    <option value={PfpSource.CerberRads}>CerberRAD</option>
    <option value={PfpSource.DapperDachshunds}>Dapper Dachshund</option>
</select>
#
<input style:width="4em"
       type="text"
       disabled="{selectedPfpSeries===undefined}"
       placeholder="0"
       style:text-align="right"
       bind:value="{selectedPfpId}">
