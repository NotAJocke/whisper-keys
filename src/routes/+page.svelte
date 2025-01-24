<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { data } = $props() as any;
  let packs = data.packs;

  let error: string = $state("");
  let selected: string = $state("");

  $effect(() => {
    if (selected == "") return;

    selectPack(selected);
  });

  function selectPack(pack: string) {
    invoke("select_pack", { pack })
      .then(() => (error = ""))
      .catch((err) => (error = err));
  }
</script>

<main class="container">
  {#if error}
    <p>{error}</p>
  {/if}

  <select bind:value={selected}>
    {#each packs as pack}
      <option>{pack}</option>
    {/each}
  </select>
</main>

<style></style>
