<script lang="ts">
  import SmallEpisodeCard from "../components/SmallEpisodeCard.svelte";
  import SortIcon from "../components/SortIcon.svelte";
  import { infiniteEpisodesQuery } from "../state/queries.svelte";

  let { params }: { params: { seriesId: string; episodeId: string } } =
    $props();
  $inspect(params.episodeId);

  let episodesQuery = infiniteEpisodesQuery(params.seriesId);
</script>

<div class="grid h-full grid-cols-[4fr_2fr] gap-12 overflow-hidden">
  <article class="flex flex-col gap-4 overflow-y-auto">
    <!-- TODO: Set a fixed width/height for the video player -->
    <!-- svelte-ignore a11y_media_has_caption -->
    <video controls>
      <source
        src="http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
        type="video/mp4"
      />
    </video>
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
