export type Status = "idle" | "loading" | "success" | "error";

export type QueryState<T> = {
  data: T | null;
  status: Status;
  error: Error | null;
};

export type CreateQueryResult<T> = {
  readonly data: T | null;
  readonly isLoading: boolean;
  readonly isError: boolean;
  readonly error: Error | null;
  readonly isSuccess: boolean;
  refetch: () => Promise<void>;
};

export function createQuery<T>(
  queryFn: () => Promise<T>,
): CreateQueryResult<T> {
  const state = $state<QueryState<T>>({
    data: null,
    status: "idle",
    error: null,
  });

  let runId = 0;

  async function fetch() {
    const id = ++runId;

    state.status = "loading";
    state.error = null;

    try {
      const result = await queryFn();
      if (id !== runId) return;

      state.data = result;
      state.status = "success";
    } catch (error) {
      if (id !== runId) return;

      state.status = "error";
      state.error = error instanceof Error ? error : new Error(String(error));
    }
  }

  function refetch() {
    return fetch();
  }

  fetch();

  return {
    get data() {
      return state.data;
    },
    get isLoading() {
      return state.status === "loading";
    },
    get isError() {
      return state.status === "error";
    },
    get error() {
      return state.error;
    },
    get isSuccess() {
      return state.status === "success";
    },
    refetch,
  };
}
