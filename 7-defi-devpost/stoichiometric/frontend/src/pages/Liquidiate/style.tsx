function styleFunction(device: string, liquidateLoading: boolean) {
    return {
        main: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            position: 'absolute' as 'absolute',
            left: `${device == "mobile" ? "10px" : device == "tablet" ? "115px" : '170px'}`,
            top: `${device == "mobile" ? "95px" : "100px"}`,
            height: `${device == "mobile" ? "calc(100% - 105px)" : "calc(100% - 60px)"}`,
            width: `${device == "mobile" ? "calc(100% - 20px)" : device == "tablet" ? "calc(100% - 135px)" : 'calc(100% - 190px)'}`,
        },

        lend: {
            width: `${device == "mobile" ? '100%' : '250px'}`,
            height: '150px',
            padding: '20px',
            marginBottom: '20px',
            background: 'background2',
            display: 'flex',
            flexDirection: 'column' as 'column',
            position: 'relative' as 'relative',
            borderRadius: '5px',

            '& p': {
                color: 'text2',
                fontFamily: 'primary',
                fontSize: 1,
                padding: '0',
                margin: '0',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                width: '100%',
                marginTop: '10px',

                '&:nth-of-type(1)': {
                    marginTop: '15px',
                }
            },

            '& span': {
                display: 'flex',
                alignItems: 'center',
                color: 'text',
                fontWeight: '600'
            },

            '& img': {
                width: '25px',
                height: '25px',
                borderRadius: '1000px',
                marginLeft: '10px'
            }
        },

        check: {
            display: 'flex',
            alignItems: 'center',
            color: 'text',
            fontFamily: 'primary',
            fontSize: 0,
            width: '80vw',
            marginBottom: '20px',

            '& input': {
                marginRight: '10px'
            },

            '& *': {
                cursor: 'pointer'
            }
        },

        lendColumn: {
            display: 'flex',
            justifyContent: 'space-between',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            width: `80vw`,
        },

        lendRow: {
            display: 'flex',
            alignItems: 'center',
            width: '100%',
            justifyContent: 'space-between',
        },

        lendContainer: {
            overflow: 'scroll',
            height: 'calc(100vh - 180px)',
        },

        barContainer: {
            width: 'calc(100% - 40px)',
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '30px',
            transform: 'translateX(-50%)',


            '&:hover div': {
                display: 'block'
            }
        },


        infos: {
            background: 'background3',
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '20px',
            transform: 'TranslateX(-50%)',
            display: 'none',
            width: '100%',
            padding: '5px 10px',
            borderRadius: '5px',

            '& p': {
                fontSize: 0,
                margin: '0 !important'
            }
        },

        bar: {
            width: '100%',
            height: '10px',
            background: 'background3',
            borderRadius: '1000px',
            position: 'relative' as 'relative',
            overflow: 'hidden',

            '& div': {
                position: 'absolute' as 'absolute',
                width: '100%',
                height: '100%',
                top: '50%',
                transform: 'translateY(-50%)',
            },
        },

        swapButton: {
            background: 'primary',
            position: 'absolute' as 'absolute',
            bottom: '20px',
            left: '50%',
            transform: 'TranslateX(-50%)',
            border: 'none',
            color: 'white',
            borderRadius: '10px',
            width: '150px',
            height: '50px',
            fontFamily: 'primary',
            fontSize: 2,
            cursor: 'pointer',
            '&:hover': {
                opacity: `${liquidateLoading ? '1' : '.8'}`
            }
        },

        swapButtonLoading: {
            background: 'primary',
            cursor: 'default !important',
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            '&::after': {
                content: '""',
                border: 'solid 3px',
                borderLeftColor: 'primary',
                borderRadius: '1000px',
                height: '30% !important',
                aspectRatio: '1',
                background: 'transparent',
                animation: 'rotating 1s linear infinite',
            },
        },

    }
}

export default styleFunction;