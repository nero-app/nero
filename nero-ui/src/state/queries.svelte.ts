import type { FilterCategory, SearchFilter } from "../types/filters";
import type { HttpResource } from "../types/httpResource";
import type { EpisodesPage, SeriesPage } from "../types/page";
import type { Series } from "../types/series";
import type { Video } from "../types/video";
import { createInfiniteQuery, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export function createResourceUrlQuery(resource: HttpResource) {
  return createQuery({
    queryKey: ["httpResource", resource.url.toString()],
    queryFn: () =>
      invoke<string>("plugin:http-resources|resolve_resource", {
        resource,
      }),
  });
}

export function createFiltersQuery() {
  return createQuery({
    queryKey: ["filters"],
    queryFn: () => invoke<FilterCategory[]>("get_filters"),
  });
}

export function createInfiniteSearchQuery(
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

export function createSeriesInfoQuery(seriesId: string) {
  return createQuery({
    queryKey: ["series", seriesId],
    queryFn: () => invoke<Series>("get_series_info", { seriesId }),
  });
}

export function createInfiniteEpisodesQuery(seriesId: string, initialPage = 1) {
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

export function createSeriesVideosQuery(seriesId: string, episodeId: string) {
  return createQuery({
    queryKey: ["videos", seriesId, episodeId],
    queryFn: () =>
      invoke<Video[]>("get_series_videos", { seriesId, episodeId }),
  });
}
