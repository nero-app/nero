<script lang="ts">
  import sortIcon from "../assets/icons/sort_icon.svg";
  import type { EpisodesPage } from "../types/page";
  import EpisodeCard from "./EpisodeCard.svelte";
  import SmallEpisodeCard from "./SmallEpisodeCard.svelte";
  import type {
    CreateInfiniteQueryResult,
    InfiniteData,
  } from "@tanstack/svelte-query";

  interface EpisodesListProps {
    episodesQuery: CreateInfiniteQueryResult<InfiniteData<EpisodesPage>>;
    seriesId: string;
    smallCard?: boolean;
  }
  let {
    episodesQuery,
    seriesId,
    smallCard = false,
  }: EpisodesListProps = $props();
</script>

{#snippet errorState(error: Error)}
  <article class="flex size-full flex-col items-center justify-center gap-2">
    <p class="text-center">
      Whoops...
      <br />
      Apparently an error has occurred while loading the episodes.
    </p>
    <pre
      class="rounded-md border border-gray-300 bg-gray-100 p-4 text-sm break-words
        whitespace-pre-wrap text-gray-800">{error.message}</pre>
  </article>
{/snippet}

<section>
  <header class="sticky top-0 z-10 bg-white">
    <div class="flex w-full items-center justify-between">
      <h2 class="text-2xl font-semibold">Episodes</h2>
      <!-- TODO: onclick -->
      <button class="cursor-pointer">
        <img src={sortIcon} alt="Sort icon" />
      </button>
    </div>
    <hr class="border-gray-300" />
  </header>

  {#if $episodesQuery.isLoading}
    <p>Loading...</p>
  {:else if $episodesQuery.isError}
    {@render errorState($episodesQuery.error)}
  {:else if $episodesQuery.isSuccess}
    <ul>
      {#each $episodesQuery.data.pages as page, pageIndex (pageIndex)}
        {#each page.items as episode (episode.id)}
          <li>
            {#if smallCard}
              <SmallEpisodeCard {seriesId} {episode} />
            {:else}
              <EpisodeCard {seriesId} {episode} />
            {/if}
          </li>
        {/each}
      {/each}
    </ul>
  {/if}
</section>
