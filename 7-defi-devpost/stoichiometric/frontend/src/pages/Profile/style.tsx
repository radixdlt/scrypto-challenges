function styleFunction(device: string, themeStyle: string, accountSelect: boolean) {
    return {
        main: {
            display: 'flex',
            flexDirection: `${device == "mobile" || device == "tablet" ? "column-reverse" : 'row'}` as 'row',
            alignItems: 'center',
            justifyContent: `${device == "mobile" || device == "tablet" ? "flex-end" : 'space-evenly'}`,
            position: 'absolute' as 'absolute',
            left: `${device == "mobile" ? "10px" : device == "tablet" ? "115px" : '170px'}`,
            top: `${device == "mobile" ? "95px" : "30px"}`,
            height: `${device == "mobile" ? "calc(100% - 105px)" : "calc(100% - 60px)"}`,
            width: `${device == "mobile" ? "calc(100% - 20px)" : device == "tablet" ? "calc(100% - 135px)" : 'calc(100% - 190px)'}`,
        },

        settings: {
            padding: '20px',
            background: 'background2',
            borderRadius: '5px',
            width: `${device == "mobile" ? "calc(100vw - 40px - 20px)" : '300px'}`,
            color: 'shadow',
            boxShadow: '0px 1px 4px',
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            marginBottom: `${device == "mobile" || device == "tablet" ? '20px' : '40px'}`,
            marginRight: `${device == "mobile" || device == "tablet" ? "0" : '25px'}`,

            '& h1': {
                color: 'text',
                fontFamily: 'primary',
                fontSize: 4,
                fontWeight: '500',
                margin: '0',
                padding: '0',
                display: 'flex',
                justifyContent: 'space-between',
                width: '100%',
                marginBottom: '20px',
            }
        },

        profile: {
            padding: '20px',
            background: 'background2',
            borderRadius: '5px',
            width: `${device == "mobile" ? "calc(100vw - 40px - 25px)" : '600px'}`,
            color: 'shadow',
            boxShadow: '0px 1px 4px',
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            marginBottom: '20px',
            position: 'relative' as 'relative',

            '& h1': {
                color: 'text',
                fontFamily: 'primary',
                fontSize: 4,
                fontWeight: '500',
                margin: '0',
                padding: '0',
                display: 'flex',
                justifyContent: 'space-between',
                width: '100%',
                marginBottom: '20px',
            },

            '& button': {
                background: 'primary',
                marginTop: '20px',
                border: 'none',
                color: 'white',
                borderRadius: '10px',
                width: '150px',
                height: '50px',
                fontFamily: 'primary',
                fontSize: 2,
                cursor: 'pointer',
                zIndex: '1000',
                '&:hover': {
                    opacity: '.8'
                }
            }
        },


        themeSelector: {
            height: '25px',
            width: '25px',
            background: 'text',
            opacity: '.6',
            cursor: 'pointer',
            "-webkit-mask": `${themeStyle == "dark" ? "url('/icons/moon.svg') center/contain no-repeat" : "url('/icons/sun.svg') center/contain no-repeat"}`,
            mask: `${themeStyle == "dark" ? "url('/icons/moon.svg') center/contain no-repeat" : "url('/icons/sun.svg') center/contain no-repeat"}`,
            '&:hover': {
                opacity: '1'
            }
        },


        themeRow: {
            display: 'flex',
            alignItems: 'center',
            marginBottom: '20px',
            width: '100%',
            justifyContent: 'space-evenly'
        },

        accountSelector: {
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            borderRadius: '5px',
            border: 'solid .5px',
            borderColor: 'background3',
            width: 'calc(100% - 40px)',
            padding: '10px 20px',
            position: 'relative' as 'relative',
            cursor: 'pointer',

            '&: hover': {
                borderColor: 'text',

                '& div': {
                    opacity: 1
                }
            },

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
            }
        },

        expand: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask": `url('/icons/expand.svg') center/contain no-repeat`,
            mask: `url('/icons/expand.svg') center/contain no-repeat`,
        },

        accountSelectorContainer: {
            position: 'absolute' as 'absolute',
            left: 0,
            top: 0,
            width: '100%',
            height: '100%',
            overflow: 'hidden',
        },

        accountSelectorList: {
            position: 'absolute' as 'absolute',
            transition: '.2s',
            display: 'flex',
            flexDirection: 'column' as 'column',
            left: `${accountSelect ? '0%' : '100%'}`,
            top: 0,
            width: 'calc(100% - 40px)',
            height: 'calc(100% - 40px)',
            zIndex: '2000',
            background: 'background2',
            padding: '20px',

            '& h2': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '600',
                width: '100%',
                textAlign: 'center' as 'center',
                position: 'relative' as 'relative',
                marginBottom: '20px'
            }
        },

        accList: {
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            }
        },

        accChoice: {
            display: 'flex',
            justifyContent: 'space-between',
            width: 'calc(100% - 40px)',
            padding: '15px 20px',
            alignItems: 'center',
            cursor: 'pointer',
            '&: hover': {
                background: 'background3'
            },
            '& img': {
                height: '20px',
                aspectRatio: '1',
                objectFit: 'contain' as 'contain',
                borderRadius: '1000px',
                background: 'almostTransparent',
                marginRight: '15px'
            },
            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,

                '& span': {
                    opacity: '.3',
                    marginLeft: '15px'
                }
            }
        },

        close: {
            height: '17px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask": `url('/icons/expand.svg') center/contain no-repeat`,
            mask: `url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%) Rotate(180deg)',
            zIndex: '25',
            cursor: 'pointer',
            position: 'absolute' as 'absolute',
            left: '0',
            top: '50%',
            '&:hover': {
                opacity: '1'
            }
        },
    }
}

export default styleFunction;