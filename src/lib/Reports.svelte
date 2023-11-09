<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import StatTreeMap from './StatTreeMap.svelte';
  import StatLineGraph from './StatLineGraph.svelte';

  let date = new Date().toISOString().slice(0, 10);
  let reportType: string | undefined = undefined;

  let reportOptions: string[] = [];

  type Report = {
    total: number;
    uncategorized: number;
    category_expenses: { [key: string]: number };
    category_income: { [key: string]: number };
    dates: { [key: string]: number };
  };

  let data: Report | undefined = undefined;

  const loadReport = () => {
    invoke('get_basic_report', {
      reportType: reportType,
      selectedDate: date
    }).then((res) => {
      if (res != null) {
        console.log(res);
        data = res;
      }
    });
  };

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
    <select bind:value={reportType}>
      {#each reportOptions as option}
        <option value={option}>{option.toLowerCase()}</option>
      {/each}
    </select>
    <button disabled={reportType == undefined} on:click={loadReport}
      >Load</button
    >
  </div>

  {#if data != undefined}
    <div class="report__data">
      <StatLineGraph
        label="By {reportType.toLowerCase()}"
        data={Object.entries(data.dates).map(([key, value]) => {
          return { key: key, value: value };
        })}
        fill="#ea580c"
      />
      <div class="report__categories">
        <StatTreeMap
          label="Expenses"
          data={data.category_expenses}
          fill="#ea580c"
        />
        <StatTreeMap
          label="Income"
          data={data.category_income}
          fill="#a3e635"
        />
      </div>
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
    padding: 1rem;
    padding-bottom: 80px;
  }
  .report__controls {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .report__controls > select,
  input {
    margin: 0 6px;
    width: 160px;
  }

  .report__data {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    margin: auto;
    align-items: center;
  }

  .report__categories {
    margin-left: 2rem;
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    justify-content: center;
  }

  option {
    // uppercase the first letter
    text-transform: capitalize;
  }
</style>
