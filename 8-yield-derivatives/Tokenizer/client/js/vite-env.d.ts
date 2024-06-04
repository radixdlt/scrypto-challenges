/// <reference types="vite/client" />

declare global {
    interface ImportMetaEnv {
      readonly VITE_DAPP_ID: string;
      // Add other environment variables here if needed
    }
  }
  
