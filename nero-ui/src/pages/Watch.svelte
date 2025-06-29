<script lang="ts">
  import EpisodesList from "../components/EpisodesList.svelte";
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
  <aside class="flex flex-col overflow-y-auto">
    <EpisodesList {episodesQuery} seriesId={params.seriesId} smallCard />
  </aside>
</div>
