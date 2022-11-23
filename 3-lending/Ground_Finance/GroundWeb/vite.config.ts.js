// vite.config.ts
import react from "@vitejs/plugin-react";
import { getThemeVariables } from "antd/dist/theme.js";
import autoImport from "unplugin-auto-import/vite";
import { defineConfig } from "vite";
import vitePluginImp from "vite-plugin-imp";
import tsconfigPaths from "vite-tsconfig-paths";
var vite_config_default = defineConfig({
  base: "/vite-react/",
  plugins: [
    react({
      babel: {
        parserOpts: {
          plugins: ["decorators-legacy"]
        }
      }
    }),
    tsconfigPaths(),
    autoImport({
      imports: [
        "react",
        {
          react: [
            "useImperativeHandle",
            "createElement",
            "cloneElement",
            "createContext",
            "useLayoutEffect",
            "forwardRef"
          ]
        }
      ]
    }),
    vitePluginImp({
      libList: [
        {
          libName: "antd",
          style: (name) => `antd/es/${name}/style`
        }
      ]
    })
  ],
  server: {
    port: 8e3
  },
  resolve: {
    alias: [
      { find: /^~/, replacement: "" }
    ]
  },
  css: {
    modules: {
      localsConvention: "camelCaseOnly"
    },
    preprocessorOptions: {
      less: {
        modifyVars: getThemeVariables({}),
        javascriptEnabled: true
      }
    }
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          "react-venders": ["react", "react-dom", "@vitjs/runtime"]
        }
      }
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImltcG9ydCByZWFjdCBmcm9tICdAdml0ZWpzL3BsdWdpbi1yZWFjdCdcclxuaW1wb3J0IHsgZ2V0VGhlbWVWYXJpYWJsZXMgfSBmcm9tICdhbnRkL2Rpc3QvdGhlbWUnXHJcbmltcG9ydCBhdXRvSW1wb3J0IGZyb20gJ3VucGx1Z2luLWF1dG8taW1wb3J0L3ZpdGUnXHJcbmltcG9ydCB7IGRlZmluZUNvbmZpZyB9IGZyb20gJ3ZpdGUnXHJcbmltcG9ydCB2aXRlUGx1Z2luSW1wIGZyb20gJ3ZpdGUtcGx1Z2luLWltcCdcclxuaW1wb3J0IHdpbmRpQ1NTIGZyb20gJ3ZpdGUtcGx1Z2luLXdpbmRpY3NzJ1xyXG5pbXBvcnQgdHNjb25maWdQYXRocyBmcm9tICd2aXRlLXRzY29uZmlnLXBhdGhzJ1xyXG5cclxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cclxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcclxuICBiYXNlOiAnL3ZpdGUtcmVhY3QvJyxcclxuICBwbHVnaW5zOiBbXHJcbiAgICByZWFjdCh7XHJcbiAgICAgIGJhYmVsOiB7XHJcbiAgICAgICAgcGFyc2VyT3B0czoge1xyXG4gICAgICAgICAgcGx1Z2luczogWydkZWNvcmF0b3JzLWxlZ2FjeSddLFxyXG4gICAgICAgIH0sXHJcbiAgICAgIH0sXHJcbiAgICB9KSxcclxuICAgIHRzY29uZmlnUGF0aHMoKSxcclxuICAgIGF1dG9JbXBvcnQoe1xyXG4gICAgICBpbXBvcnRzOiBbXHJcbiAgICAgICAgJ3JlYWN0JyxcclxuICAgICAgICB7XHJcbiAgICAgICAgICByZWFjdDogW1xyXG4gICAgICAgICAgICAndXNlSW1wZXJhdGl2ZUhhbmRsZScsXHJcbiAgICAgICAgICAgICdjcmVhdGVFbGVtZW50JyxcclxuICAgICAgICAgICAgJ2Nsb25lRWxlbWVudCcsXHJcbiAgICAgICAgICAgICdjcmVhdGVDb250ZXh0JyxcclxuICAgICAgICAgICAgJ3VzZUxheW91dEVmZmVjdCcsXHJcbiAgICAgICAgICAgICdmb3J3YXJkUmVmJyxcclxuICAgICAgICAgIF0sXHJcbiAgICAgICAgfSxcclxuICAgICAgXSxcclxuICAgIH0pLFxyXG4gICAgdml0ZVBsdWdpbkltcCh7XHJcbiAgICAgIGxpYkxpc3Q6IFtcclxuICAgICAgICB7XHJcbiAgICAgICAgICBsaWJOYW1lOiAnYW50ZCcsXHJcbiAgICAgICAgICBzdHlsZTogKG5hbWUpID0+IGBhbnRkL2VzLyR7bmFtZX0vc3R5bGVgLFxyXG4gICAgICAgIH0sXHJcbiAgICAgIF0sXHJcbiAgICB9KSxcclxuICBdLFxyXG4gIHNlcnZlcjoge1xyXG4gICAgLy8gb3BlbjogdHJ1ZSxcclxuICAgIHBvcnQ6IDgwMDAsXHJcbiAgfSxcclxuICByZXNvbHZlOiB7XHJcbiAgICBhbGlhczogW1xyXG4gICAgICAvLyB7IGZpbmQ6ICdAJywgcmVwbGFjZW1lbnQ6IHBhdGgucmVzb2x2ZShcIkU6XFxcXERldmVsb3BlclxcXFxQZXRlci1zLVdvcmstU3BhY2VcXFxcR3JvdW5kX0ZpbmFuY2VcXFxcR3JvdW5kV2ViXCIsICdzcmMnKSB9LFxyXG4gICAgICAvLyBmaXggbGVzcyBpbXBvcnQgYnk6IEBpbXBvcnQgflxyXG4gICAgICAvLyBodHRwczovL2dpdGh1Yi5jb20vdml0ZWpzL3ZpdGUvaXNzdWVzLzIxODUjaXNzdWVjb21tZW50LTc4NDYzNzgyN1xyXG4gICAgICB7IGZpbmQ6IC9efi8sIHJlcGxhY2VtZW50OiAnJyB9LFxyXG4gICAgXSxcclxuICB9LFxyXG4gIGNzczoge1xyXG4gICAgbW9kdWxlczoge1xyXG4gICAgICBsb2NhbHNDb252ZW50aW9uOiAnY2FtZWxDYXNlT25seScsXHJcbiAgICB9LFxyXG4gICAgcHJlcHJvY2Vzc29yT3B0aW9uczoge1xyXG4gICAgICBsZXNzOiB7XHJcbiAgICAgICAgLy8gbW9kaWZ5VmFyczogeyAncHJpbWFyeS1jb2xvcic6ICcjMTNjMmMyJyB9LFxyXG4gICAgICAgIG1vZGlmeVZhcnM6IGdldFRoZW1lVmFyaWFibGVzKHtcclxuICAgICAgICAgIC8vIGRhcms6IHRydWUsIC8vIFx1NUYwMFx1NTQyRlx1NjY5N1x1OUVEMVx1NkEyMVx1NUYwRlxyXG4gICAgICAgICAgLy8gY29tcGFjdDogdHJ1ZSwgLy8gXHU1RjAwXHU1NDJGXHU3RDI3XHU1MUQxXHU2QTIxXHU1RjBGXHJcbiAgICAgICAgfSksXHJcbiAgICAgICAgamF2YXNjcmlwdEVuYWJsZWQ6IHRydWUsXHJcbiAgICAgIH0sXHJcbiAgICB9LFxyXG4gIH0sXHJcbiAgYnVpbGQ6IHtcclxuICAgIHJvbGx1cE9wdGlvbnM6IHtcclxuICAgICAgb3V0cHV0OiB7XHJcbiAgICAgICAgbWFudWFsQ2h1bmtzOiB7XHJcbiAgICAgICAgICAncmVhY3QtdmVuZGVycyc6IFsncmVhY3QnLCAncmVhY3QtZG9tJywgJ0B2aXRqcy9ydW50aW1lJ10sXHJcbiAgICAgICAgfSxcclxuICAgICAgfSxcclxuICAgIH0sXHJcbiAgfSxcclxufSkiXSwKICAibWFwcGluZ3MiOiAiO0FBQUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUVBO0FBR0EsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDMUIsTUFBTTtBQUFBLEVBQ04sU0FBUztBQUFBLElBQ1AsTUFBTTtBQUFBLE1BQ0osT0FBTztBQUFBLFFBQ0wsWUFBWTtBQUFBLFVBQ1YsU0FBUyxDQUFDLG1CQUFtQjtBQUFBLFFBQy9CO0FBQUEsTUFDRjtBQUFBLElBQ0YsQ0FBQztBQUFBLElBQ0QsY0FBYztBQUFBLElBQ2QsV0FBVztBQUFBLE1BQ1QsU0FBUztBQUFBLFFBQ1A7QUFBQSxRQUNBO0FBQUEsVUFDRSxPQUFPO0FBQUEsWUFDTDtBQUFBLFlBQ0E7QUFBQSxZQUNBO0FBQUEsWUFDQTtBQUFBLFlBQ0E7QUFBQSxZQUNBO0FBQUEsVUFDRjtBQUFBLFFBQ0Y7QUFBQSxNQUNGO0FBQUEsSUFDRixDQUFDO0FBQUEsSUFDRCxjQUFjO0FBQUEsTUFDWixTQUFTO0FBQUEsUUFDUDtBQUFBLFVBQ0UsU0FBUztBQUFBLFVBQ1QsT0FBTyxDQUFDLFNBQVMsV0FBVztBQUFBLFFBQzlCO0FBQUEsTUFDRjtBQUFBLElBQ0YsQ0FBQztBQUFBLEVBQ0g7QUFBQSxFQUNBLFFBQVE7QUFBQSxJQUVOLE1BQU07QUFBQSxFQUNSO0FBQUEsRUFDQSxTQUFTO0FBQUEsSUFDUCxPQUFPO0FBQUEsTUFJTCxFQUFFLE1BQU0sTUFBTSxhQUFhLEdBQUc7QUFBQSxJQUNoQztBQUFBLEVBQ0Y7QUFBQSxFQUNBLEtBQUs7QUFBQSxJQUNILFNBQVM7QUFBQSxNQUNQLGtCQUFrQjtBQUFBLElBQ3BCO0FBQUEsSUFDQSxxQkFBcUI7QUFBQSxNQUNuQixNQUFNO0FBQUEsUUFFSixZQUFZLGtCQUFrQixDQUc5QixDQUFDO0FBQUEsUUFDRCxtQkFBbUI7QUFBQSxNQUNyQjtBQUFBLElBQ0Y7QUFBQSxFQUNGO0FBQUEsRUFDQSxPQUFPO0FBQUEsSUFDTCxlQUFlO0FBQUEsTUFDYixRQUFRO0FBQUEsUUFDTixjQUFjO0FBQUEsVUFDWixpQkFBaUIsQ0FBQyxTQUFTLGFBQWEsZ0JBQWdCO0FBQUEsUUFDMUQ7QUFBQSxNQUNGO0FBQUEsSUFDRjtBQUFBLEVBQ0Y7QUFDRixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
