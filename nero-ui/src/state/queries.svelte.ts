import type { EpisodesPage, SeriesPage } from "../types/page";
import type { SearchFilter } from "../types/searchFilter";
import type { Series } from "../types/series";
import type { Video } from "../types/video";
import { createInfiniteQuery, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export function infiniteSearchQuery(
  query: string,
  initialPage = 1,
  filters: SearchFilter[] = [],
) {
  return createInfiniteQuery({
    queryKey: ["search", query, filters],
    queryFn: ({ pageParam = initialPage }) =>
      invoke<SeriesPage>("search", { query, page: pageParam, filters }),
    initialPageParam: initialPage,
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.hasNextPage) {
        return pages.length + 1;
      }
      return undefined;
    },
  });
}

export function seriesInfoQuery(seriesId: string) {
  return createQuery({
    queryKey: ["series", seriesId],
    queryFn: () => invoke<Series>("get_series_info", { seriesId }),
  });
}

export function infiniteEpisodesQuery(seriesId: string, initialPage = 1) {
  return createInfiniteQuery({
    queryKey: ["episodes", seriesId],
    queryFn: ({ pageParam = initialPage }) =>
      invoke<EpisodesPage>("get_series_episodes", {
        seriesId,
        page: pageParam,
      }),
    initialPageParam: initialPage,
    getNextPageParam: (lastPage, pages) => {
      if (lastPage.hasNextPage) {
        return pages.length + 1;
      }
      return undefined;
    },
  });
}

export function getSeriesVideos(seriesId: string, episodeId: string) {
  return createQuery({
    queryKey: ["videos", seriesId, episodeId],
    queryFn: () =>
      invoke<Video[]>("get_series_videos", { seriesId, episodeId }),
  });
}
