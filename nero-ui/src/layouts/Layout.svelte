<script lang="ts">
  import Toolbar from "../components/Toolbar.svelte";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="fixed flex h-screen w-full flex-col gap-4 px-12 pt-4">
  <Toolbar />
  <main class="h-full overflow-auto">
    {@render children?.()}
  </main>
</div>
