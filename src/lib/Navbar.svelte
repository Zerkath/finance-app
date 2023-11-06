<script lang="ts">
  import PlaceholderIcon from './PlaceholderIcon.svelte';
  import NavbarOptionText from './NavbarOptionText.svelte';

  let isCollapsed = false;

  const collapse = () => {
    isCollapsed = !isCollapsed;
  };
</script>

<div class="navbar">
  <div class="navbar__option">
    <PlaceholderIcon content="1" />
    <NavbarOptionText
      header="Listing"
      desc="List expenses, do modifications and removals"
      {isCollapsed}
    />
  </div>
  <div class="navbar__option">
    <PlaceholderIcon content="2" />
    <NavbarOptionText
      header="Reports"
      desc="View visualized aggregate information about transactions"
      {isCollapsed}
    />
  </div>
  <div class="navbar__option">
    <PlaceholderIcon content="3" />
    <NavbarOptionText
      header="Categories & Transactions"
      desc="Add new transactions, modify, add or remove categories"
      {isCollapsed}
    />
  </div>
  <div class="navbar__spacer navbar__landscape" />
  <div class="navbar__option navbar__landscape" on:click={collapse}>
    <PlaceholderIcon content="4" />
    <NavbarOptionText
      header="Collapse"
      desc="Collapse the sidebar"
      {isCollapsed}
    />
  </div>
  <div class="navbar__option">
    <PlaceholderIcon content="5" />
    <NavbarOptionText
      header="Options"
      desc="Application options"
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
