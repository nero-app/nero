<script module lang="ts">
  import { tick } from "svelte";

  /**
   * Represents a navigation location.
   * @property {string} location - Location (page/view), for example `/book`
   * @property {string} [querystring] - Querystring from the hash, as a string not parsed
   */
  interface Location {
    location: string;
    querystring?: string;
  }

  /**
   * Returns the current location from the hash.
   *
   * @returns {Location} Location object
   * @private
   */
  function getLocation(): Location {
    const hash = window.location.hash;
    const fullPath = hash.startsWith("#/") ? hash.slice(1) : "/";

    const [path, querystring = ""] = fullPath.split("?");

    return {
      location: path || "/",
      ...(querystring && { querystring }),
    };
  }

  /**
   * Navigates to a new page programmatically.
   *
   * @param {string} location - Path to navigate to (must start with `/` or '#/')
   * @return {Promise<void>} Promise that resolves after the page navigation has completed
   */
  export async function push(location: string): Promise<void> {
    if (
      location.length < 1 ||
      !(location.startsWith("/") || location.startsWith("#/"))
    ) {
      throw Error("Invalid parameter location");
    }

    await tick();
    window.location.hash = location.startsWith("#") ? location : `#${location}`;
  }

  /**
   * Svelte Action that enables a link element (`<a>`) to use our history management.
   *
   * For example:
   *
   * ````html
   * <a href="/books" use:link>View books</a>
   * ````
   *
   * @param {HTMLElement} node - The target node (automatically set by Svelte). Must be an anchor tag (`<a>`) with a href attribute starting in `/`
   */
  export function link(node: HTMLElement) {
    if (node.tagName.toLowerCase() != "a") {
      throw Error('Action "link" can only be used with <a> tags');
    }

    const href = node.getAttribute("href");
    if (href?.startsWith("/")) {
      node.setAttribute("href", `#${href}`);
    } else if (!href?.startsWith("#/")) {
      throw Error(`Invalid href: "${href}". Must start with "/" or "#/"`);
    }
  }
</script>

<script lang="ts">
  import { parse } from "regexparam";
  import type { Component as ComponentType } from "svelte";

  type Props = {
    /**
     * Dictionary of all routes, in the format `'/path': component`.
     *
     * For example:
     * ````js
     * import HomeRoute from './routes/HomeRoute.svelte'
     * import BooksRoute from './routes/BooksRoute.svelte'
     * import NotFoundRoute from './routes/NotFoundRoute.svelte'
     * routes = {
     *     '/': HomeRoute,
     *     '/books': BooksRoute,
     *     '*': NotFoundRoute
     * }
     * ````
     */
    routes: Record<string, ComponentType<any>>;
  };
  let { routes }: Props = $props();

  /**
   * Initializes the object and creates a regular expression from the path, using regexparam.
   *
   * @param {string} path - Path to the route (must start with '/' or '*')
   * @param {ComponentType} component - Svelte component for the route.
   */
  function createRoute(path: string, component: ComponentType) {
    if (path.length < 1 || (path[0] != "/" && path[0] != "*")) {
      throw Error(
        'Invalid value for "path" argument - strings must start with / or *',
      );
    }
    const { pattern, keys } = parse(path);
    return { component, pattern, keys };
  }

  /**
   * Checks if `path` matches the current route.
   * If there's a match, will return the list of parameters from the URL (if any).
   * In case of no match, the method will return `null`.
   *
   * @param {Object} route - Route object
   * @param {string} path - Path to test
   * @returns {null|Object.<string, string>} List of paramters from the URL if there's a match, or `null` otherwise.
   */
  function matchRoute(
    route: { pattern: RegExp; keys: string[] },
    path: string,
  ): Record<string, string> | null {
    const matches = route.pattern.exec(path);
    if (!matches) return null;

    return route.keys.reduce(
      (params, key, index) => {
        try {
          params[key] = decodeURIComponent(matches[index + 1] || "");
        } catch {
          params[key] = matches[index + 1] || "";
        }
        return params;
      },
      {} as Record<string, string>,
    );
  }

  const routesList = $derived(
    Object.entries(routes).map(([path, component]) =>
      createRoute(path, component),
    ),
  );

  let currentLocation = $state(getLocation());

  $effect(() => {
    const updateLocation = () => (currentLocation = getLocation());
    window.addEventListener("hashchange", updateLocation);
    return () => window.removeEventListener("hashchange", updateLocation);
  });

  let { Component, routeParams, querystring } = $derived.by(() => {
    let { location } = currentLocation;
    for (const route of routesList) {
      const match = matchRoute(route, location);
      if (match) {
        return {
          Component: route.component,
          routeParams: match,
          querystring: currentLocation.querystring || null,
        };
      }
    }
    return {
      Component: null,
      routeParams: null,
      querystring: null,
    };
  });
</script>

{#if Component}
  <Component
    {...routeParams && { params: routeParams }}
    {...querystring && { querystring }}
  />
{/if}
