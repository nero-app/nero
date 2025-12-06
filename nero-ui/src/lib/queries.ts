import { appState } from "./appState.svelte";
import { Extension, type SearchFilter } from "@nero/plugin-extensions";
import {
  createInfiniteQuery,
  createMutation,
  createQuery,
} from "@tanstack/svelte-query";

export function createExtensionMetadataQuery(filePath: string) {
  return createQuery({
    queryKey: ["extension-metadata", filePath],
    queryFn: () => Extension.getMetadata(filePath),
  });
}

export function createLoadExtensionMutation() {
  return createMutation({
    mutationFn: async (filePath: string) => {
      const loadedExtension = await Extension.load(filePath);
      appState.extension = loadedExtension;
      return loadedExtension;
    },
  });
}

export function createFiltersQuery() {
  const extension = appState.extension;
  return createQuery({
    queryKey: ["filters", extension?.filePath],
    queryFn: () => {
      if (!extension) {
        throw new Error("No extension loaded");
      }
      return extension.getFilters();
    },
    enabled: !!extension,
  });
}

export function createInfiniteSearchQuery(
  query: string,
  initialPage = 1,
  filters: SearchFilter[] = [],
) {
  const extension = appState.extension;
  return createInfiniteQuery({
    queryKey: ["search", extension?.filePath, query, filters],
    queryFn: ({ pageParam = initialPage }) => {
      if (!extension) {
        throw new Error("No extension loaded");
      }
      return extension.search(query, pageParam, filters);
    },
    initialPageParam: initialPage,
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.hasNextPage) {
        return pages.length + 1;
      }
      return undefined;
    },
    enabled: !!extension,
  });
}

export function createSeriesInfoQuery(seriesId: string) {
  const extension = appState.extension;
  return createQuery({
    queryKey: ["series", extension?.filePath, seriesId],
    queryFn: () => {
      if (!extension) {
        throw new Error("No extension loaded");
      }
      return extension.getSeriesInfo(seriesId);
    },
    enabled: !!extension,
  });
}

export function createInfiniteEpisodesQuery(seriesId: string, initialPage = 1) {
  const extension = appState.extension;
  return createInfiniteQuery({
    queryKey: ["episodes", extension?.filePath, seriesId],
    queryFn: ({ pageParam = initialPage }) => {
      if (!extension) {
        throw new Error("No extension loaded");
      }
      return extension.getSeriesEpisodes(seriesId, pageParam);
    },
    initialPageParam: initialPage,
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.hasNextPage) {
        return pages.length + 1;
      }
      return undefined;
    },
    enabled: !!extension,
  });
}

export function createSeriesVideosQuery(seriesId: string, episodeId: string) {
  const extension = appState.extension;
  return createQuery({
    queryKey: ["videos", extension?.filePath, seriesId, episodeId],
    queryFn: () => {
      if (!extension) {
        throw new Error("No extension loaded");
      }
      return extension.getSeriesVideos(seriesId, episodeId);
    },
    enabled: !!extension,
  });
}
