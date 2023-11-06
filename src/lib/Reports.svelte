<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let date = new Date().toISOString().slice(0, 10);

  let reportOptions: string[] = [];

  type Report = {
    total: number;

  };

  let data: Report | undefined = undefined;

  onMount(() => {
    invoke('get_report_types').then((res) => {
      if (Array.isArray(res)) {
        reportOptions = res;
      }
    });
  });
</script>

<section>
  <div class="report__controls">
    <input type="date" placeholder="Select Date" bind:value={date} />
    <select>
      {#each reportOptions as option}
        <option value={option}>{option.toLowerCase()}</option>
      {/each}
    </select>
    <button>Load</button>
  </div>

  {#if data != undefined}
    <div class="report__data">
      <h2>Total: {data.total}</h2>
    </div>
  {:else}
    <div class="report__data">
      <span>No report loaded</span>
    </div>
  {/if}

</section>

<style lang="scss">
  section {
    margin: 1rem 0;
  }
  .report__controls {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .report__controls > select, input {
    margin: 0 6px;
    width: 160px;
  }

  .report__data {
    display: flex;
    flex-direction: column;
    margin: auto;  
    align-items: center;
  }

  option {
    // uppercase the first letter
    text-transform: capitalize;
  }
</style>
