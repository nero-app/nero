<script lang="ts">
  import sortIcon from "../assets/icons/sort_icon.svg";
  import { createInfiniteScroll } from "../lib/infiniteScroll.svelte";
  import EpisodeCard from "./EpisodeCard.svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import type { EpisodesPage, Episode } from "@nero/plugin-extensions";
  import type {
    CreateInfiniteQueryResult,
    InfiniteData,
  } from "@tanstack/svelte-query";

  interface EpisodesListProps {
    episodesQuery: CreateInfiniteQueryResult<InfiniteData<EpisodesPage>>;
    seriesId: string;
    smallCards?: boolean;
    onEpisodeSelect?: (episode: Episode) => void;
  }
  let {
    episodesQuery,
    seriesId,
    smallCards = false,
    onEpisodeSelect,
  }: EpisodesListProps = $props();

  let infiniteScroll = createInfiniteScroll(() =>
    $episodesQuery.fetchNextPage(),
  );

  let episodes = $derived(
    $episodesQuery.data?.pages.flatMap((page) => page.items) ?? [],
  );

  function handleEpisodeClick(episode: Episode) {
    onEpisodeSelect?.(episode);
  }
</script>

{#snippet episodesList()}
  <ul>
    {#each episodes as episode (episode.id)}
      <li>
        <EpisodeCard
          {seriesId}
          {episode}
          small={smallCards}
          onclick={() => handleEpisodeClick(episode)}
        />
      </li>
    {/each}
  </ul>
  <div bind:this={infiniteScroll.element}></div>
  {#if $episodesQuery.isFetchingNextPage}
    <p class="p-2 text-center text-sm text-gray-500">Loading more...</p>
  {/if}
{/snippet}

<section class="flex size-full flex-col">
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
    <ErrorMessage
      message="Apparently an error has occurred"
      error={$episodesQuery.error}
    />
  {:else if $episodesQuery.isSuccess}
    {@render episodesList()}
  {/if}
</section>
