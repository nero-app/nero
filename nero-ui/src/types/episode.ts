import type { HttpResource } from "./httpResource";

export interface Episode {
  id: string;
  number: number;
  title?: string;
  thumbnailResource?: HttpResource;
  description?: string;
}
