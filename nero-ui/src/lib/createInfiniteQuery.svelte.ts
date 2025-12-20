import type { CreateQueryResult, QueryState } from "./createQuery.svelte";

export type InfiniteQueryState<T> = Omit<QueryState<T[]>, "data"> & {
  data: T[];
  hasNextPage: boolean;
  isFetchingNextPage: boolean;
};

export type CreateInfiniteQueryResult<T> = Omit<
  CreateQueryResult<T[]>,
  "data"
> & {
  readonly data: T[];
  readonly hasNextPage: boolean;
  readonly isFetchingNextPage: boolean;
  fetchNextPage: () => Promise<void>;
};

export function createInfiniteQuery<T>(
  queryFn: (page: number) => Promise<{ data: T[]; hasNextPage: boolean }>,
  options?: { initialPage?: number },
): CreateInfiniteQueryResult<T> {
  const initialPage = options?.initialPage ?? 1;

  let state = $state<InfiniteQueryState<T>>({
    data: [],
    status: "idle",
    error: null,
    hasNextPage: false,
    isFetchingNextPage: false,
  });

  let currentPage = $state(initialPage);
  let runId = 0;

  async function fetchPage(page: number, append: boolean) {
    const id = ++runId;

    if (append) {
      state.isFetchingNextPage = true;
    } else {
      state.status = "loading";
    }

    state.error = null;

    try {
      if (id !== runId) return;
      const result = await queryFn(page);

      state.data = append ? [...state.data, ...result.data] : result.data;
      state.hasNextPage = result.hasNextPage;
      state.status = "success";
      currentPage = page;
    } catch (error) {
      if (id !== runId) return;
      state.error = error instanceof Error ? error : new Error(String(error));
      state.status = "error";
    } finally {
      state.isFetchingNextPage = false;
    }
  }

  async function fetchNextPage() {
    if (!state.hasNextPage || state.isFetchingNextPage) return;
    return fetchPage(currentPage + 1, true);
  }

  function refetch() {
    return fetchPage(initialPage, false);
  }

  fetchPage(initialPage, false);

  return {
    get data() {
      return state.data;
    },
    get isLoading() {
      return state.status === "loading";
    },
    get isSuccess() {
      return state.status === "success";
    },
    get isError() {
      return state.status === "error";
    },
    get error() {
      return state.error;
    },
    get hasNextPage() {
      return state.hasNextPage;
    },
    get isFetchingNextPage() {
      return state.isFetchingNextPage;
    },
    fetchNextPage,
    refetch,
  };
}
