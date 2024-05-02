import { defineConfig } from 'vite';
import dotenv from 'dotenv';

dotenv.config({ path: `.env.${process.env.NODE_ENV}` });

export default {
    build: {
      rollupOptions: {
        input: {
          index: 'index.html',
          admin: 'admin.html'
        }
      }
    },  
    define: {
      'process.env.NODE_ENV': JSON.stringify('local')
    }
  }


// export default defineConfig({
//   // Your Vite configuration
// });

  