<script lang="ts">
  import playIcon from "../assets/icons/play_icon.svg";
  import shareIcon from "../assets/icons/share_icon.svg";
  import EpisodesList from "../components/EpisodesList.svelte";
  import {
    infiniteEpisodesQuery,
    seriesInfoQuery,
  } from "../state/queries.svelte";
  import type { Series } from "../types/series";
  import { link } from "./Router.svelte";

  let { params }: { params: { seriesId: string } } = $props();

  let seriesQuery = seriesInfoQuery(params.seriesId);
  let episodesQuery = infiniteEpisodesQuery(params.seriesId);

  // TODO:
  const firstEpisodeId = 0;
</script>

{#snippet seriesHeaderSkeleton()}
  <header class="flex flex-col gap-4">
    <div class="h-9 w-3/4 animate-pulse rounded bg-gray-200"></div>

    <div class="flex gap-4">
      <div class="h-10 w-32 animate-pulse rounded-lg bg-gray-200"></div>
      <div class="h-10 w-40 animate-pulse rounded-lg bg-gray-200"></div>
    </div>

    <div class="flex flex-col gap-2">
      <div class="h-4 w-full animate-pulse rounded bg-gray-200"></div>
      <div class="h-4 w-full animate-pulse rounded bg-gray-200"></div>
      <div class="h-4 w-4/5 animate-pulse rounded bg-gray-200"></div>
      <div class="h-4 w-3/5 animate-pulse rounded bg-gray-200"></div>
    </div>
  </header>
{/snippet}

{#snippet seriesErrorState(error: Error)}
  <header class="flex flex-col gap-4">
    <h1 class="text-3xl font-bold">Whoops...</h1>
    <p>
      Apparently an error has occurred while loading the series information.
    </p>
    <pre
      class="whitespace-pre-wrap break-words rounded-md border border-gray-300 bg-gray-100
        p-4 text-sm text-gray-800">{error.message}</pre>
  </header>
{/snippet}

{#snippet seriesHeader(series: Series)}
  <header class="flex flex-col gap-4">
    <h1 class="truncate text-3xl font-bold">{series.title}</h1>
    <div class="flex gap-4">
      <!-- TODO: Disable if episodes are not available or if the page does not contain any episodes -->
      <a
        class="rounded-lg bg-red-300 px-3 py-1.5 duration-300 active:scale-95"
        href="/watch/{series.id}/{firstEpisodeId}"
        use:link
      >
        <div class="flex items-center gap-2">
          <img src={playIcon} alt="Play icon" />
          <span>Watch now</span>
        </div>
      </a>
      <!-- TODO: onclick -->
      <button
        class="cursor-pointer rounded-lg bg-red-300 px-3 py-1.5 duration-300 active:scale-95"
      >
        <div class="flex items-center gap-2">
          <img src={shareIcon} alt="Share icon" />
          <span>Share the series</span>
        </div>
      </button>
    </div>
    <p class="line-clamp-5">{series.synopsis}</p>
  </header>
{/snippet}

<div class="grid h-full grid-cols-[2fr_3fr] gap-20">
  <figure class="overflow-hidden pb-8">
    {#if $seriesQuery.isLoading}
      <div class="size-full animate-pulse rounded-xl bg-gray-200"></div>
    {:else if $seriesQuery.isError}
      <div class="size-full rounded-xl bg-gray-700"></div>
    {:else if $seriesQuery.isSuccess}
      <img
        class="size-full rounded-xl object-cover"
        src={$seriesQuery.data.posterUrl}
        alt="{$seriesQuery.data.title} poster"
      />
    {/if}
  </figure>
  <article class="flex flex-col gap-4 overflow-auto">
    {#if $seriesQuery.isLoading}
      {@render seriesHeaderSkeleton()}
    {:else if $seriesQuery.isError}
      {@render seriesErrorState($seriesQuery.error)}
    {:else if $seriesQuery.isSuccess}
      {@render seriesHeader($seriesQuery.data)}
    {/if}
    <EpisodesList {episodesQuery} seriesId={params.seriesId} />
  </article>
</div>
