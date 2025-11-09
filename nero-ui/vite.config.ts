import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  server: {
    port: 1420,
    strictPort: true,
  },
  cacheDir: "../node_modules/.vite",
});
