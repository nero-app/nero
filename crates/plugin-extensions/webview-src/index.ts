import { invoke } from "@tauri-apps/api/core";

export interface Metadata {
  name?: string;
  producers?: string[];
  authors?: string[];
  description?: string;
  licenses?: string[];
  source?: string;
  homepage?: string;
  revision?: string;
  version?: string;
  range?: string;
  dependencies?: string[];
}

export interface Series {
  id: string;
  title: string;
  posterUrl?: string;
  synopsis?: string;
  type?: string;
}

export interface Episode {
  id: string;
  number: number;
  title?: string;
  thumbnailUrl?: string;
  description?: string;
}

export interface Page<T> {
  items: T[];
  hasNextPage: boolean;
}

export type SeriesPage = Page<Series>;
export type EpisodesPage = Page<Episode>;

export interface FilterCategory {
  id: string;
  displayName: string;
  filters: Filter[];
}

export interface Filter {
  id: string;
  displayName: string;
}

export interface SearchFilter {
  id: string;
  values: string[];
}

export interface Video {
  url: string;
  server: string;
  resolution: [number, number];
}

export class Extension {
  readonly filePath: string;

  private constructor(filePath: string) {
    this.filePath = filePath;
  }

  static async getMetadata(filePath: string): Promise<Metadata> {
    return await invoke("plugin:nero-extensions|get_extension_metadata", {
      filePath,
    });
  }

  static async load(filePath: string): Promise<Extension> {
    await invoke("plugin:nero-extensions|load_extension", { filePath });
    return new Extension(filePath);
  }

  async getFilters(): Promise<FilterCategory[]> {
    return await invoke("plugin:nero-extensions|get_filters");
  }

  async search(
    query: string,
    page?: number,
    filters: SearchFilter[] = [],
  ): Promise<SeriesPage> {
    return await invoke("plugin:nero-extensions|search", {
      query,
      page,
      filters,
    });
  }

  async getSeriesInfo(seriesId: string): Promise<Series> {
    return await invoke("plugin:nero-extensions|get_series_info", {
      seriesId,
    });
  }

  async getSeriesEpisodes(
    seriesId: string,
    page?: number,
  ): Promise<EpisodesPage> {
    return await invoke("plugin:nero-extensions|get_series_episodes", {
      seriesId,
      page,
    });
  }

  async getSeriesVideos(seriesId: string, episodeId: string): Promise<Video[]> {
    return await invoke("plugin:nero-extensions|get_series_videos", {
      seriesId,
      episodeId,
    });
  }
}
