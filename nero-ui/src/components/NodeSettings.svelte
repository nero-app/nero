<script lang="ts">
  import { appState } from "../lib/appState.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  const REQUIRED_NODE_VERSION = ">=17.0.0, <23.0.0";

  let nodeConfigType = $state(appState.config.nodeConfigType);
  let nodeCustomPath = $state(appState.config.nodeCustomPath);
  let npmCustomPath = $state(appState.config.npmCustomPath);
  let nodeAutoDownload = $state(appState.config.nodeAutoDownload);

  let isInitializing = $state(false);
  let initError = $state<string | null>(null);
  let initSuccess = $state(false);

  function setAutoDetectMode() {
    nodeConfigType = "auto";
  }

  function setCustomPathsMode() {
    nodeConfigType = "custom";
  }

  function toggleAutoDownload() {
    nodeAutoDownload = !nodeAutoDownload;
  }

  async function selectNodePath() {
    const file = await open({ title: "Select Node.js Binary" });
    if (file) {
      nodeCustomPath = file;
    }
  }

  async function selectNpmPath() {
    const file = await open({ title: "Select NPM Binary" });
    if (file) {
      npmCustomPath = file;
    }
  }

  async function initializeNodeJs() {
    isInitializing = true;
    initError = null;
    initSuccess = false;

    try {
      const config =
        nodeConfigType === "custom"
          ? {
              type: "custom",
              node: nodeCustomPath,
              npm: npmCustomPath,
            }
          : {
              type: "autoDetect",
              autoDownload: nodeAutoDownload,
            };

      await invoke("initialize_nodejs", { config });

      initSuccess = true;

      appState.config.nodeEnabled = true;
      appState.config.nodeConfigType = nodeConfigType;
      appState.config.nodeCustomPath = nodeCustomPath;
      appState.config.npmCustomPath = npmCustomPath;
      appState.config.nodeAutoDownload = nodeAutoDownload;
    } catch (error) {
      initError = error instanceof Error ? error.message : String(error);
    } finally {
      isInitializing = false;
    }
  }

  function isApplyDisabled(): boolean {
    return (
      isInitializing ||
      (nodeConfigType === "custom" && (!nodeCustomPath || !npmCustomPath))
    );
  }
</script>

<section class="rounded-lg border border-neutral-200">
  <header class="border-b border-neutral-200 px-4 py-3">
    <h2 class="font-medium text-neutral-900">Node.js Configuration</h2>
    <p class="text-xs text-neutral-500">
      Configure Node.js paths for tools that require it ({REQUIRED_NODE_VERSION})
    </p>
  </header>

  <div class="flex flex-col gap-4 p-4">
    <div class="flex gap-2">
      <button
        onclick={setAutoDetectMode}
        class={`flex-1 rounded-md border px-4 py-2 text-sm font-medium transition-colors ${
  nodeConfigType === "auto"
            ? "border-blue-600 bg-blue-50 text-blue-700"
            : "border-neutral-300 bg-white text-neutral-700 hover:bg-neutral-50"
  }`}
      >
        Auto Detect
      </button>
      <button
        onclick={setCustomPathsMode}
        class={`flex-1 rounded-md border px-4 py-2 text-sm font-medium transition-colors ${
  nodeConfigType === "custom"
            ? "border-blue-600 bg-blue-50 text-blue-700"
            : "border-neutral-300 bg-white text-neutral-700 hover:bg-neutral-50"
  }`}
      >
        Custom Paths
      </button>
    </div>

    {#if nodeConfigType === "auto"}
      <div
        class="flex items-center justify-between rounded-md border border-neutral-200
          bg-neutral-50 p-3"
      >
        <div>
          <p class="text-sm font-medium text-neutral-900">Auto Download</p>
          <p class="text-xs text-neutral-500">
            Automatically download Node.js {REQUIRED_NODE_VERSION} if not found in
            system PATH
          </p>
        </div>
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
          onclick={toggleAutoDownload}
          class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
        nodeAutoDownload ? "bg-blue-600" : "bg-neutral-300" }`}
        >
          <span
            class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
            nodeAutoDownload ? "translate-x-6" : "translate-x-1" }`}
          ></span>
        </button>
      </div>
    {:else}
      <div class="flex flex-col gap-4">
        <div>
          <p class="mb-1 block text-xs font-medium text-neutral-700">
            Node Binary Path
          </p>
          <div class="flex gap-2">
            <input
              type="text"
              bind:value={nodeCustomPath}
              placeholder="/usr/bin/node or C:\Program Files\nodejs\node.exe"
              class="flex-1 rounded-md border border-neutral-300 px-3 py-2 text-sm"
            />
            <button
              onclick={selectNodePath}
              class="rounded-md border border-neutral-300 bg-white px-3 py-2 text-sm
                hover:bg-neutral-50"
            >
              Browse
            </button>
          </div>
        </div>
        <div>
          <p class="mb-1 block text-xs font-medium text-neutral-700">
            NPM Binary Path
          </p>
          <div class="flex gap-2">
            <input
              type="text"
              bind:value={npmCustomPath}
              placeholder="/usr/bin/npm or C:\Program Files\nodejs\npm.cmd"
              class="flex-1 rounded-md border border-neutral-300 px-3 py-2 text-sm"
            />
            <button
              onclick={selectNpmPath}
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
      onclick={initializeNodeJs}
      disabled={isApplyDisabled()}
      class={`w-full rounded-md px-4 py-2 text-sm font-medium text-white transition-colors
            disabled:cursor-not-allowed disabled:bg-neutral-300 ${
            isInitializing
                ? "cursor-not-allowed bg-blue-400"
                : "bg-blue-600 hover:bg-blue-700"
            }`}
    >
      {#if isInitializing}
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

    {#if initSuccess}
      <div class="rounded-md border border-green-200 bg-green-50 p-3">
        <p class="text-sm text-green-800">
          Node.js configuration applied successfully
        </p>
      </div>
    {/if}

    {#if initError}
      <div class="rounded-md border border-red-200 bg-red-50 p-3">
        <p class="text-sm font-medium text-red-800">Configuration failed:</p>
        <p class="mt-1 text-xs text-red-700">{initError}</p>
      </div>
    {/if}
  </div>
</section>
