import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': '/src',
    },
  },
  build: {
    rollupOptions: {
      input: 'src/content.ts',
      output: {
        entryFileNames: 'content.js',
        assetFileNames: (assetInfo) => {
          if (assetInfo.name?.endsWith('.css')) {
            return 'content.css';
          }
          return assetInfo.name || 'assets/[name].[ext]';
        },
        format: 'iife',
        inlineDynamicImports: false,
      },
    },
    outDir: 'dist',
    emptyOutDir: false,
    target: 'es2015',
    cssCodeSplit: false,
  },
})