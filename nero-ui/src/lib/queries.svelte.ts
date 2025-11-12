import type { Extension, SearchFilter } from "@nero/plugin-extensions";
import { createInfiniteQuery, createQuery } from "@tanstack/svelte-query";

let extension = $state<Extension | null>(null);

export function setExtension(extension: Extension | null) {
  extension = extension;
}

export function createFiltersQuery() {
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
