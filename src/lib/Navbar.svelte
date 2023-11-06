<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  import PlaceholderIcon from './PlaceholderIcon.svelte';
  import NavbarOptionText from './NavbarOptionText.svelte';
  import type { Option } from './types';

  let isCollapsed = false;

  const collapse = () => {
    isCollapsed = !isCollapsed;
  };

  const changeOption = (option: Option) => {
    dispatch('optionChange', option);
  };

  type NavbarOption = {
    header: string;
    desc: string;
    className?: string;
    action: () => void;
  };

  const options: NavbarOption[] = [
    {
      header: 'Listing',
      desc: 'List expenses, do modifications and removals',
      action: () => {
        changeOption('listing');
      }
    },
    {
      header: 'Reports',
      desc: 'View visualized aggregate information about transactions',
      action: () => {
        changeOption('reports');
      }
    },
    {
      header: 'Categories & Transactions',
      desc: 'Add new transactions, modify, add or remove categories',
      action: () => {
        changeOption('categories');
      }
    },
    {
      header: 'Collapse',
      desc: 'Collapse the sidebar',
      className: 'navbar__landscape',
      action: collapse
    },
    {
      header: 'Options',
      desc: 'Application options',
      action: () => {
        changeOption('options');
      }
    }
  ];
</script>

<div class="navbar">
  {#each options as option, index}
    {#if index === 3}
      <div class="navbar__spacer" />
    {/if}
    <div class={`navbar__option ${option.className ? option.className : ""}`} on:click={option.action}>
      <PlaceholderIcon content={index} />
      <NavbarOptionText
        header={option.header}
        desc={option.desc}
        {isCollapsed}
      />
    </div>
  {/each}
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
