<script lang="ts">
  import shockedCat from "../assets/images/shocked_cat.svg";
  import SearchIcon from "../components/icons/SearchIcon.svelte";

  function focusSearchInput() {
    document.getElementById("search-input")!.focus();
  }
</script>

{#snippet emptyFeedback()}
  <article class="flex flex-col items-center justify-center overflow-auto pb-8">
    <img class="w-64" src={shockedCat} alt="Shocked cat" />
    <p class="pb-2 text-center">
      Whoops...
      <br />
      Apparently there's nothing around here.
    </p>
    <button
      class="cursor-pointer rounded-lg bg-orange-200 px-3 py-1.5 duration-300 active:scale-95"
      onclick={focusSearchInput}
    >
      <div class="flex items-center gap-2">
        <SearchIcon />
        <span>Search for series</span>
      </div>
    </button>
  </article>
{/snippet}

<div class="grid h-full grid-cols-[2fr_auto_3fr]">
  <figure class="overflow-hidden pb-8">
    <div class="size-full rounded-xl bg-gray-700"></div>
  </figure>
  <div class="flex w-20 items-center pb-8">
    <p class="rotate-90">Indicators...</p>
  </div>
  {@render emptyFeedback()}
</div>
