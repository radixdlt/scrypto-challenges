import React from 'react';
import ReactDOM from 'react-dom/client';
import { ToastContainer } from 'react-toastify';

import App from './App.tsx';
import { rdt } from './rdt/rdt';
import { RdtProvider } from './rdt/rdt-provider';
import 'react-toastify/dist/ReactToastify.css';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RdtProvider value={rdt}>
      <App />
      <ToastContainer position="bottom-right" />
    </RdtProvider>
  </React.StrictMode>,
);
