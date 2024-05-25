import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import eslintPlugin from 'vite-plugin-eslint';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    eslintPlugin() // Add this line to enable ESLint checking during build/development
  ],
});
