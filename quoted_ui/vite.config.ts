import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import svgr from "vite-plugin-svgr";
import path from "path";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd());
  return {
    plugins: [react(), svgr()],
    resolve: {
      alias: {
        "~": path.resolve(__dirname, "./src"),
        // See https://github.com/tabler/tabler-icons/issues/1233#issuecomment-2428245119
        // /esm/icons/index.mjs only exports the icons statically, so no separate chunks are created
        "@tabler/icons-react": "@tabler/icons-react/dist/esm/icons/index.mjs",
      },
    },
    server: {
      cors: true,
      proxy: {
        "/api": {
          target:
            env.VITE_BASE_API_URL || "https://devklick-quoted-api.vercel.app",
          changeOrigin: true,
        },
      },
    },
  };
});
