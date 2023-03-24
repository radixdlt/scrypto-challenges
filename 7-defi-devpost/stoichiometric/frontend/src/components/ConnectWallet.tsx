/** @jsxImportSource theme-ui */

import { useContext } from "react";

import { UserContext } from "contexts/UserContext";

import { ResponsiveContext } from "contexts/ResponsiveContext";

import { NavLink } from "react-router-dom";

export interface Props {
    children?: string;
    parameters?: boolean;
};

const ConnectWallet: React.FC<Props> = (props) => {
    const { connectUser, user, connectionLoading } = useContext(UserContext);

    const { device } = useContext(ResponsiveContext);

    const style = {
        button: {
            color: 'text',
            fontFamily: 'primary',
            position: 'relative' as 'relative',
            fontSize: 1,
            background: 'almostTransparent',
            fontWeight: '300',
            border: 'none',
            cursor: 'pointer',
            zIndex: '2000',
            height: `${device == "mobile" ? '40px' : '50px'}`,
            aspectRatio: '1',
            padding: `0`,
            borderColor: 'transparent',
            borderRadius: `1000px`,
            whiteSpace: 'nowrap' as 'nowrap',
            display: 'flex',
            alignItems: 'center',
            justifyContent: `center`,
            '&:hover': {
                background: 'primary',
            },
            '&:hover div': {
                opacity: '1 !important',
                background: 'white'
            },
            '&:hover p': {
                opacity: '1 !important',
                color: 'white'
            }
        },

        buttonLoading: {
            background: 'primary',
            cursor: 'default !important',
            '&:after': {
                content: '""',
                border: 'solid 3px',
                borderRadius: '1000px',
                borderColor: 'white',
                borderLeftColor: 'primary',
                width: '30% !important',
                aspectRatio: '1',
                background: 'transparent',
                animation: 'rotating 1s linear infinite',
            }
        },

        buttonConnected: {
            background: 'primary',

            '&:hover div': {
                transform: 'Rotate(180deg)',
            }
        },

        iconLogin: {
            transform: 'scaleX(-1)',

            background: 'text',
            height: '17px',
            aspectRatio: '1',
            opacity: '.3',

            margin: '0',
            padding: '0',
            "-webkit-mask": `url('/pages/wallet.svg') center/contain no-repeat`,
            mask: `url('/pages/wallet.svg') center/contain no-repeat`,
        },

        iconProfile: {

            background: 'white',
            height: '17px',
            aspectRatio: '1',
            opacity: '1',

            margin: '0',
            padding: '0',
            "-webkit-mask": `url('/pages/parameters.svg') center/contain no-repeat`,
            mask: `url('/pages/parameters.svg') center/contain no-repeat`,

            transition: 'transform .2s'
        },

        loading: {
            position: 'absolute' as 'absolute',
        },

        phrase: {
            color: 'text',
            fontFamily: 'primary',
            fontSize: 0,
            fontWeight: 500,
            marginLeft: "25px",
            padding: 0,
            opacity: `${user.address ? 1 : '.3'}`,
        },

    };


    /*        { device == 'mobile' && !connectionLoading ?
    <p sx={style.phrase}>{user.address ? user.address.slice(0,10)+"..."+user.address.slice(user.address.length - 5, user.address.length) : "Connect wallet"}</p>
    : null
}
*/

    if (!user.address) {
        return (
            <button sx={connectionLoading ? { ...style.button, ...style.buttonLoading } : style.button} onClick={connectUser}>
                <div sx={connectionLoading ? style.loading : style.iconLogin} ></div>
            </button>
        )
    } else {
        return (
            <NavLink to={`/profile`}>
                <button sx={{ ...style.button, ...style.buttonConnected }}>
                    <div sx={style.iconProfile}></div>
                </button>
            </NavLink>
        )
    }
}

export default ConnectWallet;