<script lang="ts">
  import { appState } from "../lib/appState.svelte";
  import LoadExtensionModal from "./LoadExtensionModal.svelte";
  import InboxIcon from "./icons/InboxIcon.svelte";
  import PlusIcon from "./icons/PlusIcon.svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  const currentExtension = $derived(appState.extension);
  let selectedFile = $state<string | null>(null);
  let showAddDialog = $state(false);

  async function selectExtension() {
    const file = await open({
      filters: [{ name: "Extension", extensions: ["wasm"] }],
    });
    if (file) {
      selectedFile = file;
      showAddDialog = true;
    }
  }

  function closeDialog() {
    showAddDialog = false;
    selectedFile = null;
  }
</script>

<section class="rounded-lg border border-neutral-200">
  <header
    class="flex items-center justify-between border-b border-neutral-200 px-4 py-3"
  >
    <h2 class="font-medium text-neutral-900">Loaded Extension</h2>
    <button
      onclick={selectExtension}
      class="size-6 cursor-pointer rounded-md duration-300 active:scale-95"
    >
      <PlusIcon />
    </button>
  </header>
  {#if !currentExtension}
    <div class="flex flex-col items-center px-4 py-8">
      <div class="size-12 text-neutral-400">
        <InboxIcon />
      </div>
      <p class="text-sm text-neutral-600">No extension loaded</p>
      <p class="text-xs text-neutral-500">
        Only one extension can be loaded at a time
      </p>
    </div>
  {:else}
    <div class="px-4 py-3">
      <h3 class="truncate text-sm font-medium text-neutral-900">
        {currentExtension.metadata.name}
      </h3>
      <p class="text-xs text-neutral-500">
        v{currentExtension.metadata.version} • {currentExtension.metadata
          .authors}
      </p>
    </div>
  {/if}
</section>

{#if showAddDialog && selectedFile}
  <LoadExtensionModal filePath={selectedFile} onClose={closeDialog} />
{/if}
