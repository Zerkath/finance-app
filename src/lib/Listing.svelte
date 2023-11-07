<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import ListNavigation from './ListNavigation.svelte';
  import type { Category } from './types';

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
    updatePage();
  });

  let currentPage = 1;
  let totalPages = 1;
  let pageSize = 15;

  let categories: Category[] = [];

  const updateCategoriesList = () => {
    invoke('get_categories').then((res) => {
      if (Array.isArray(res)) {
        categories = res;
      }
    });
  };

  const changePage = (e) => {
    const page = e.detail;
    if (page < 1 || page > totalPages) return;
    currentPage = page;
    updatePage();
  };

  const updatePage = () => {
    invoke('get_transactions', {
      pageSize: pageSize,
      currentPage: currentPage,
      search: search,
      selectedCategories: selectedCategories,
      startDate: startDate,
      endDate: endDate
    }).then((res) => {
      if (res == null) return;
      const { total_pages, transactions } = res;
      totalPages = total_pages;
      data = transactions;
    });
  };

  const remove = (id: number) => {
    invoke('delete_transaction', { id: id }).then(() => {
      updatePage();
    });
  };

  let data: Row[] = [];

  let search: string = "";
  let startDate: string;
  let endDate: string | undefined;
  let selectedCategories: number[] = [];

  type Row = {
    id: number;
    name: string;
    description: string;
    value: number;
    date_created: string;
    categories: Category[];
  };
</script>

<div class="list__container">
  <div class="list__bar">
    <div class="list__form">
      <input type="text" placeholder="Search" bind:value={search} />
      <!-- TODO should be a multi-select dropdown -->
      <select
        multiple
        size="1"
        bind:value={selectedCategories}
        style="min-width: 100px"
        id="list_category_select"
      >
        {#each categories as category}
          <option value={category.id}>{category.label}</option>
        {/each}
      </select>
      <!-- TODO should be a date picker, hidden for now -->
      <input hidden bind:value={startDate} type="date" id="list_start_date" />
      <input hidden bind:value={endDate} type="date" id="list_end_date" />

      <button on:click={updatePage}>Search</button>
    </div>
    <ListNavigation {currentPage} {totalPages} on:changePage={changePage} />
  </div>

  <div class="list__and__nav">
    <table>
      <tr>
        {#each columns as column}
          <th>{column}</th>
        {/each}
      </tr>

      {#each data as row}
        <tr>
          <td>{row.name}</td>
          <td>{row.description ? row.description : ''}</td>
          <td>{row.value}</td>
          <td style="overflow: hidden">{row.date_created}</td>
          <td>
            {#each row.categories as category}
              <span
                style="padding: 4px; margin: 5px; background: lightgrey; border-radius: 5px;"
                id={category.id.toString()}>{category.label}</span
              >
            {/each}
          </td>
          <td>
            <button on:click={() => remove(row.id)}>Remove</button>
            <button disabled>Modify</button>
          </td>
        </tr>
      {/each}
    </table>
    <ListNavigation {currentPage} {totalPages} on:changePage={changePage} />
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

  .list__and__nav {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    align-items: center;
    justify-content: space-between;
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
