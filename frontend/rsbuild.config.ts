import { defineConfig } from '@rsbuild/core';

export default defineConfig({
  html: {
    template: './src/index.html'
  },
  server: {
    proxy: {
      '/api': 'http://localhost:10003',
    }
  }
});
