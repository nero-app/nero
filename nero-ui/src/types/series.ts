import type { HttpResource } from "./httpResource";

export interface Series {
  id: string;
  title: string;
  posterResource?: HttpResource;
  synopsis?: string;
  type?: string;
}
