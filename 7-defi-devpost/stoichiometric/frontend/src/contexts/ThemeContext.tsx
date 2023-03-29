import React, { useState } from "react";

const ThemeContext = React.createContext(null as any);

interface Props {
    children: any;
}

const ThemeCtx: React.FC<Props> = (props) => {

    const [themeStyle, setThemeStyle] = useState('dark');

    const toggleTheme = () => {
        setThemeStyle(themeStyle === 'light' ? 'dark' : 'light')
    }

    return (
        <ThemeContext.Provider value={{ themeStyle, toggleTheme, setThemeStyle }}>
            {props.children}
        </ThemeContext.Provider>
    )

};

export { ThemeContext, ThemeCtx };