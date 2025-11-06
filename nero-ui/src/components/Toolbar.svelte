<script lang="ts">
  import searchIcon from "../assets/icons/search_icon.svg";
  import { link, push } from "../pages/Router.svelte";

  let searchQuery = $state("");
  function handleSearch(event: Event) {
    event.preventDefault();
    if (searchQuery) {
      push(`/search?q=${encodeURIComponent(searchQuery)}`);
    }
  }
</script>

<nav class="grid w-full grid-cols-[1fr_1fr_1fr]">
  <p>Nero app v1.0</p>
  <ul class="flex gap-4">
    <li>
      <a href="/" use:link>Home</a>
    </li>
    <li>
      <a href="/settings/extensions" use:link>Extensions</a>
    </li>
    <li>
      <a href="/settings" use:link>Settings</a>
    </li>
  </ul>
  <form
    class="flex items-center gap-2 rounded-lg bg-slate-100 px-3 py-1"
    onsubmit={handleSearch}
  >
    <img src={searchIcon} alt="Search icon" />
    <input
      class="w-full bg-slate-100 text-slate-400 outline-none"
      id="search-input"
      type="search"
      placeholder="Search"
      bind:value={searchQuery}
    />
  </form>
</nav>
