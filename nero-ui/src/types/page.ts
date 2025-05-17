import type { Episode } from "./episode";
import type { Series } from "./series";

export interface Page<T> {
  items: T[];
  hasNextPage: boolean;
}

export type SeriesPage = Page<Series>;

export type EpisodesPage = Page<Episode>;
