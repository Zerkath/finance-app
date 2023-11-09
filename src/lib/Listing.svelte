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

  let search: string = '';
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

  <div class="list__content">
    <div class="list__table">
      <div class="row">
        {#each columns as column}
          <div class="cell">{column}</div>
        {/each}
      </div>

      {#each data as row}
        <div class="row">
          <div class="cell">{row.name}</div>
          <div class="cell">{row.description ? row.description : ''}</div>
          <div class="cell">{row.value}</div>
          <div class="cell">{row.date_created}</div>
          <div class="cell">
            <div class="category">
              {#each row.categories as category}
                <span class="category__label" id={category.id.toString()}
                  >{category.label}</span
                >
              {/each}
            </div>
          </div>
          <div class="cell">
            <button on:click={() => remove(row.id)}>Remove</button>
            <button disabled>Modify</button>
          </div>
        </div>
      {/each}
    </div>
    <ListNavigation {currentPage} {totalPages} on:changePage={changePage} />
  </div>
</div>

<style lang="scss">
  $colors: lightblue, lightcoral, lightgreen, lightgoldenrodyellow, lightskyblue,
    lightpink, lightseagreen, lightsalmon, lightsteelblue, lightslategray;

  .list__table {
    display: table;
    flex-direction: column;
    width: 100%;
    margin-bottom: 20px;
  }

  .row {
    display: table-row;
  }

  .cell {
    display: table-cell;
    padding: 5px;
    border: 1px solid #ccc;
  }

  .category {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: center;
    font-size: 0.9rem;
    text-transform: capitalize;
  }

  @for $i from 1 through length($colors) {
    .category > span:nth-child(#{$i}) {
      background-color: nth($colors, $i);
    }
  }

  .category__label {
    margin: 0 2px;
    padding: 6px 6px;
    border-radius: 4px;
  }

  .list__content {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    align-items: center;
    justify-content: space-between;
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
</style>
