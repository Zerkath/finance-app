<script lang="ts">
  import Navbar from './lib/Navbar.svelte';
  import Listing from './lib/Listing.svelte';
  import Categories from './lib/Categories.svelte';
  import Reports from './lib/Reports.svelte';
  import Options from './lib/Options.svelte';
  import type { Option } from './lib/types';

  // TODO could be stored in browser storage
  let activeOption: Option = 'listing';

  function handleChange(e) {
    activeOption = e.detail;
  }
</script>

<main class="container">
  <Navbar on:optionChange={handleChange} />
  <div class="content__container">
    {#if activeOption == 'listing'}
      <Listing />
    {:else if activeOption == 'reports'}
      <Reports />
    {:else if activeOption == 'categories'}
      <Categories />
    {:else if activeOption == 'options'}
      <Options />
    {/if}
  </div>
</main>

<style>
  .content__container {
    width: 100%;
    height: 100%;
  }
</style>
