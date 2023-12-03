<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  const resetDB = async () => {
    const response = await invoke('reset_database');
    console.log(response);
  };

  let isVisible = false;
</script>

<section>
  <div>
    <span>Show options</span>
    <input type="checkbox" bind:checked={isVisible} />
  </div>

  {#if isVisible}
    <button on:click={resetDB}>Reset DB</button>
  {/if}
</section>

<style>
  section {
    margin: 10px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }
  button {
    margin: 10px;
  }
</style>
