import { useContext } from "react";

import { Theme } from 'theme-ui';

import { ThemeProvider } from '@theme-ui/core';
import { Global } from '@emotion/react';

import { ThemeContext } from 'contexts/ThemeContext';
import themes from "./themes";

import Routes from './routes';

const makeTheme = <T extends Theme>(t: T) => t;

function App() {
    const { themeStyle } = useContext(ThemeContext);

    return (
        <div>
            <Global
                styles={() => ({
                    'body': {
                        background: (themeStyle == "light" ? themes.light : themes.dark).colors.background,
                    },
                })}
            />
            <ThemeProvider theme={themeStyle == "light" ? {
                ...themes.global, ...makeTheme({
                    colors: themes.light.colors
                })
            }
                : {
                    ...themes.global, ...makeTheme({
                        colors: themes.dark.colors
                    })
                }}
            >
                <Routes />
            </ThemeProvider>
        </div>
    );
}

export default App;
