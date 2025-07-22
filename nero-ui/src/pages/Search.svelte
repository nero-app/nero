<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import ErrorMessage from "../components/ErrorMessage.svelte";
  import SeriesCard from "../components/SeriesCard.svelte";
  import { createInfiniteScroll } from "../state/createInfiniteScroll.svelte";
  import { infiniteSearchQuery } from "../state/queries.svelte";

  let { querystring }: { querystring: string } = $props();
  $inspect(querystring);

  let searchQuery = $derived(
    decodeURIComponent(querystring.substring(querystring.indexOf("q=") + 2)),
  );
  let seriesQuery = $derived(infiniteSearchQuery(searchQuery));
  let infiniteScroll = createInfiniteScroll(() => $seriesQuery.fetchNextPage());
  let series = $derived(
    $seriesQuery.data?.pages.flatMap((page) => page.items) ?? [],
  );
</script>

{#snippet filterList()}
  <p>Filters go here!</p>
{/snippet}

{#snippet seriesList()}
  <ul class="grid grid-cols-4">
    {#each series as series (series.id)}
      <li>
        <SeriesCard {series} />
      </li>
    {/each}
    <div bind:this={infiniteScroll.element}></div>
    {#if $seriesQuery.isFetchingNextPage}
      <p class="p-2 text-center text-sm text-gray-500">Loading more...</p>
    {/if}
  </ul>
{/snippet}

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <div class="overflow-y-auto">
    {#if $seriesQuery.isLoading}
      <p>Loading...</p>
    {:else if $seriesQuery.isError}
      <ErrorMessage
        message="Apparently an error has occurred"
        imageSrc={shockedCat}
        error={$seriesQuery.error}
        centered
      />
    {:else if $seriesQuery.isSuccess}
      {@render seriesList()}
    {/if}
  </div>
  <aside class="overflow-y-auto">
    {@render filterList()}
  </aside>
</div>
