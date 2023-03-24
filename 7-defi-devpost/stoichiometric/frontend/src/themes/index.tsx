import light from './light';
import dark from './dark';

const globalTheme = {
    fonts: {
        primary: 'Poppins',
    },
    fontSizes: [
        14, 16, 18, 20, 24, 28, 32, 36, 42, "4vw", "1.3vw", "1.1vw", "2.3vw", 12
    ]
};


const themes = { light: light, dark: dark, global: globalTheme };

export default themes;