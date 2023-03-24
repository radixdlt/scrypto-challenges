/** @jsxImportSource theme-ui */

import { useContext, useEffect } from "react";

import { SnackbarContext } from "contexts/SnackbarContext";

import { ResponsiveContext } from "contexts/ResponsiveContext";

function Snackbar() {
    const { alerts } = useContext(SnackbarContext);

    const { device } = useContext(ResponsiveContext);


    const style = {
        container: {
            display: 'flex',
            flexDirection: 'column-reverse' as 'column-reverse',
            alignItems: 'start',
            position: 'absolute' as 'absolute',
            bottom: `${device == "mobile" ? '20px' : '30px'}`,
            left: `${device == "mobile" ? '10px' : device == "tablet" ? '106px' : '160px'}`,
            height: '100px',
            zIndex: '999'
        },

        snackbar: {
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            padding: '15px',
            fontSize: 1,
            color: 'black',
            fontFamily: 'primary',
            borderRadius: '5px',
            marginTop: '10px',
            maxWidth: '300px',
            opacity: .9,
        },

        check: {
            background: 'green',
            '& div': {
                height: '20px',
                width: '20px',
                minHeight: '20px',
                minWidth: '20px',
                background: 'black',
                marginRight: '20px',
                "-webkit-mask": `url('/icons/check.svg') center/contain no-repeat`,
                mask: `url('/icons/check.svg') center/contain no-repeat`,
            }
        },

        error: {
            background: 'red',
            '& div': {
                height: '20px',
                width: '20px',
                minHeight: '20px',
                minWidth: '20px',
                background: 'black',
                marginRight: '20px',
                "-webkit-mask": `url('/icons/error.svg') center/contain no-repeat`,
                mask: `url('/icons/error.svg') center/contain no-repeat`,
            }
        },

        warning: {
            background: 'orange',
            '& div': {
                height: '20px',
                width: '20px',
                minHeight: '20px',
                minWidth: '20px',
                background: 'black',
                marginRight: '20px',
                "-webkit-mask": `url('/icons/error.svg') center/contain no-repeat`,
                mask: `url('/icons/error.svg') center/contain no-repeat`,
            }
        }
    }

    return (
        <div sx={style.container}>
            {alerts.map((x: any, index: number) => {
                return (
                    <div key={"snackbar" + index} sx={x.type == "check" ? { ...style.snackbar, ...style.check } : x.type == "error" ? { ...style.snackbar, ...style.error } : { ...style.snackbar, ...style.warning }}>
                        <div />
                        <span>{x.message}</span>
                    </div>)
            })}
        </div>
    )
}

export default Snackbar;