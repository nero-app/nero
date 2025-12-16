<script lang="ts">
  import LoadedExtension from "../components/ExtensionLoader.svelte";
  import SecurityExtensionSettings from "../components/ExtensionSecuritySettings.svelte";
</script>

<div class="flex flex-col gap-4 pb-2">
  <header>
    <h1 class="text-xl font-semibold text-neutral-900">Extensions</h1>
    <p class="text-neutral-600">Manage and configure extensions</p>
  </header>
  <LoadedExtension />
  <SecurityExtensionSettings />
</div>
