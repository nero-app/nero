<script lang="ts">
  import Router from "./components/Router.svelte";
  import Layout from "./layouts/Layout.svelte";
  import HomePage from "./pages/HomePage.svelte";
  import SearchPage from "./pages/SearchPage.svelte";
  import SeriesPage from "./pages/SeriesPage.svelte";
  import SettingsPage from "./pages/SettingsPage.svelte";
  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

  const queryClient = new QueryClient();
  const routes = {
    "/": HomePage,
    "/search": SearchPage,
    "/series/:seriesId": SeriesPage,
    "/settings/:section?": SettingsPage,
  };
</script>

<QueryClientProvider client={queryClient}>
  <Layout>
    <Router {routes} />
  </Layout>
  <SvelteQueryDevtools />
</QueryClientProvider>
