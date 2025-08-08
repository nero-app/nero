export interface HttpResource {
  method: string;
  url: string;
  headers: Array<[string, string]>;
  body?: string;
}

function encodeToBase64(str: string): string {
  const bytes = new TextEncoder().encode(str);
  const binString = Array.from(bytes, (byte) =>
    String.fromCodePoint(byte),
  ).join("");
  return btoa(binString);
}

function httpResourceToUrl(resource: HttpResource): string {
  const params = new URLSearchParams();
  params.set("method", resource.method);
  params.set("url", resource.url);

  if (resource.headers?.length) {
    const headersEncoded = encodeToBase64(JSON.stringify(resource.headers));
    params.set("headers", headersEncoded);
  }

  if (resource.body) {
    const bodyEncoded = encodeToBase64(resource.body);
    params.set("body", bodyEncoded);
  }

  // TODO: Tauri command to obtain the actual URL
  return `http://localhost:8080/?${params.toString()}`;
}

export function httpResource(node: HTMLElement, resource: HttpResource) {
  if (!("src" in node)) {
    throw new Error(
      'Action "httpResource" can only be used on elements with a "src" attribute',
    );
  }

  node.src = httpResourceToUrl(resource);

  return {
    update(newResource: HttpResource) {
      node.src = httpResourceToUrl(newResource);
    },
  };
}
