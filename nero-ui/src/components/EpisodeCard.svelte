<script lang="ts">
  import type { Episode } from "@nero/plugin-extensions";

  interface EpisodeCardProps {
    seriesId: string;
    episode: Episode;
    onclick?: () => void;
  }
  let { episode, onclick }: EpisodeCardProps = $props();
</script>

<button
  class="grid grid-cols-[1fr_4fr_7fr] items-center gap-4 rounded-md p-1
    text-start duration-300 hover:bg-gray-100 active:scale-95"
  {onclick}
>
  <span class="truncate text-center font-medium">{episode.number}</span>
  <picture class="aspect-video min-w-[150px]">
    <!-- TODO: Handle missing poster -->
    <img
      class="size-full rounded-lg object-cover"
      src={episode.thumbnailUrl!}
      alt="Episode thumbnail"
    />
  </picture>
  <div class="min-w-0">
    <h3 class="truncate font-medium">
      {episode.title ?? `Episode ${episode.number}`}
    </h3>
    <p class="line-clamp-3 text-sm text-gray-500">
      {episode.description}
    </p>
  </div>
</button>
