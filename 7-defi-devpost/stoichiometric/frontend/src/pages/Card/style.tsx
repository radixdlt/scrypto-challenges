function styleFunction(device: string, swapLoading: boolean, token1Select: boolean) {
    return {
        main: {
            display: 'flex',
            justifyContent: 'space-between',
            position: 'absolute' as 'absolute',
            left: `${device == "mobile" ? "10px" : device == "tablet" ? "115px" : '170px'}`,
            top: `${device == "mobile" ? "95px" : "30px"}`,
            height: `${device == "mobile" ? "calc(100% - 105px)" : "calc(100% - 60px)"}`,
            width: `${device == "mobile" ? "calc(100% - 20px)" : device == "tablet" ? "calc(100% - 135px)" : 'calc(100% - 190px)'}`,
        },

        top: {
            height: '100%',
            width: '100%',
            position: 'relative' as 'relative',
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            }
        },

        container: {
            width: `${device == "mobile" ? '100%' : '510px'}`,
            position: 'absolute' as 'absolute',
            left: '50%',
            top: `${device == "mobile" ? '0px' : '50%'}`,
            transform: `${device == "mobile" ? 'TranslateX(-50%)' : 'Translate(-50%, -50%)'}`,
            display: 'flex',
            flexDirection: 'column' as 'column'
        },

        swapZone: {
            padding: '20px',
            height: 'auto',
            width: 'calc(100% - 40px)',
            background: 'background2',
            color: 'shadow',
            boxShadow: '0px 1px 4px',
            borderRadius: '5px',
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            overflow: "hidden",
            position: 'relative' as 'relative',
            marginBottom: '20px',

            '& h1': {
                margin: '0',
                padding: '0',
                width: '100%',
                color: 'text',
                fontSize: 4,
                fontWeight: '600',
                marginBottom: '20px'
            }
        },

        inputBar: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '5px',

            '& input': {
                fontFamily: 'primary',
                fontSize: 2,
                width: 'calc(100% - 140px - 2px)',
                padding: '10px',
                paddingRight: '130px',
                margin: '0',
                border: 'solid 1px',
                borderColor: 'background3',
                background: 'background2',
                borderRadius: '5px',
                color: 'text',

                '&:focus': {
                    outline: 'none',
                    borderColor: 'text',

                    '& ~ label': {
                        left: '10px',
                        top: '0',
                        fontSize: 1,
                        transform: 'TranslateY(-50%)',
                        color: 'text'
                    }
                },

                '&:not(:placeholder-shown) ~ label': {
                    left: '10px',
                    top: '0',
                    fontSize: 1,
                    transform: 'TranslateY(-50%)',
                }
            },

            '& label': {
                position: 'absolute' as 'absolute',
                left: '20px',
                top: '50%',
                transform: 'TranslateY(-50%)',
                fontFamily: 'primary',
                fontSize: 2,
                zIndex: '10',
                padding: '0 5px',
                transition: '.1s',
                transitionProperty: 'left, top',
                cursor: 'text',
                background: 'background2',
                color: 'text2',
            },

            '& #get ~ label, #get': {
                cursor: 'not-allowed'
            }
        },

        swapIcon: {
            height: '20px',
            marginBottom: '20px',
            cursor: 'pointer',
            aspectRatio: '1',
            opacity: '.3',
            transform: 'Rotate(90deg)',
            background: 'text',
            "-webkit-mask": `url('/icons/swap.svg') center/contain no-repeat`,
            mask: `url('/icons/swap.svg') center/contain no-repeat`,
            '&:hover': {
                opacity: '1'
            }
        },

        swapIcon2: {
            height: '20px',
            marginRight: '20px',
            cursor: 'pointer',
            aspectRatio: '1',
            opacity: '.3',
            transform: 'Rotate(90deg)',
            background: 'text',
            "-webkit-mask": `url('/icons/swap.svg') center/contain no-repeat`,
            mask: `url('/icons/swap.svg') center/contain no-repeat`,
            '&:hover': {
                opacity: '1'
            }
        },

        token: {
            width: '110px',
            height: '100%',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '0',
            cursor: 'pointer',

            display: 'flex',
            alignItems: 'center',

            '& img': {
                height: '50%',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px'
            },

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '500'
            },
            '&:hover div': {
                opacity: '1'
            }
        },

        token2: {
            width: '130px',
            height: '100%',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '0',
            cursor: 'not-allowed',

            display: 'flex',
            alignItems: 'center',

            '& img': {
                height: '50%',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px'
            },

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '500'
            },
        },

        token3: {
            width: '130px',
            height: '100%',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '0',
            cursor: 'pointer',

            display: 'flex',
            alignItems: 'center',

            '& img': {
                height: '50%',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px'
            },

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '500'
            },
        },

        expand: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask": `url('/icons/expand.svg') center/contain no-repeat`,
            mask: `url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%)',
            position: 'absolute' as 'absolute',
            right: '0',
            top: '50%',
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

        tokenAddress: {
            color: 'text2',
            fontFamily: 'primary',
            marginBottom: '25px',
            fontSize: 0,
            width: 'calc(100% - 10px)',
            display: 'flex',
            justifyContent: 'space-between',
            overflow: 'hidden',


            '& span': {
                color: 'text2',
                fontWeight: '500',
                whiteSpace: 'nowrap' as 'nowrap',
                marginRight: '20px',
            }
        },

        swapInfos: {
            width: '100%',
            display: 'flex',
            flexDirection: 'column' as 'column',
            marginTop: '10px',
            marginBottom: '20px'
        },

        swapInfoMain: {
            color: 'text',
            fontFamily: 'primary',
            width: '100%',
            fontSize: 1,
            fontWeight: '600',
            display: 'flex',
            justifyContent: 'space-between',

            '& div': {
                display: 'flex',
                alignItems: 'center'
            },

            '& div div': {
                height: '15px',
                transform: 'Rotate(90deg)',
                margin: '0 5px',
                aspectRatio: '1',
                background: 'text', "-webkit-mask": `url('/icons/smallArrow.svg') center/contain no-repeat`,
                mask: `url('/icons/smallArrow.svg') center/contain no-repeat`,
            }
        },

        swapInfo: {
            color: 'text2',
            fontFamily: 'primary',
            width: '100%',
            fontSize: 0,
            fontWeight: '400',
            display: 'flex',
            justifyContent: 'space-between',
            marginTop: '5px',
        },

        swapButton: {
            background: 'primary',
            border: 'none',
            color: 'white',
            borderRadius: '10px',
            width: '150px',
            height: '50px',
            fontFamily: 'primary',
            fontSize: 2,
            cursor: 'pointer',
            margin: '0 20px',
            '&:hover': {
                opacity: `${swapLoading ? '1' : '.8'}`
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

        selectToken: {
            height: 'calc(100% - 40px)',
            width: 'calc(100% - 40px)',
            padding: '20px',
            background: 'background2',
            position: 'absolute' as 'absolute',
            left: `${token1Select ? '0' : '100%'}`,
            transition: '.2s',
            top: '0',
            zIndex: '20',
            display: 'flex',
            flexDirection: 'column' as 'column',

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

        tokensList: {
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            }
        },

        tokenChoice: {
            display: 'flex',
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
                fontWeight: '500',
                fontSize: 1,

                '& span': {
                    opacity: '.3',
                    marginLeft: '15px'
                }
            }
        },

        stableBarContainer: {
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            marginTop: '20px',
            marginBottom: '20px',
        },

        stableBar: {
            width: '100%'
        },

        alert: {
            width: '60%',
            border: 'solid 1px',
            borderColor: 'red',
            borderRadius: '5px',
            padding: '5px 15px',

            marginBottom: '20px',

            '& p': {
                margin: '0',
                padding: '0',
                fontSize: 0,
                fontFamily: 'primary',
                color: 'red',
                textAlign: 'center' as 'center'
            }
        }
    }
}

export default styleFunction;