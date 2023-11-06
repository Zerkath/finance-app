<script lang="ts">

  import Navbar from './Navbar.svelte';
  import Listing from './Listing.svelte';
  import Categories from './Categories.svelte';
  import Reports from './Reports.svelte';
  import Options from './Options.svelte';
  import type { Option } from './types';

  let activeOption: Option = localStorage.getItem('option')
    ? (localStorage.getItem('option') as Option)
    : 'listing';

  const handleChange = (e) => {
    const option = <Option> e.detail;
    console.log(option);
    localStorage.setItem('option', option);
    activeOption = option;
  };

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
    margin: 1rem;
    width: 100%;
    height: 100%;
  }
</style>
