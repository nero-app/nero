<script lang="ts">
  import { appState } from "../../lib/appState.svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  let playerPath = $derived(appState.config.playerPath);
  let showManualInput = $state(false);
  let manualPath = $state("");

  async function selectPlayer() {
    const file = await open({
      title: "Choose Video Player",
      filters: [
        { name: "Application", extensions: ["app", "exe", "*"] },
        { name: "All Files", extensions: ["*"] },
      ],
    });
    if (file) {
      appState.config.playerPath = file;
    }
  }

  function setManualPath() {
    if (manualPath.trim()) {
      appState.config.playerPath = manualPath.trim();
      showManualInput = false;
    }
  }

  function removePlayer() {
    appState.config.playerPath = null;
  }
</script>

{#snippet playerCard(playerPath: string)}
  <div class="flex items-center gap-3 rounded-md bg-neutral-50 p-3">
    <p class="min-w-0 flex-1 text-sm font-medium text-neutral-900">
      {playerPath}
    </p>
    <button
      onclick={removePlayer}
      class="shrink-0 rounded-md border border-neutral-200 bg-white px-3 py-1.5 text-xs
        font-medium text-neutral-700 transition-colors hover:bg-neutral-50"
    >
      Change
    </button>
  </div>
{/snippet}

{#snippet manualPlayerInput()}
  {#if showManualInput}
    <div class="mt-3 flex gap-2">
      <input
        type="text"
        bind:value={manualPath}
        placeholder="/usr/bin/vlc or C:\Program Files\..."
        class="flex-1 rounded-md border border-neutral-300 px-3 py-2 text-sm"
      />
      <button
        onclick={setManualPath}
        class="rounded-md bg-neutral-900 px-4 py-2 text-sm text-white hover:bg-neutral-800"
      >
        Set
      </button>
      <button
        onclick={() => (showManualInput = false)}
        class="rounded-md border border-neutral-300 px-4 py-2 text-sm hover:bg-neutral-50"
      >
        Cancel
      </button>
    </div>
  {:else}
    <button
      onclick={() => (showManualInput = true)}
      class="mt-3 text-xs text-neutral-600 underline hover:text-neutral-900"
    >
      Or enter path manually
    </button>
  {/if}
{/snippet}

{#snippet playerSettings()}
  <section class="rounded-lg border border-neutral-200">
    <header class="border-b border-neutral-200 px-4 py-3">
      <h2 class="font-medium text-neutral-900">Default Player</h2>
      <p class="text-xs text-neutral-500">
        Choose which application will open video files
      </p>
    </header>
    {#if !playerPath}
      <div class="p-4">
        <button
          onclick={selectPlayer}
          class="w-full rounded-md border-2 border-dashed border-neutral-300 p-4 px-4 py-6
            text-center transition-colors hover:border-neutral-400"
        >
          <p class="text-sm text-neutral-600">Select Video Player</p>
          <p class="text-xs text-neutral-500">
            Click to browse for your preferred player
          </p>
        </button>
        {@render manualPlayerInput()}
      </div>
    {:else}
      {@render playerCard(playerPath)}
    {/if}
    <footer class="border-t border-neutral-200 bg-neutral-50 px-4 py-3">
      <p class="text-xs text-neutral-600">
        <strong>Tip:</strong> Popular options include VLC, MPV, and others. On
        Linux/macOS, you can use paths like
        <code class="rounded bg-neutral-200 px-1">/usr/bin/vlc</code>
      </p>
    </footer>
  </section>
{/snippet}

<div class="flex flex-col gap-4 pb-4">
  <header>
    <h1 class="text-xl font-semibold text-neutral-900">Application</h1>
    <p class="text-neutral-600">Customize how the application behaves</p>
  </header>
  {@render playerSettings()}
</div>
