<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let categoryId: number;
  export let label: string;

  let edit = false;
  let tempLabel = label;

  const editCancelAction = () => {
    if (edit) {
      edit = false;
      label = tempLabel;
    } else {
      tempLabel = label;
      edit = true;
    }
  };

  const applyAction = () => {
    if (edit) {
      tempLabel = label;
      edit = false;
      invoke('update_category_label', { label: label, id: categoryId }).then(
        () => {
          dispatch('updateHook');
        }
      );
    } else {
      invoke('delete_category', { id: categoryId }).then(() => {
        dispatch('deleteHook');
      });
    }
  };
</script>

<section class="category">
  <input data-testid="category-input" bind:value={label} disabled={!edit} />
  <section>
    <button data-testid="category-modify-action" on:click={editCancelAction}
      >{edit ? 'Cancel' : 'Edit'}</button
    >
    <button data-testid="category-apply-action" on:click={applyAction}
      >{edit ? 'Save' : 'Delete'}</button
    >
  </section>
</section>

<style>
  .category {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0.3rem 0;
  }
</style>
