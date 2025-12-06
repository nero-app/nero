<script lang="ts">
  import { appState } from "../lib/appState.svelte";
  import {
    createExtensionMetadataQuery,
    createLoadExtensionMutation,
  } from "../lib/queries";
  import XMarkIcon from "./icons/XMarkIcon.svelte";
  import { onMount } from "svelte";

  interface LoadExtensionModalProps {
    filePath: string;
    onClose: () => void;
  }
  let { filePath, onClose }: LoadExtensionModalProps = $props();

  const extension = $derived(appState.extension);
  const metadataQuery = createExtensionMetadataQuery(filePath);
  const loadMutation = createLoadExtensionMutation();

  let dialogElement: HTMLDialogElement;

  onMount(() => {
    if (dialogElement && !dialogElement.open) {
      dialogElement.showModal();
    }
  });

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === dialogElement) {
      onClose();
    }
  }
  function handleCancel(e: Event) {
    e.preventDefault();
    onClose();
  }

  async function handleLoad() {
    await $loadMutation.mutateAsync(filePath);
  }
</script>

<dialog
  bind:this={dialogElement}
  onclick={handleBackdropClick}
  oncancel={handleCancel}
  class="m-auto w-full max-w-lg rounded-lg shadow-xl backdrop:bg-black/50"
>
  <div class="flex flex-col">
    <div
      class="flex items-center justify-between border-b border-neutral-200 px-6 py-4"
    >
      <h2 class="text-lg font-semibold text-neutral-900">Load Extension</h2>
      <button
        onclick={onClose}
        type="button"
        class="size-6 cursor-pointer rounded-md duration-300 active:scale-95"
      >
        <XMarkIcon />
      </button>
    </div>

    <div class="p-6">
      {#if $metadataQuery.isPending}
        <div class="flex items-center justify-center py-8">
          <div
            class="size-8 animate-spin rounded-full border-4 border-neutral-200
              border-t-neutral-900"
          ></div>
        </div>
      {:else if $metadataQuery.isError}
        <div class="rounded-md border border-red-200 bg-red-50 p-4">
          <p class="text-sm text-red-800">{$metadataQuery.error.message}</p>
        </div>
      {:else if $metadataQuery.data}
        <div class="space-y-4">
          <div>
            <h3 class="text-base font-medium text-neutral-900">
              {$metadataQuery.data.name}
            </h3>
            <p class="mt-1 text-sm text-neutral-600">
              {$metadataQuery.data.description}
            </p>
          </div>

          <div class="space-y-2 rounded-md bg-neutral-50 p-3 text-sm">
            <div class="flex justify-between">
              <span class="text-neutral-600">Version:</span>
              <span class="font-medium text-neutral-900"
                >v{$metadataQuery.data.version}</span
              >
            </div>
            <div class="flex justify-between">
              <span class="text-neutral-600">Authors:</span>
              <span class="font-medium text-neutral-900"
                >{$metadataQuery.data.authors}</span
              >
            </div>
            <div class="flex justify-between">
              <span class="text-neutral-600">Source:</span>
              <span class="font-medium text-neutral-900"
                >{$metadataQuery.data.source}</span
              >
            </div>
          </div>

          {#if extension}
            <div class="rounded-md border border-amber-200 bg-amber-50 p-3">
              <p class="text-sm text-amber-800">
                Loading this extension will replace the currently loaded
                extension.
              </p>
            </div>
          {/if}

          {#if $loadMutation.isError}
            <div class="rounded-md border border-red-200 bg-red-50 p-3">
              <p class="text-sm text-red-800">{$loadMutation.error.message}</p>
            </div>
          {/if}

          {#if $loadMutation.isSuccess}
            <div class="rounded-md border border-green-200 bg-green-50 p-3">
              <p class="text-sm text-green-800">
                Extension loaded successfully
              </p>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <div class="flex justify-end gap-3 border-t border-neutral-200 px-6 py-4">
      <button
        onclick={onClose}
        type="button"
        class="cursor-pointer rounded-md border border-neutral-300 px-4 py-2 text-sm
          font-medium text-neutral-700 duration-300 active:scale-95"
      >
        Cancel
      </button>
      <button
        onclick={handleLoad}
        type="button"
        disabled={$metadataQuery.isPending || $loadMutation.isPending}
        class="cursor-pointer rounded-md bg-orange-200 px-4 py-2 text-sm font-medium
          text-neutral-900 duration-300 active:scale-95 disabled:cursor-not-allowed
          disabled:opacity-50"
      >
        {$loadMutation.isPending ? "Loading..." : "Load Extension"}
      </button>
    </div>
  </div>
</dialog>
