<script lang="ts" context="module">
  import PlaceholderIcon from './PlaceholderIcon.svelte';
  import NavbarOptionText from './NavbarOptionText.svelte';

  let isCollapsed = false;

  const collapse = () => {
    isCollapsed = !isCollapsed;
    console.log(isCollapsed);
  };
</script>

<div class="navbar">
  <div class="navbar__option">
    <PlaceholderIcon content="1" />
    <NavbarOptionText
      header="Listing"
      desc="List expenses, do modifications and removals"
      isCollapsed={isCollapsed}
    />
  </div>
  <div class="navbar__option">
    <PlaceholderIcon content="2" />
    <NavbarOptionText
      header="Reports"
      desc="View visualized aggregate information about transactions"
      isCollapsed={isCollapsed}
    />
  </div>
  <div class="navbar__option">
    <PlaceholderIcon content="3" />
    <NavbarOptionText
      header="Categories & Transactions"
      desc="Add new transactions, modify, add or remove categories"
      isCollapsed={isCollapsed}
    />
  </div>
  <div class="navbar__spacer navbar__landscape" />
  <div class="navbar__option navbar__landscape" on:click={collapse}>
    <PlaceholderIcon content="4"/>
    <NavbarOptionText
      header="Collapse"
      desc="Collapse the sidebar"
      isCollapsed={isCollapsed}
    />
  </div>
  <div class="navbar__option">
    <PlaceholderIcon content="5" />
    <NavbarOptionText
      header="Options"
      desc="Application options"
      isCollapsed={isCollapsed}
    />
  </div>
</div>

<style lang="scss">
  .navbar__spacer {
    height: 100%;
  }

  .navbar__option {
    margin: 10px;
    display: flex;
    align-items: center;
    width: fit-content;
    flex-direction: row;
  }
</style>
