import { Theme } from 'theme-ui';
const makeTheme = <T extends Theme>(t: T) => t;

const light = makeTheme({
    colors: {
        primary: '#fa464b',
        background: 'rgba(250,250,250)',
        background2: 'rgba(255,255,255)',
        background3: 'rgba(220,220,220)',
        text: '#0b0b0b',
        text2: 'rgba(0, 0, 0, .5)',
        text3: 'rgba(0,0,0,.1)',
        text4: 'rgba(0,0,0,.25)',
        black: '#0b0b0b',
        white: '#fff',
        almostTransparent: 'rgba(0,0,0,.04)',
        greenSnackbar: '#509b52',
        redSnackbar: '#ca464b',
        green: '#14c784',
        red: '#ea3943',
        orange: '#f68b33',
        shadow: 'rgba(0,0,0,0.1)',
    }
});

export default light;