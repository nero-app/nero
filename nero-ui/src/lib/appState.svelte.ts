import { Extension } from "@nero/plugin-extensions";

export interface AppConfig {
  playerPath: string | null;
  allowUntrustedExtensions: boolean;
  extensionPath: string | null;
}

class AppState {
  config = $state<AppConfig>({
    playerPath: null,
    allowUntrustedExtensions: true,
    extensionPath: null,
  });

  extension = $state<Extension | null>(null);
}

export const appState = new AppState();
