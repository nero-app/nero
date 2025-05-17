<script lang="ts">
  import EmptyFeedback from "../components/EmptyFeedback.svelte";
</script>

<div class="grid h-full grid-cols-[2fr_auto_3fr]">
  <figure class="overflow-hidden pb-8">
    <div class="size-full rounded-xl bg-gray-700"></div>
  </figure>
  <div class="flex w-20 items-center pb-8">
    <p class="rotate-90">Indicators...</p>
  </div>
  <EmptyFeedback />
</div>
