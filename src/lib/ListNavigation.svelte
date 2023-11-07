<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let currentPage = 1;
  export let totalPages = 1;
  const changePage = (page: number) => {
    if (page < 1 || page > totalPages) return;
    dispatch('changePage', page);
  };
</script>

<div>
  <button
    on:click={() => changePage(currentPage - 1)}
    disabled={currentPage === 1}
  >
    {'<<'}
  </button>
  <span>{`[${currentPage} of ${totalPages}]`}</span>
  <button
    on:click={() => changePage(currentPage + 1)}
    disabled={currentPage === totalPages}
  >
    {'>>'}
  </button>
</div>
