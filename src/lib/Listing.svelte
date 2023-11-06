<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  const columns = [
    'Name',
    'Description',
    'Value',
    'Date',
    'Categories',
    'Actions'
  ];

  onMount(() => {
    updateCategoriesList();
    getTransactions();
  });

  let currentPage = 1;
  let pageSize = 25;

  let categories: string[] = [];
  const updateCategoriesList = () => {
    invoke('get_categories', {
      pageSize: pageSize,
      currentPage: currentPage
    }).then((res) => {
      if (Array.isArray(res)) {
        categories = res;
      }
    });
  };

  const getTransactions = () => {
    invoke('get_transactions').then((res) => {
      if (Array.isArray(res)) {
        data = res;
      }
    });
  };

  let data: Row[] = [];

  let startDate: string;
  let endDate: string | undefined;
  let selectedCategories: string[] = [];

  const search = () => {
    updateCategoriesList();
  };

  type Row = {
    id: number;
    name: string;
    description: string;
    value: number;
    date: Date;
    categories: string[];
  };
</script>

<div class="list__container">
  <div class="list__bar">
    <div class="list__form">
      <input type="text" placeholder="Search" />
      <label for="list_category_select">Category</label>
      <!-- TODO should be a multi-select dropdown -->
      <select
        multiple
        size="1"
        bind:value={selectedCategories}
        style="min-width: 100px"
        id="list_category_select"
      >
        {#each categories as category}
          <option>{category}</option>
        {/each}
      </select>
      <label for="list_start_date">Start Date</label>
      <input bind:value={startDate} type="date" id="list_start_date" />
      <label for="list_end_date">End Date</label>
      <input bind:value={endDate} type="date" id="list_end_date" />
      <button on:click={search}>Search</button>
    </div>
    <div>
      <span>(Previous Page)</span>
      <span>[1 of 10]</span>
      <span>(Next Page)</span>
    </div>
  </div>

  <table>
    <tr>
      {#each columns as column}
        <th>{column}</th>
      {/each}
    </tr>

    {#each data as row}
      <tr>
        <td>{row.name}</td>
        <td>{row.description}</td>
        <td>{row.value}</td>
        <td>{row.date}</td>
        <td>{row.categories}</td>
        <td style="max-width: fit-content">
          <button>Edit</button>
          <button>Delete</button>
        </td>
      </tr>
    {/each}
  </table>
  <div>
    <span>(Previous Page)</span>
    <span>[1 of 10]</span>
    <span>(Next Page)</span>
  </div>
</div>

<style lang="scss">
  table {
    border-collapse: collapse;
    width: 100%;
  }

  .list__container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    align-items: center;
  }

  .list__container > * {
    margin: 10px 0;
  }

  .list__bar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    margin: 6px 20px;
  }

  .list__form {
    margin: 10px 0;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .list__form > * {
    margin: 5px 10px 0 0;
  }

  th,
  td {
    text-align: left;
    padding: 8px;
    border: 1px solid #000;
  }
</style>
