<script lang="ts">
  import EpisodeCard from "../components/EpisodeCard.svelte";
  import PlayIcon from "../components/PlayIcon.svelte";
  import ShareIcon from "../components/ShareIcon.svelte";
  import SortIcon from "../components/SortIcon.svelte";
  import {
    infiniteEpisodesQuery,
    seriesInfoQuery,
  } from "../state/queries.svelte";
  import { link } from "./Router.svelte";

  let { params }: { params: { seriesId: string } } = $props();

  let seriesQuery = seriesInfoQuery(params.seriesId);
  let episodesQuery = infiniteEpisodesQuery(params.seriesId);

  // TODO:
  const firstEpisodeId = 0;
</script>

{#if $seriesQuery.isLoading}
  <p>Loading...</p>
{:else if $seriesQuery.isError}
  <p>Error: {$seriesQuery.error.message}</p>
{:else if $seriesQuery.isSuccess}
  <div class="grid h-full grid-cols-[2fr_3fr] gap-20">
    <figure class="overflow-hidden pb-8">
      <img
        class="size-full rounded-xl object-cover"
        src={$seriesQuery.data.posterUrl}
        alt="{$seriesQuery.data.title} poster"
      />
    </figure>
    <article class="flex flex-col gap-4 overflow-auto">
      <header class="flex flex-col gap-4">
        <h1 class="truncate text-3xl font-bold">{$seriesQuery.data.title}</h1>
        <div class="flex gap-4">
          <!-- TODO: Disable if episodes are not available or if the page does not contain any episodes -->
          <a
            class="rounded-lg bg-red-300 px-3 py-1.5 duration-300 active:scale-95"
            href="/watch/{$seriesQuery.data.id}/{firstEpisodeId}"
            use:link
          >
            <div class="flex items-center gap-2">
              <PlayIcon />
              <span>Watch now</span>
            </div>
          </a>
          <!-- TODO: onclick -->
          <button
            class="cursor-pointer rounded-lg bg-red-300 px-3 py-1.5 duration-300 active:scale-95"
          >
            <div class="flex items-center gap-2">
              <ShareIcon />
              <span>Share the series</span>
            </div>
          </button>
        </div>
        <p class="line-clamp-5">{$seriesQuery.data.synopsis}</p>
      </header>
      <section>
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
        {#if $episodesQuery.isLoading}
          <p>Loading...</p>
        {:else if $episodesQuery.isError}
          <p>Error: {$episodesQuery.error.message}</p>
        {:else if $episodesQuery.isSuccess}
          <ul>
            {#each $episodesQuery.data.pages as page, pageIndex (pageIndex)}
              {#each page.items as episode (episode.id)}
                <li>
                  <EpisodeCard seriesId={params.seriesId} {episode} />
                </li>
              {/each}
            {/each}
          </ul>
        {/if}
      </section>
    </article>
  </div>
{/if}
