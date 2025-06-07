<script lang="ts">
  import SmallEpisodeCard from "../components/SmallEpisodeCard.svelte";
  import SortIcon from "../components/SortIcon.svelte";
  import {
    getSeriesVideos,
    infiniteEpisodesQuery,
  } from "../state/queries.svelte";

  let { params }: { params: { seriesId: string; episodeId: string } } =
    $props();
  $inspect(params.episodeId);

  let videosQuery = getSeriesVideos(params.seriesId, params.episodeId);
  let episodesQuery = infiniteEpisodesQuery(params.seriesId);
</script>

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <article class="flex flex-col gap-4 overflow-y-auto">
    {#if $videosQuery.isLoading}
      <p>Loading...</p>
    {:else if $videosQuery.isError}
      <p>Error: {$videosQuery.error.message}</p>
    {:else if $videosQuery.isSuccess}
      <!-- For the moment, use the first video -->
      {#if $videosQuery.data.length > 0}
        <!-- TODO: Set a fixed width/height for the video player -->
        <!-- svelte-ignore a11y_media_has_caption -->
        <video controls>
          <!-- TODO: &headers={} ? -->
          <source
            src={`http://localhost:8080/?target=${$videosQuery.data[0].url}`}
            type="video/mp4"
          />
        </video>
      {:else}
        <p>No videos found.</p>
      {/if}
    {/if}
    <section class="flex flex-col gap-2">
      <h1 class="text-2xl font-semibold">Episode title goes here!</h1>
      <p>Episode description...</p>
    </section>
  </article>
  <aside class="overflow-y-auto">
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
                <SmallEpisodeCard seriesId={params.seriesId} {episode} />
              </li>
            {/each}
          {/each}
        </ul>
      {/if}
    </section>
  </aside>
</div>
