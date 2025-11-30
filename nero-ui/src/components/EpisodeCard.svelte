<script lang="ts">
  import type { Episode } from "@nero/plugin-extensions";

  interface EpisodeCardProps {
    seriesId: string;
    episode: Episode;
    small?: boolean;
    onclick?: () => void;
  }
  let { episode, small, onclick }: EpisodeCardProps = $props();
</script>

<button
  class={`grid items-center gap-4 rounded-md p-1 text-start duration-300 hover:bg-gray-100
  active:scale-95 ${small ? "grid-cols-[1fr_1fr]" : "grid-cols-[1fr_4fr_7fr]"}`}
  {onclick}
>
  {#if !small}
    <span class="truncate text-center font-medium">{episode.number}</span>
  {/if}
  <picture class={`${small ? "min-w-[115px]" : "min-w-[150px]"} aspect-video`}>
    <!-- TODO: Handle missing poster -->
    <img
      class="size-full rounded-lg object-cover"
      src={episode.thumbnailUrl!}
      alt="Episode thumbnail"
    />
  </picture>
  <div class="min-w-0">
    <h3 class="truncate font-medium">
      {#if small}
        Episode {episode.number}
      {:else}
        {episode.title ?? `Episode ${episode.number}`}
      {/if}
    </h3>
    <p
      class={`text-sm text-gray-500 ${small ? "line-clamp-2" : "line-clamp-3"}`}
    >
      {episode.description}
    </p>
  </div>
</button>
