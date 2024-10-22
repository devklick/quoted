import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import svgr from "vite-plugin-svgr";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd());
  return {
    plugins: [react(), svgr()],

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
