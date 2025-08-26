export interface HttpResource {
  method: string;
  url: string;
  headers: Array<[string, string]>;
  body?: string;
}
