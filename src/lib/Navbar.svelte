<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  import NavbarOptionText from './NavbarOptionText.svelte';

  import IconGear from './IconGear.svelte';
  import IconList from './IconList.svelte';
  import IconPencil from './IconPencil.svelte';
  import IconGraph from './IconGraph.svelte';
  import IconCollapse from './IconCollapse.svelte';
  import IconExpand from './IconExpand.svelte';

  let isCollapsed = false;

  const collapse = () => {
    isCollapsed = !isCollapsed;
  };

  const changeOption = (option: Option) => {
    dispatch('optionChange', option);
  };
</script>

<div class="navbar">
  <div
    class="navbar__option"
    role="button"
    tabindex={1}
    on:click={() => changeOption('listing')}
    on:keypress={() => changeOption('listing')}
  >
    <IconList />
    <NavbarOptionText
      header={'Listing'}
      desc={'List expenses, do modifications and removals'}
      {isCollapsed}
    />
  </div>

  <div
    class="navbar__option"
    role="button"
    tabindex={2}
    on:click={() => changeOption('reports')}
    on:keypress={() => changeOption('reports')}
  >
    <IconGraph />
    <NavbarOptionText
      header={'Reports'}
      desc={'View visualized aggregate information about transactions'}
      {isCollapsed}
    />
  </div>

  <div
    class="navbar__option"
    role="button"
    tabindex={3}
    on:click={() => changeOption('categories')}
    on:keypress={() => changeOption('categories')}
  >
    <IconPencil />
    <NavbarOptionText
      header={'Categories & Transactions'}
      desc={'Add new transactions, modify, add or remove categories'}
      {isCollapsed}
    />
  </div>

  <div class="navbar__spacer" />

  <div
    class="navbar__option navbar__landscape"
    role="button"
    tabindex={4}
    on:click={collapse}
    on:keypress={collapse}
  >
    {#if isCollapsed}
      <IconExpand />
    {:else}
      <IconCollapse />
    {/if}
    <NavbarOptionText
      header={'Collapse'}
      desc={'Collapse the sidebar'}
      {isCollapsed}
    />
  </div>

  <div
    class="navbar__option"
    role="button"
    tabindex={5}
    on:click={() => changeOption('options')}
    on:keypress={() => changeOption('options')}
  >
    <IconGear />
    <NavbarOptionText
      header={'Options'}
      desc={'Application options'}
      {isCollapsed}
    />
  </div>
</div>

<style lang="scss">
  .navbar__spacer {
    height: 100%;
  }

  .navbar__option {
    margin: 15px 20px;
    display: flex;
    align-items: center;
    width: fit-content;
    flex-direction: row;
    cursor: pointer;
  }
</style>
