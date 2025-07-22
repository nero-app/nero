<script lang="ts">
  import sortIcon from "../assets/icons/sort_icon.svg";
  import type { EpisodesPage } from "../types/page";
  import EpisodeCard from "./EpisodeCard.svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import type {
    CreateInfiniteQueryResult,
    InfiniteData,
  } from "@tanstack/svelte-query";

  interface EpisodesListProps {
    episodesQuery: CreateInfiniteQueryResult<InfiniteData<EpisodesPage>>;
    seriesId: string;
    smallCards?: boolean;
  }
  let {
    episodesQuery,
    seriesId,
    smallCards = false,
  }: EpisodesListProps = $props();

  let episodes = $derived(
    $episodesQuery.data?.pages.flatMap((page) => page.items) ?? [],
  );
</script>

{#snippet episodesList()}
  <ul>
    {#each episodes as episode (episode.id)}
      <li>
        <EpisodeCard {seriesId} {episode} small={smallCards} />
      </li>
    {/each}
  </ul>
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
    <ErrorMessage
      message="Apparently an error has occurred"
      error={$episodesQuery.error}
      centered
    />
  {:else if $episodesQuery.isSuccess}
    {@render episodesList()}
  {/if}
</section>
