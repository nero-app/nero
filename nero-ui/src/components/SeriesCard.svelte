<script lang="ts">
  import { link } from "../pages/Router.svelte";
  import type { Series } from "../types/series";

  let { series }: { series: Series } = $props();
</script>

<a
  class="flex flex-col gap-1 rounded-md p-1 duration-300 hover:bg-gray-200
    active:scale-95"
  href="/series/{series.id}"
  use:link
>
  <img
    class="aspect-[2/3] rounded-lg object-cover"
    src={series.posterUrl}
    alt={series.title}
  />
  <h3 class="truncate text-center text-sm font-medium">{series.title}</h3>
</a>
