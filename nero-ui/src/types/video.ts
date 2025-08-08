import type { HttpResource } from "./httpResource";

export interface Video {
  httpResource: HttpResource;
  server: string;
  resolution: [number, number];
}
