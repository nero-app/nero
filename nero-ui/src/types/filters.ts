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
