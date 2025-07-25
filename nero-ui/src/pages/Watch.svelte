<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import EpisodesList from "../components/EpisodesList.svelte";
  import ErrorMessage from "../components/ErrorMessage.svelte";
  import {
    createSeriesVideosQuery,
    createInfiniteEpisodesQuery,
  } from "../state/queries.svelte";
  import type { Video } from "../types/video";

  let { params }: { params: { seriesId: string; episodeId: string } } =
    $props();
  $inspect(params.episodeId);

  let videosQuery = createSeriesVideosQuery(params.seriesId, params.episodeId);
  let episodesQuery = createInfiniteEpisodesQuery(params.seriesId);

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

{#snippet videoPlayer(url: string)}
  <!-- svelte-ignore a11y_media_has_caption -->
  <video class="aspect-video" controls>
    <source src={url} type="video/mp4" />
  </video>
{/snippet}

{#snippet videoSection(videos: Video[])}
  {@render videoPlayer(proxyVideoUrl!)}
  {@render videoSelector(videos)}
  <section class="flex flex-col gap-2">
    <h1 class="text-2xl font-semibold">SPY x FAMILY - Episode 1</h1>
    <p class="text-gray-500">
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Esse nulla,
      nesciunt nemo eius doloribus error provident magni cumque, veritatis
      praesentium natus dolores perferendis accusantium illo dignissimos ratione
      similique nam cum.
    </p>
  </section>
{/snippet}

<div class="grid h-full grid-cols-[5fr_2fr] gap-12 overflow-hidden">
  <article class="flex flex-col gap-4 overflow-y-auto">
    {#if $videosQuery.isLoading}
      <p>Loading...</p>
    {:else if $videosQuery.isError}
      <ErrorMessage
        message="Apparently an error has occurred"
        error={$videosQuery.error}
        imageSrc={shockedCat}
      />
    {:else if $videosQuery.isSuccess}
      {@render videoSection($videosQuery.data)}
    {/if}
  </article>
  <aside class="overflow-y-auto">
    <EpisodesList {episodesQuery} seriesId={params.seriesId} smallCards />
  </aside>
</div>
