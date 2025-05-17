export interface Video {
  url: string;
  headers: Record<string, string>;
  server: string;
  resolution: [number, number];
}
