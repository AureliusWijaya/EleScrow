import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, path.resolve(__dirname, '../..'), ['CANISTER_', 'DFX_']);

  return {
    base: './',
    plugins: [react()],

    define: {
      'import.meta.env.VITE_DFX_NETWORK': JSON.stringify(env.DFX_NETWORK),
      'import.meta.env.VITE_CANISTER_ID_ELESCROW_BACKEND': JSON.stringify(env.CANISTER_ID_ELESCROW_BACKEND),
    },

    resolve: {
      alias: {
        declarations: path.resolve(__dirname, '../declarations'),
      },
    },

    server: {
      proxy: {
        '/api': {
          target: 'http://127.0.0.1:4943',
          changeOrigin: true,
        },
      },
    },
  };
});
