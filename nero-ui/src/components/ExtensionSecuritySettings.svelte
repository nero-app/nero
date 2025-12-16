<script lang="ts">
  import WarningIcon from "./icons/WarningIcon.svelte";

  let allowUntrusted = $state(true);

  // TODO:
  function toggleUntrustedExtensions() {
    allowUntrusted = !allowUntrusted;
  }
</script>

<section class="rounded-lg border border-neutral-200">
  <header class="border-b border-neutral-200 px-4 py-3">
    <h2 class="font-medium text-neutral-900">Security</h2>
    <p class="text-xs text-neutral-500">
      Configure security settings for extensions
    </p>
  </header>
  <div class="p-4">
    <div class="flex items-center gap-3">
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
  </div>
</section>
