<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import SeriesCard from "../components/SeriesCard.svelte";
  import { infiniteSearchQuery } from "../state/queries.svelte";

  let { querystring }: { querystring: string } = $props();
  $inspect(querystring);

  let searchQuery = $derived(
    decodeURIComponent(querystring.substring(querystring.indexOf("q=") + 2)),
  );
  let seriesQuery = $derived(infiniteSearchQuery(searchQuery));
</script>

{#snippet errorState(error: Error)}
  <article class="flex size-full flex-col items-center justify-center gap-2">
    <p class="text-center">
      Whoops...
      <br />
      Apparently an error has occurred.
    </p>
    <div class="flex items-center">
      <img class="w-56" src={shockedCat} alt="Shocked cat" />
      <pre
        class="max-h-56 overflow-auto rounded-md border border-gray-300 bg-gray-100 p-4 text-sm
          break-words whitespace-pre-wrap text-gray-800">{error.message}</pre>
    </div>
  </article>
{/snippet}

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <div class="overflow-y-auto">
    {#if $seriesQuery.isLoading}
      <p>Loading...</p>
    {:else if $seriesQuery.isError}
      {@render errorState($seriesQuery.error)}
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
