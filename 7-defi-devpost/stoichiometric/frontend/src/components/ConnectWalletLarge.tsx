/** @jsxImportSource theme-ui */

import { useContext } from "react";

import { UserContext } from "contexts/UserContext";

import { ResponsiveContext } from "contexts/ResponsiveContext";

export interface Props { };

const ConnectWallet: React.FC<Props> = () => {
    const { connectUser, user, connectionLoading } = useContext(UserContext);

    const { device } = useContext(ResponsiveContext);

    const style = {
        button: {
            color: 'text',
            fontFamily: 'primary',
            fontSize: 2,
            background: 'almostTransparent',
            fontWeight: '300',
            border: 'none',
            cursor: 'pointer',
            zIndex: '20',
            height: '50px',
            width: '200px',
            padding: '15px',
            borderColor: 'transparent',
            borderRadius: '10px',
            whiteSpace: 'nowrap' as 'nowrap',
            display: 'flex',
            alignItems: 'center',
            justifyContent: `space-between`,
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
            display: 'flex',
            justifyContent: 'center',
            '&::after': {
                content: '""',
                border: 'solid 3px',
                borderColor: 'white',
                borderLeftColor: 'primary',
                borderRadius: '1000px',
                height: '70% !important',
                aspectRatio: '1',
                background: 'transparent',
                animation: 'rotating 1s linear infinite',
            }
        },

        buttonConnected: {
            background: 'primary'
        },

        iconLogin: {
            transform: 'scaleX(-1)',

            background: 'text',
            height: '22px',
            aspectRatio: '1',
            opacity: '.3',

            margin: '0',
            padding: '0',
            "-webkit-mask": `url('/pages/wallet.svg') center/contain no-repeat`,
            mask: `url('/pages/wallet.svg') center/contain no-repeat`,
        },

        iconProfile: {

            background: 'white',
            height: '22px',
            aspectRatio: '1',
            opacity: '1',

            margin: '0',
            padding: '0',
            "-webkit-mask": `url('/pages/wallet.svg') center/contain no-repeat`,
            mask: `url('/pages/wallet.svg') center/contain no-repeat`,
        },

        loading: {
            position: 'absolute' as 'absolute',
        },

        phrase: {
            color: `${user.address ? 'white' : 'text'}`,
            fontFamily: 'primary',
            fontSize: 1,
            fontWeight: 500,
            marginLeft: "25px",
            padding: 1,
            opacity: `${user.address ? 1 : '.3'}`,
        }
    };


    return (
        <button sx={connectionLoading ? { ...style.button, ...style.buttonLoading } : user.address ? { ...style.button, ...style.buttonConnected } : style.button} onClick={connectUser}>
            <div sx={connectionLoading ? style.loading : user.address ? style.iconProfile : style.iconLogin} ></div>
            {!connectionLoading ?
                <p sx={style.phrase}>{user.address ? user.address.slice(0, 10) + "..." + user.address.slice(user.address.length - 5, user.address.length) : "Connect wallet"}</p>
                : null
            }
        </button>
    )
}

export default ConnectWallet;