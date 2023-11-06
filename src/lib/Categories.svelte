<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import CategoryComponent from './CategoryComponent.svelte';
  import type { Category } from './types';

  let categories: Category[] = [];

  const updateCategoriesList = () => {
    invoke('get_categories').then((res) => {
      if (Array.isArray(res)) {
        categories = res;
      }
    });
  };

  onMount(updateCategoriesList);

  let name: string = '';
  let value = 0;
  let isExpense = true;
  let description: string | undefined = undefined;
  let dateCreated: string = new Date().toISOString().split('T')[0];
  let categoryIds: number[] = [];

  const insertTransaction = () => {

    if (name == '' || value == 0 || dateCreated == '') {
      return; // alert that the name and value are required
    }

    // also absolute
    value = Math.abs(value);
    value = isExpense ? -value : value;

    invoke('insert_transaction', {
      value,
      name,
      description,
      dateCreated,
      categoryIds
    }).then(() => {
      value = 0;
      name = '';
      description = undefined;
      isExpense = true;
      dateCreated = new Date().toISOString().split('T')[0];
      categoryIds = [];
    });
  };

  let categoryLabel = '';
  const upsertCategory = () => {
    if (categoryLabel == '') {
      return;
    }
    invoke('insert_category', { label: categoryLabel }).then(() => {
      updateCategoriesList();
    });
  };
</script>

<div class="creation-forms">
  <div class="transaction-form">
    <h3>Transaction Creation Form</h3>
    <input type="text" placeholder="Name" bind:value={name} />
    <input type="text" placeholder="Description" bind:value={description} />
    <input type="number" placeholder="Amount" bind:value />
    <input type="date" placeholder="Date" bind:value={dateCreated} />
    <div style="display: flex; align-items: center;">
      <span style="font-size: 12px; margin-right: 5px;">Is Expense?</span>
      <input type="checkbox" bind:checked={isExpense}/>
    </div>
    <select multiple bind:value={categoryIds}>
      {#each categories as category}
        <option value={category.id}>{category.label}</option>
      {/each}
    </select>

    <input type="submit" on:click={insertTransaction} value="Add Transaction" />
  </div>

  <div class="category-form">
    <h3>Modify Categories</h3>
    <input bind:value={categoryLabel} />
    <button style="margin-bottom: 1rem;" on:click={upsertCategory}>Add</button>
    {#each categories as category}
      <CategoryComponent
        on:deleteHook={updateCategoriesList}
        on:updateHook={updateCategoriesList}
        categoryId={category.id}
        label={category.label}
      />
    {/each}
  </div>
</div>

<style lang="scss">
  .creation-forms {
    display: flex;
    flex-direction: row;
    justify-content: left;
    flex-wrap: wrap;
    > * {
      min-width: 20rem;
      margin: 1rem;
      border: 1px solid black;
      padding: 1rem;
    }
  }

  input {
    margin: 0.5rem 0;
  }

  .category-form {
    display: flex;
    flex-direction: column;
  }

  .transaction-form {
    display: flex;
    flex-direction: column;
  }
</style>
