import { appState } from "./appState.svelte";
import { createInfiniteQuery } from "./createInfiniteQuery.svelte";
import { createMutation } from "./createMutation.svelte";
import { createQuery } from "./createQuery.svelte";
import { Extension, type SearchFilter } from "@nero/plugin-extensions";

export function createExtensionMetadataQuery(filePath: string) {
  return createQuery(() => Extension.getMetadata(filePath));
}

export function createLoadExtensionMutation() {
  return createMutation(async (filePath: string) => {
    const loadedExtension = await Extension.load(filePath);
    appState.extension = loadedExtension;
    return loadedExtension;
  });
}

export function createFiltersQuery() {
  const extension = appState.extension;
  return createQuery(() => {
    if (!extension) throw new Error("No extension loaded");
    return extension.getFilters();
  });
}

export function createInfiniteSearchQuery(
  query: string,
  filters: SearchFilter[] = [],
) {
  const extension = appState.extension;
  return createInfiniteQuery(async (page) => {
    if (!extension) throw new Error("No extension loaded");
    const result = await extension.search(query, page, filters);
    return {
      data: result.items,
      hasNextPage: result.hasNextPage,
    };
  });
}

export function createSeriesInfoQuery(seriesId: string) {
  const extension = appState.extension;
  return createQuery(() => {
    if (!extension) throw new Error("No extension loaded");
    return extension.getSeriesInfo(seriesId);
  });
}

export function createInfiniteEpisodesQuery(seriesId: string) {
  const extension = appState.extension;
  return createInfiniteQuery(async (page) => {
    if (!extension) throw new Error("No extension loaded");
    const result = await extension.getSeriesEpisodes(seriesId, page);
    return {
      data: result.items,
      hasNextPage: result.hasNextPage,
    };
  });
}

export function createSeriesVideosQuery(seriesId: string, episodeId: string) {
  const extension = appState.extension;
  return createQuery(() => {
    if (!extension) throw new Error("No extension loaded");
    return extension.getSeriesVideos(seriesId, episodeId);
  });
}
