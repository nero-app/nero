<script lang="ts">
  import LoadExtensionModal from "../../components/LoadExtensionModal.svelte";
  import InboxIcon from "../../components/icons/InboxIcon.svelte";
  import PlusIcon from "../../components/icons/PlusIcon.svelte";
  import WarningIcon from "../../components/icons/WarningIcon.svelte";
  import { appState } from "../../lib/appState.svelte";
  import type { Metadata } from "@nero/plugin-extensions";
  import { open } from "@tauri-apps/plugin-dialog";

  let selectedFile = $state<string | null>(null);
  let showAddDialog = $state(false);
  let allowUntrusted = $state(true);

  const currentExtension = $derived(appState.extension);

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

  // TODO:
  function toggleUntrustedExtensions() {
    allowUntrusted = !allowUntrusted;
  }
</script>

{#snippet extensionCard(metadata: Metadata)}
  <div class="px-4 py-3">
    <h3 class="truncate text-sm font-medium text-neutral-900">
      {metadata.name}
    </h3>
    <p class="text-xs text-neutral-500">
      v{metadata.version} • {metadata.authors}
    </p>
  </div>
{/snippet}

{#snippet extensionSettings()}
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
      {@render extensionCard(currentExtension.metadata)}
    {/if}
  </section>
{/snippet}

{#snippet securitySettings()}
  <section class="p-4">
    <h3 class="text-sm font-medium text-neutral-900">Security</h3>
    <div class="mt-3 flex items-center gap-3">
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button
        class={`relative inline-flex h-6 w-11 shrink-0 items-center rounded-full
  transition-colors ${allowUntrusted ? "bg-amber-500" : "bg-neutral-200"}`}
        onclick={toggleUntrustedExtensions}
      >
        <span
          class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
      allowUntrusted ? "translate-x-6" : "translate-x-1" }`}
        ></span>
      </button>
      <div class="flex-1">
        <p class="block text-sm font-medium text-neutral-900">
          Allow unsigned extensions
        </p>
        <p class="mt-1 text-xs text-neutral-600">
          Enable loading of extensions that are not officially signed. Only
          enable this if you trust the source.
        </p>
      </div>
    </div>
    {#if allowUntrusted}
      <div
        class="mt-3 flex items-center gap-2 rounded-md border border-amber-200 bg-amber-50 p-3"
      >
        <div class="size-6 text-amber-600">
          <WarningIcon />
        </div>
        <p class="text-xs text-amber-800">
          Warning: Unsigned extensions could load data from untrusted sources.
          Only load extensions from trusted origins.
        </p>
      </div>
    {/if}
  </section>
{/snippet}

{#snippet advancedSettings()}
  <section class="rounded-lg border border-neutral-200">
    <header class="border-b border-neutral-200 px-4 py-3">
      <h2 class="font-medium text-neutral-900">Advanced Configuration</h2>
    </header>
    <div class="divide-y divide-neutral-200">
      {@render securitySettings()}
    </div>
  </section>
{/snippet}

<div class="flex flex-col gap-4 pb-2">
  <header>
    <h1 class="text-xl font-semibold text-neutral-900">Extensions</h1>
    <p class="text-neutral-600">Manage and configure extensions</p>
  </header>
  {@render extensionSettings()}
  {@render advancedSettings()}
</div>

{#if showAddDialog && selectedFile}
  <LoadExtensionModal filePath={selectedFile} onClose={closeDialog} />
{/if}
