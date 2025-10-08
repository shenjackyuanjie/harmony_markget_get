import { defineConfig } from '@rsbuild/core';

export default defineConfig({
  server: {
    proxy: {
      "/api": "http://localhost:10003"
    }
  },
  html:{
    template: './src/index.html'
  }
});
