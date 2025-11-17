import { Extension } from "@nero/plugin-extensions";

let extension = $state<Extension | null>(null);

export function getExtension() {
  return extension;
}

export function setExtension(value: Extension | null) {
  extension = value;
}
