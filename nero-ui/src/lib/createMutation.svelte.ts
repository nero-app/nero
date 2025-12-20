import type { CreateQueryResult, QueryState } from "./createQuery.svelte";

export type CreateMutationResult<TData, TVariables> = Omit<
  CreateQueryResult<TData>,
  "refetch"
> & {
  mutate: (variables: TVariables) => Promise<TData | null>;
};

export function createMutation<TData, TVariables>(
  mutationFn: (variables: TVariables) => Promise<TData>,
): CreateMutationResult<TData, TVariables> {
  let state = $state<QueryState<TData>>({
    data: null,
    status: "idle",
    error: null,
  });

  async function mutate(variables: TVariables): Promise<TData | null> {
    state.status = "loading";
    state.error = null;

    try {
      const result = await mutationFn(variables);
      state.data = result;
      state.status = "success";
      return result;
    } catch (error) {
      state.status = "error";
      state.error = error instanceof Error ? error : new Error(String(error));
      return null;
    }
  }

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
    mutate,
  };
}
