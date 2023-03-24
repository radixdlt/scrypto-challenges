import { Theme } from 'theme-ui';
const makeTheme = <T extends Theme>(t: T) => t;

const dark = makeTheme({
    colors: {
        primary: '#fa464b',
        background2: 'rgba(24,24,24)',
        background: 'rgba(16,16,16)',
        background3: 'rgba(44,44,44)',
        text: '#fff',
        text2: 'rgba(255, 255, 255, .5)',
        text3: 'rgba(255,255,255,.1)',
        text4: 'rgba(255,255,255,.25)',
        black: '#0b0b0b',
        white: '#fff',
        almostTransparent: 'rgba(255,255,255,.04)',
        greenSnackbar: '#509b52',
        redSnackbar: '#ca464b',
        green: '#14c784',
        red: '#ea3943',
        orange: '#f68b33',
        green2: '#14c78422',
        red2: '#ea394322',
        orange2: '#f68b3322',
        shadow: 'rgba(0,0,0,0.5)',
    }
});

export default dark;