<script lang="ts">
  import Router from "./components/Router.svelte";
  import Layout from "./layouts/Layout.svelte";
  import HomePage from "./pages/HomePage.svelte";
  import SearchPage from "./pages/SearchPage.svelte";
  import SeriesPage from "./pages/SeriesPage.svelte";
  import SettingsPage from "./pages/SettingsPage.svelte";

  const routes = {
    "/": HomePage,
    "/search": SearchPage,
    "/series/:seriesId": SeriesPage,
    "/settings/:section?": SettingsPage,
  };
</script>

<Layout>
  <Router {routes} />
</Layout>
