<script lang="ts">
  import { api } from "$lib/api";
  import { onMount } from "svelte";

  let health = $state("loading...");

  onMount(async () => {
    try {
      const res = await api("/api/health");
      const data = await res.json();
      health = data.status;
    } catch {
      health = "backend unreachable";
    }
  });
</script>

<h1>openslate</h1>
<p>Backend: {health}</p>
