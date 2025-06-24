<script lang="ts">
  import SeriesCard from "../components/SeriesCard.svelte";
  import { infiniteSearchQuery } from "../state/queries.svelte";

  let { querystring }: { querystring: string } = $props();
  $inspect(querystring);

  let searchQuery = $derived(
    decodeURIComponent(querystring.substring(querystring.indexOf("q=") + 2)),
  );
  let seriesQuery = $derived(infiniteSearchQuery(searchQuery));
</script>

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <div class="overflow-y-auto">
    {#if $seriesQuery.isLoading}
      <p>Loading...</p>
    {:else if $seriesQuery.isError}
      <p>Error: {$seriesQuery.error.message}</p>
    {:else if $seriesQuery.isSuccess}
      <ul class="grid grid-cols-4">
        {#each $seriesQuery.data.pages as page, pageIndex (pageIndex)}
          {#each page.items as series (series.id)}
            <li>
              <SeriesCard {series} />
            </li>
          {/each}
        {/each}
      </ul>
    {/if}
  </div>
  <aside class="overflow-y-auto">Filters go here!</aside>
</div>
