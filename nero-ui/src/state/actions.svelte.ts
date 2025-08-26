import type { HttpResource } from "../types/httpResource";
import { createResourceUrlQuery } from "./queries.svelte";

export function httpResource(node: HTMLElement, resource: HttpResource) {
  if (!("src" in node)) {
    throw new Error(
      'Action "httpResource" can only be used on elements with a "src" attribute',
    );
  }

  let currentResource = $state(resource);
  let query = $derived(createResourceUrlQuery(currentResource));

  $effect(() => {
    const unsubscribe = query.subscribe((result) => {
      if (result.status === "success" && result.data) {
        node.src = result.data;
      } else if (result.status === "error") {
        console.error(
          "An error has occurred while loading the resource",
          result.error,
        );
      }
    });

    return () => unsubscribe();
  });

  return {
    update(newResource: HttpResource) {
      currentResource = newResource;
    },
  };
}
