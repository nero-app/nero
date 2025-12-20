<script lang="ts">
  import type { CreateInfiniteQueryResult } from "../lib/createInfiniteQuery.svelte";
  import { createInfiniteScroll } from "../lib/infiniteScroll.svelte";
  import EpisodeCard from "./EpisodeCard.svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import SortIcon from "./icons/SortIcon.svelte";
  import type { Episode } from "@nero/plugin-extensions";

  interface EpisodesListProps {
    episodesQuery: CreateInfiniteQueryResult<Episode>;
    seriesId: string;
    onEpisodeSelect?: (episode: Episode) => void;
  }
  let { episodesQuery, seriesId, onEpisodeSelect }: EpisodesListProps =
    $props();

  let infiniteScroll = createInfiniteScroll(() =>
    episodesQuery.fetchNextPage(),
  );

  function handleEpisodeClick(episode: Episode) {
    onEpisodeSelect?.(episode);
  }
</script>

{#snippet episodesList()}
  <ul>
    {#each episodesQuery.data as episode (episode.id)}
      <li>
        <EpisodeCard
          {seriesId}
          {episode}
          onclick={() => handleEpisodeClick(episode)}
        />
      </li>
    {/each}
  </ul>
  <div bind:this={infiniteScroll.element}></div>
  {#if episodesQuery.isFetchingNextPage}
    <p class="p-2 text-center text-sm text-gray-500">Loading more...</p>
  {/if}
{/snippet}

<section class="flex size-full flex-col">
  <header class="sticky top-0 z-10 bg-white">
    <div class="flex w-full items-center justify-between">
      <h2 class="text-2xl font-semibold">Episodes</h2>
      <!-- TODO: onclick -->
      <button class="cursor-pointer">
        <SortIcon />
      </button>
    </div>
    <hr class="border-gray-300" />
  </header>
  {#if episodesQuery.isLoading}
    <p>Loading...</p>
  {:else if episodesQuery.error}
    <ErrorMessage
      message="Apparently an error has occurred"
      error={episodesQuery.error}
    />
  {:else if episodesQuery.isSuccess}
    {@render episodesList()}
  {/if}
</section>
