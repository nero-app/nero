<script lang="ts">
  import { appState } from "../lib/appState.svelte";
  import { createMutation } from "../lib/createMutation.svelte";
  import WarningIcon from "./icons/WarningIcon.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  const REQUIRED_WEBTORRENT_VERSION = ">=5.0.0, <6.0.0";

  let torrentConfigType = $state(appState.config.torrentConfigType);
  let torrentCustomPath = $state(appState.config.torrentCustomPath);
  let torrentAutoDownload = $state(appState.config.torrentAutoDownload);

  type TorrentConfig =
    | { type: "autoDetect"; autoDownload: boolean }
    | { type: "custom"; webtorrent: string };

  const initMutation = createMutation(async (config: TorrentConfig) => {
    await invoke("initialize_webtorrent", { config });

    appState.config.torrentConfigType = torrentConfigType;
    appState.config.torrentCustomPath = torrentCustomPath;
    appState.config.torrentAutoDownload = torrentAutoDownload;
  });

  let isNodeConfigured = $derived(appState.config.nodeEnabled);

  function setAutoDetectMode() {
    torrentConfigType = "auto";
  }

  function setCustomPathMode() {
    torrentConfigType = "custom";
  }

  function toggleAutoDownload() {
    torrentAutoDownload = !torrentAutoDownload;
  }

  async function selectWebtorrentPath() {
    const file = await open({ title: "Select WebTorrent Binary" });
    if (file) {
      torrentCustomPath = file;
    }
  }

  async function handleInitialize() {
    if (!isNodeConfigured) return;

    const config: TorrentConfig =
      torrentConfigType === "custom"
        ? {
            type: "custom",
            webtorrent: torrentCustomPath,
          }
        : {
            type: "autoDetect",
            autoDownload: torrentAutoDownload,
          };

    await initMutation.mutate(config);
  }

  function isApplyDisabled(): boolean {
    return (
      initMutation.isLoading ||
      (torrentConfigType === "custom" && !torrentCustomPath)
    );
  }
</script>

<section class="rounded-lg border border-neutral-200">
  <header class="border-b border-neutral-200 px-4 py-3">
    <h2 class="font-medium text-neutral-900">Torrent Configuration</h2>
    <p class="text-xs text-neutral-500">
      Configure WebTorrent paths for torrent functionality
    </p>
  </header>

  <div class="flex flex-col gap-4 p-4">
    {#if !isNodeConfigured}
      <div
        class="flex items-center gap-2 rounded-md border border-amber-200 bg-amber-50 p-3"
      >
        <div class="size-6 text-amber-600">
          <WarningIcon />
        </div>
        <div>
          <p class="text-sm font-medium text-amber-800">Node.js Required</p>
          <p class="mt-1 text-xs text-amber-700">
            WebTorrent requires Node.js to be configured first. Please complete
            the Node.js setup above before configuring WebTorrent.
          </p>
        </div>
      </div>
    {:else}
      <div class="flex gap-2">
        <button
          onclick={setAutoDetectMode}
          class={`flex-1 rounded-md border px-4 py-2 text-sm font-medium transition-colors ${
            torrentConfigType === "auto"
              ? "border-blue-600 bg-blue-50 text-blue-700"
              : "border-neutral-300 bg-white text-neutral-700 hover:bg-neutral-50"
          }`}
        >
          Auto Detect
        </button>
        <button
          onclick={setCustomPathMode}
          class={`flex-1 rounded-md border px-4 py-2 text-sm font-medium transition-colors ${
            torrentConfigType === "custom"
              ? "border-blue-600 bg-blue-50 text-blue-700"
              : "border-neutral-300 bg-white text-neutral-700 hover:bg-neutral-50"
          }`}
        >
          Custom Path
        </button>
      </div>

      {#if torrentConfigType === "auto"}
        <div
          class="flex items-center justify-between rounded-md border border-neutral-200
            bg-neutral-50 p-3"
        >
          <div>
            <p class="text-sm font-medium text-neutral-900">Auto Download</p>
            <p class="text-xs text-neutral-500">
              Automatically download WebTorrent {REQUIRED_WEBTORRENT_VERSION} if not
              found in system
            </p>
          </div>
          <!-- svelte-ignore a11y_consider_explicit_label -->
          <button
            onclick={toggleAutoDownload}
            class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
              torrentAutoDownload ? "bg-blue-600" : "bg-neutral-300"
            }`}
          >
            <span
              class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                torrentAutoDownload ? "translate-x-6" : "translate-x-1"
              }`}
            ></span>
          </button>
        </div>
      {:else}
        <div class="flex flex-col gap-4">
          <div>
            <p class="mb-1 block text-xs font-medium text-neutral-700">
              WebTorrent Binary Path
            </p>
            <div class="flex gap-2">
              <input
                type="text"
                bind:value={torrentCustomPath}
                placeholder="/usr/bin/webtorrent or C:\Program Files\webtorrent\webtorrent.exe"
                class="flex-1 rounded-md border border-neutral-300 px-3 py-2 text-sm"
              />
              <button
                onclick={selectWebtorrentPath}
                class="rounded-md border border-neutral-300 bg-white px-3 py-2 text-sm
                  hover:bg-neutral-50"
              >
                Browse
              </button>
            </div>
          </div>
        </div>
      {/if}

      <button
        onclick={handleInitialize}
        disabled={isApplyDisabled()}
        class={`w-full rounded-md px-4 py-2 text-sm font-medium text-white transition-colors
          disabled:cursor-not-allowed disabled:bg-neutral-300 ${
            initMutation.isLoading
              ? "cursor-not-allowed bg-blue-400"
              : "bg-blue-600 hover:bg-blue-700"
          }`}
      >
        {#if initMutation.isLoading}
          <span class="flex items-center justify-center gap-2">
            <div
              class="size-4 animate-spin rounded-full border-2 border-white/30 border-t-white"
            ></div>
            Applying Configuration...
          </span>
        {:else}
          Apply Configuration
        {/if}
      </button>

      {#if initMutation.isSuccess}
        <div class="rounded-md border border-green-200 bg-green-50 p-3">
          <p class="text-sm text-green-800">
            WebTorrent configuration applied successfully
          </p>
        </div>
      {/if}

      {#if initMutation.error}
        <div class="rounded-md border border-red-200 bg-red-50 p-3">
          <p class="text-sm font-medium text-red-800">Configuration failed:</p>
          <p class="mt-1 text-xs text-red-700">{initMutation.error.message}</p>
        </div>
      {/if}
    {/if}
  </div>
</section>
