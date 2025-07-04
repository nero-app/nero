<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import EpisodesList from "../components/EpisodesList.svelte";
  import {
    getSeriesVideos,
    infiniteEpisodesQuery,
  } from "../state/queries.svelte";
  import { type Video } from "../types/video";

  let { params }: { params: { seriesId: string; episodeId: string } } =
    $props();
  $inspect(params.episodeId);

  let videosQuery = getSeriesVideos(params.seriesId, params.episodeId);
  let episodesQuery = infiniteEpisodesQuery(params.seriesId);

  let selectedVideoIndex = $state(0);
  let proxyVideoUrl = $derived.by(() => {
    if (
      !$videosQuery.isSuccess ||
      !$videosQuery.data ||
      $videosQuery.data.length === 0
    ) {
      return null;
    }
    const selectedVideo = $videosQuery.data[selectedVideoIndex];
    if (!selectedVideo) {
      return null;
    }

    // TODO: Tauri command to get the proxy URL?
    const baseUrl = "http://localhost:8080/";
    const params = new URLSearchParams();
    params.append("target", selectedVideo.url);
    if (
      selectedVideo.headers &&
      Object.keys(selectedVideo.headers).length > 0
    ) {
      const headersString = Object.entries(selectedVideo.headers)
        .map(([key, value]) => `${key}:${value}`)
        .join("|");
      params.append("headers", headersString);
    }
    return `${baseUrl}?${params.toString()}`;
  });
</script>

{#snippet errorState(error: Error)}
  <article class="flex size-full flex-col items-center justify-center gap-2">
    <p class="text-center">
      Whoops...
      <br />
      Apparently an error has occurred.
    </p>
    <div class="flex items-center">
      <img class="w-56" src={shockedCat} alt="Shocked cat" />
      <pre
        class="max-h-56 overflow-auto whitespace-pre-wrap break-words rounded-md border
          border-gray-300 bg-gray-100 p-4 text-sm text-gray-800">{error.message}</pre>
    </div>
  </article>
{/snippet}

<!-- TODO: Delete this when the custom video player is created -->
{#snippet videoSelector(videos: Video[])}
  <select class="rounded-md border" bind:value={selectedVideoIndex}>
    {#each videos as video, index}
      <option value={index}>
        {video.server} - {video.resolution[0]}x{video.resolution[1]}
      </option>
    {/each}
  </select>
{/snippet}

<!-- TODO: Set a fixed width/height for the video player -->
{#snippet videoPlayer(url: string)}
  <!-- svelte-ignore a11y_media_has_caption -->
  <video controls>
    <source src={url} type="video/mp4" />
  </video>
{/snippet}

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <article class="flex flex-col gap-4 overflow-y-auto">
    {#if $videosQuery.isLoading}
      <p>Loading...</p>
    {:else if $videosQuery.isError}
      {@render errorState($videosQuery.error)}
    {:else if $videosQuery.isSuccess}
      {#if $videosQuery.data.length > 0}
        {@render videoPlayer(proxyVideoUrl!)}
        {@render videoSelector($videosQuery.data)}
        <section class="flex flex-col gap-2">
          <h1 class="text-2xl font-semibold">Episode title goes here!</h1>
          <p>Episode description...</p>
        </section>
      {:else}
        <p>No videos found.</p>
      {/if}
    {/if}
  </article>
  <aside class="flex flex-col overflow-y-auto">
    <EpisodesList {episodesQuery} seriesId={params.seriesId} smallCard />
  </aside>
</div>
