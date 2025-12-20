<script lang="ts">
  import { appState } from "../lib/appState.svelte";
  import { createSeriesVideosQuery } from "../lib/queries";
  import ErrorMessage from "./ErrorMessage.svelte";
  import type { Episode, Video } from "@nero/plugin-extensions";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface VideoSelectionModalProps {
    seriesId: string;
    episode: Episode;
    onClose: (selected?: Video) => void;
  }

  let { seriesId, episode, onClose }: VideoSelectionModalProps = $props();

  const videosQuery = $derived(createSeriesVideosQuery(seriesId, episode.id));

  let dialogElement: HTMLDialogElement;
  let scrollProgress = $state(0);

  onMount(() => {
    if (dialogElement && !dialogElement.open) {
      dialogElement.showModal();
      dialogElement.focus();
    }
  });

  function handleScroll(e: Event) {
    const target = e.target as HTMLDivElement;
    const maxScroll = 150;
    scrollProgress = Math.min(target.scrollTop / maxScroll, 1);
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === dialogElement) close();
  }

  function handleCancel(e: Event) {
    e.preventDefault();
    close();
  }

  async function selectVideo(video: Video) {
    const playerPath = appState.config.playerPath;

    if (!playerPath) {
      alert(
        "No video player configured. Please set a video player path in settings.",
      );
      return;
    }

    try {
      await invoke("open_video_player", {
        playerPath,
        url: video.url,
      });
      close(video);
    } catch (error) {
      alert(`Error opening video player: ${error}`);
    }
  }

  function close(selected?: Video) {
    try {
      dialogElement.close();
    } catch {}
    onClose && onClose(selected);
  }
</script>

{#snippet videoCard(video: Video, index: number)}
  <button
    onclick={() => selectVideo(video)}
    class="flex w-full items-center gap-3 rounded-lg border border-neutral-200 px-4 py-3
      text-left transition-colors hover:bg-neutral-50"
  >
    <span
      class="flex h-9 w-9 shrink-0 items-center justify-center rounded-md bg-neutral-900
        text-sm font-semibold text-white"
    >
      {index + 1}
    </span>
    <div class="min-w-0 flex-1">
      <p class="text-sm font-medium text-neutral-900">
        {video.server}
      </p>
      {#if video.resolution}
        <p class="text-xs text-neutral-500">{video.resolution}</p>
      {/if}
    </div>
  </button>
{/snippet}

{#snippet loadingSpinner()}
  <section
    class="flex size-full max-h-[50vh] flex-col items-center justify-center gap-4"
  >
    <div
      class="size-12 animate-spin rounded-full border-4 border-neutral-200
        border-t-neutral-900"
    ></div>
    <p class="text-sm text-neutral-600">Loading videos...</p>
  </section>
{/snippet}

{#snippet videosList(videos: Video[])}
  <section class="p-4">
    <h2 class="mb-3 text-sm font-medium text-neutral-900">Available servers</h2>
    <ul class="space-y-2" role="list">
      {#each videos as video, i}
        <li>
          {@render videoCard(video, i)}
        </li>
      {/each}
    </ul>
  </section>
{/snippet}

<dialog
  bind:this={dialogElement}
  class="m-auto size-full max-h-[90vh] max-w-100 overflow-y-auto rounded-lg
    border border-neutral-200 shadow-xl backdrop:bg-black/40
    backdrop:backdrop-blur-sm"
  onclick={handleBackdropClick}
  oncancel={handleCancel}
  onscroll={handleScroll}
>
  <figure class="relative aspect-video">
    <img
      src={episode.thumbnailUrl}
      alt={episode.title || `Episode ${episode.number}`}
      class="size-full object-cover"
      style="opacity: {1 - scrollProgress}"
    />
    <div
      class="absolute inset-0 bg-linear-to-t from-black/70 to-transparent"
      style="opacity: {1 - scrollProgress}"
    ></div>
  </figure>

  <header
    class="sticky top-0 z-10 border-b border-neutral-200 px-4 py-3
      transition-all duration-200"
    style="
			margin-top: -60px;
			background-color: rgba(255, 255, 255, {scrollProgress});
			backdrop-filter: blur({scrollProgress * 8}px);
		"
  >
    <p
      class="text-xs font-medium transition-colors duration-200"
      style="color: {scrollProgress > 0.5
        ? 'rgb(115, 115, 115)'
        : 'rgba(255, 255, 255, 0.9)'}"
    >
      Episode {episode.number}
    </p>
    <h3
      class="truncate text-sm font-semibold transition-colors duration-200"
      style="color: {scrollProgress > 0.5 ? 'rgb(23, 23, 23)' : 'white'}"
    >
      {episode.title || `Episode ${episode.number}`}
    </h3>
  </header>

  {#if videosQuery.isLoading}
    {@render loadingSpinner()}
  {:else if videosQuery.error}
    <section class="size-full max-h-[50vh]">
      <ErrorMessage
        message="Apparently an error has occurred"
        error={videosQuery.error}
      />
    </section>
  {:else if videosQuery.data}
    {@render videosList(videosQuery.data)}
  {/if}
</dialog>
