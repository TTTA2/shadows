import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { viteSingleFile } from 'vite-plugin-singlefile';

// https://vitejs.dev/config/
export default defineConfig(async () => ({

  plugins: [sveltekit()],
  // plugins: [sveltekit(), viteSingleFile ()],
  
  // build: {
  //   rollupOptions: {
  //     output: {
  //       inlineDynamicImports: true
  //     }
  //   }
  // },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
