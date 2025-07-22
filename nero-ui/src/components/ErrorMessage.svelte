<script lang="ts">
  import type { Snippet } from "svelte";

  interface ErrorStateProps {
    message: string;
    imageSrc?: string;
    error: Error;
    centered?: boolean;
    children?: Snippet;
  }
  let { message, imageSrc, error, centered, children }: ErrorStateProps =
    $props();
</script>

<article
  class={`flex flex-col gap-2 ${centered ? "size-full items-center justify-center" : ""}`}
>
  <p class="text-center">
    Whoops...
    <br />
    {message}
  </p>
  <div class="flex items-center">
    {#if imageSrc}
      <img class="w-56" src={imageSrc} alt="Error illustration" />
    {/if}
    <pre
      class="max-h-56 overflow-auto whitespace-pre-wrap break-words rounded-md border
        border-gray-300 bg-gray-100 p-4 text-sm text-gray-800">{error.message}</pre>
  </div>
  {@render children?.()}
</article>
