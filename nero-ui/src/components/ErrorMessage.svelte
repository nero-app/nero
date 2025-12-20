<script lang="ts">
  import type { Snippet } from "svelte";

  interface ErrorStateProps {
    message: string;
    imageSrc?: string;
    error: Error;
    children?: Snippet;
  }
  let { message, imageSrc, error, children }: ErrorStateProps = $props();
</script>

<article class="flex size-full flex-col items-center justify-center gap-2">
  <p class="text-center">
    Whoops...
    <br />
    {message}
  </p>
  <div class="flex w-full items-center justify-center">
    {#if imageSrc}
      <img class="w-56" src={imageSrc} alt="Error illustration" />
    {/if}
    <pre
      class="max-h-64 overflow-auto rounded-md border border-gray-300
        bg-gray-100 p-4 text-sm wrap-break-word whitespace-pre-wrap
        text-gray-800">{error}</pre>
  </div>
  {@render children?.()}
</article>
