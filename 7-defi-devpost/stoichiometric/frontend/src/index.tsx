import React from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';

import { ThemeCtx } from 'contexts/ThemeContext';
import { ResponsiveCtx } from 'contexts/ResponsiveContext';
import { UserCtx } from 'contexts/UserContext';
import { SnackbarCtx } from 'contexts/SnackbarContext';
import { BurgerCtx } from 'contexts/BurgerContext';
import { TokensCtx } from 'contexts/TokensContext';

import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';

const root = createRoot(
    document.getElementById('root') as HTMLElement
);

root.render(
    <React.StrictMode>
        <ResponsiveCtx>
            <SnackbarCtx>
                <ThemeCtx>
                    <TokensCtx>
                        <UserCtx>
                            <BurgerCtx>
                                <BrowserRouter>
                                    <App />
                                </BrowserRouter>
                            </BurgerCtx>
                        </UserCtx>
                    </TokensCtx>
                </ThemeCtx>
            </SnackbarCtx>
        </ResponsiveCtx>
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();