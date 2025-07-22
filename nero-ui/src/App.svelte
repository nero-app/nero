<script lang="ts">
  import Layout from "./layouts/Layout.svelte";
  import Home from "./pages/Home.svelte";
  import Router from "./pages/Router.svelte";
  import Search from "./pages/Search.svelte";
  import Series from "./pages/Series.svelte";
  import Watch from "./pages/Watch.svelte";
  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

  const queryClient = new QueryClient();
  const routes = {
    "/": Home,
    "/search": Search,
    "/series/:seriesId": Series,
    "/watch/:seriesId/:episodeId": Watch,
  };
</script>

<QueryClientProvider client={queryClient}>
  <Layout>
    <Router {routes} />
  </Layout>
  <SvelteQueryDevtools />
</QueryClientProvider>
