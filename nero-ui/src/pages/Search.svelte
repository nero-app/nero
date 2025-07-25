<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import ErrorMessage from "../components/ErrorMessage.svelte";
  import SeriesCard from "../components/SeriesCard.svelte";
  import { createInfiniteScroll } from "../state/createInfiniteScroll.svelte";
  import {
    createFiltersQuery,
    createInfiniteSearchQuery,
  } from "../state/queries.svelte";
  import type { Filter, FilterCategory, SearchFilter } from "../types/filters";

  let { querystring }: { querystring: string } = $props();
  $inspect(querystring);

  let filtersQuery = createFiltersQuery();
  let searchFilters = $state<SearchFilter[]>([]);

  let query = $derived(
    decodeURIComponent(querystring.substring(querystring.indexOf("q=") + 2)),
  );
  let searchQuery = $derived(
    createInfiniteSearchQuery(query, 1, searchFilters),
  );
  let infiniteScroll = createInfiniteScroll(() => $searchQuery.fetchNextPage());
  let series = $derived(
    $searchQuery.data?.pages.flatMap((page) => page.items) ?? [],
  );

  function toggleFilter(categoryId: string, filterId: string) {
    const category = searchFilters.find((sf) => sf.id === categoryId);

    if (!category) {
      searchFilters.push({
        id: categoryId,
        values: [filterId],
      });
      return;
    }

    const index = category.values.indexOf(filterId);
    if (index >= 0) {
      category.values.splice(index, 1);
      if (category.values.length === 0) {
        const i = searchFilters.findIndex((sf) => sf.id === categoryId);
        searchFilters.splice(i, 1);
      }
    } else {
      category.values.push(filterId);
    }
  }

  function isFilterSelected(categoryId: string, filterId: string): boolean {
    return searchFilters.some(
      (sf) => sf.id === categoryId && sf.values.includes(filterId),
    );
  }
</script>

{#snippet filter(categoryId: string, filter: Filter)}
  <button
    class="flex w-full cursor-pointer items-center gap-2 rounded px-1 text-left
      hover:bg-gray-50"
    onclick={() => toggleFilter(categoryId, filter.id)}
  >
    <input
      class="pointer-events-none cursor-pointer"
      type="checkbox"
      checked={isFilterSelected(categoryId, filter.id)}
      tabindex="-1"
    />
    <p>{filter.displayName}</p>
  </button>
{/snippet}

{#snippet filterCategory(category: FilterCategory)}
  <details open>
    <summary>{category.displayName}</summary>
    <ul>
      {#each category.filters as f}
        <li>
          {@render filter(category.id, f)}
        </li>
      {/each}
    </ul>
  </details>
{/snippet}

{#snippet filtersList(categories: FilterCategory[])}
  <ul>
    {#each categories as category}
      <li>
        {@render filterCategory(category)}
      </li>
    {/each}
  </ul>
{/snippet}

{#snippet seriesList()}
  <ul class="grid grid-cols-4">
    {#each series as series (series.id)}
      <li>
        <SeriesCard {series} />
      </li>
    {/each}
    <div bind:this={infiniteScroll.element}></div>
    {#if $searchQuery.isFetchingNextPage}
      <p class="p-2 text-center text-sm text-gray-500">Loading more...</p>
    {/if}
  </ul>
{/snippet}

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <div class="overflow-y-auto">
    {#if $searchQuery.isLoading}
      <p>Loading...</p>
    {:else if $searchQuery.isError}
      <ErrorMessage
        message="Apparently an error has occurred"
        imageSrc={shockedCat}
        error={$searchQuery.error}
      />
    {:else if $searchQuery.isSuccess}
      {@render seriesList()}
    {/if}
  </div>
  <aside class="overflow-y-auto">
    {#if $filtersQuery.isLoading}
      <p>Loading...</p>
    {:else if $filtersQuery.isError}
      <ErrorMessage
        message="Apparently an error has occurred"
        error={$filtersQuery.error}
      />
    {:else if $filtersQuery.isSuccess}
      {@render filtersList($filtersQuery.data)}
    {/if}
  </aside>
</div>
