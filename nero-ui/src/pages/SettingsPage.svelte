<script lang="ts">
  import AppSettingsPage from "./AppSettingsPage.svelte";
  import ExtensionsSettingsPage from "./ExtensionsSettingsPage.svelte";
  import { link } from "./Router.svelte";
  import type { Component } from "svelte";

  let { params }: { params: { section?: string } } = $props();

  const settingsRoutes: Record<string, Component> = {
    app: AppSettingsPage,
    extensions: ExtensionsSettingsPage,
  };

  const activeSection = $derived(params?.section || "app");
  const ActiveComponent = $derived(
    settingsRoutes[activeSection] || settingsRoutes.app,
  );
</script>

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <div class="size-full overflow-y-auto">
    <ActiveComponent />
  </div>
  <aside class="overflow-y-auto">
    <nav>
      <ul>
        <li>
          <a href="/settings/app" use:link> App </a>
        </li>
        <li>
          <a href="/settings/extensions" use:link> Extensions </a>
        </li>
      </ul>
    </nav>
  </aside>
</div>
