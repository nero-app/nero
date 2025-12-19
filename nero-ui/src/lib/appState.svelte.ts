import { Extension } from "@nero/plugin-extensions";

export interface AppConfig {
  playerPath: string | null;
  allowUntrustedExtensions: boolean;
  extensionPath: string | null;

  nodeEnabled: boolean;
  nodeConfigType: "auto" | "custom";
  nodeCustomPath: string;
  npmCustomPath: string;
  nodeAutoDownload: boolean;

  torrentConfigType: "auto" | "custom";
  torrentCustomPath: string;
  torrentAutoDownload: boolean;
}

class AppState {
  config = $state<AppConfig>({
    playerPath: null,
    allowUntrustedExtensions: true,
    extensionPath: null,

    nodeEnabled: false,
    nodeConfigType: "auto",
    nodeCustomPath: "",
    npmCustomPath: "",
    nodeAutoDownload: false,

    torrentConfigType: "auto",
    torrentCustomPath: "",
    torrentAutoDownload: false,
  });

  extension = $state<Extension | null>(null);
}

export const appState = new AppState();
