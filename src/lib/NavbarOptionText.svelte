<script lang="ts">
  export let header: string;
  export let desc: string;
  export let isCollapsed: boolean = false;

</script>

<div class={`navbar__text ${isCollapsed ? 'collapsed' : 'open'}`}>
  <span class="navbar__text__header">{header}</span>
  <span class="navbar__text__desc">{desc}</span>
</div>

<style lang="scss">

  .navbar__text.collapsed {
    display: none;
  }

  .navbar__text {
    margin: 0 12px;
    display: flex;
    flex-direction: column;
    min-width: 150px;
  }

  .navbar__text__header {
    font-size: 1.1rem;
  }

  .navbar__text__desc {
    font-size: 0.7rem;
    font-style: italic;
  }
</style>
