function styleFunction(device: string, swapLoading: boolean, token1Select: boolean, chosePosition: boolean, price1: number, price2: number, minPrice: number, maxPrice: number) {
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
            width: `100%`,
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
        },


        inputBar2: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '5px',

            '& input': {
                fontFamily: 'primary',
                fontSize: 2,
                width: 'calc(100% - 30px - 2px)',
                padding: '10px',
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
        },

        swapIcon: {
            height: '20px',
            marginBottom: '20px',
            cursor: 'pointer',
            aspectRatio: '1',
            opacity: '.3',
            transform: 'Rotate(90deg)',
            background: '',
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
                fontSize: 1,
                fontWeight: '500'
            },
            '&:hover div': {
                opacity: '1'
            }
        },

        token2: {
            width: '110px',
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

        expand2: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask": `url('/icons/expand.svg') center/contain no-repeat`,
            mask: `url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%)',
            position: 'absolute' as 'absolute',
            right: '20px',
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
            marginBottom: '10px',
            fontSize: 0,
            width: 'calc(100% - 10px)',
            display: 'flex',
            justifyContent: 'space-between',
            overflow: 'hidden',

            '&:nth-of-type(1)': {
                marginBottom: '25px'
            },


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
            marginBottom: '20px',

            '& button': {
                background: 'primary',
                color: 'text',
                borderRadius: '5px',
                border: 'none',
                fontFamily: 'primary',
                fontSize: 1,
                cursor: 'pointer',
                marginBottom: '10px',
                width: '40%',
                marginLeft: '30%',

                '&:hover': {
                    opacity: '.8'
                }
            }
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
            width: '250px',
            height: '50px',
            fontFamily: 'primary',
            fontSize: 2,
            cursor: 'pointer',
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
            }
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

        poolsList: {
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            },
            width: '100%',
        },

        poolChoice: {
            display: 'flex',
            width: 'calc(100% - 40px)',
            padding: '15px 20px',
            alignItems: 'center',
            cursor: 'pointer',
            '&: hover': {
                background: 'background3'
            },
            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1,
                fontWeight: '500'
            },

            '& img': {
                height: '25px',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px',
                '&:nth-of-type(1)': {
                    transform: 'Translate(0, -3px)'
                },
                '&:nth-of-type(2)': {
                    transform: 'Translate(-15px, 3px)',
                    marginRight: '0px',
                }
            },
        },

        buttons: {
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            width: '100%',
            marginBottom: '10px',

            '& span': {
                fontFamily: 'primary',
                fontSize: 1,
                borderRadius: '5px',
                padding: '5px 10px',
            }
        },

        active: {
            background: 'background2',
            color: 'text',
        },

        inactive: {
            color: 'text2',
            cursor: 'pointer',
            '&: hover': {
                background: 'background2',
            }
        },

        chosePosition: {
            padding: '20px',
            height: 'auto',
            width: 'calc(100% - 40px)',
            background: 'background2',
            color: 'shadow',
            boxShadow: '0px 1px 4px',
            borderRadius: '5px',
            display: 'flex',
            alignItems: 'center',
            overflow: "hidden",
            position: 'relative' as 'relative',
            marginBottom: '20px',
            cursor: 'pointer',

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1,
                fontWeight: '500'
            },

            '& img': {
                height: '25px',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px',
                '&:nth-of-type(1)': {
                    transform: 'Translate(0, -3px)'
                },
                '&:nth-of-type(2)': {
                    transform: 'Translate(-15px, 3px)',
                    marginRight: '0px',
                }
            },

            '&:hover div': {
                opacity: 1
            }
        },

        chosePositionZone: {
            position: 'absolute' as 'absolute',
            top: `0px`,
            left: `${chosePosition ? '0px' : '100%'}`,
            height: 'calc(100% - 60px)',
            width: 'calc(100% - 40px)',
            padding: '20px',
            background: 'background2',
            color: 'shadow',
            borderRadius: '5px',
            display: 'flex',
            alignItems: 'center',
            cursor: 'default',
            zIndex: '1000',
            transition: '.2s',
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

        chosePositionContainer: {
            height: '100%',
            width: '100%',
            position: 'absolute' as 'absolute',
            left: 0,
            top: 0,
            overflow: 'hidden',
        },

        myPositionColumn: {
            position: 'relative' as 'relative',
        },

        rangeRow: {
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            width: '100%',
            marginBottom: '20px',

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1,
            },
        },

        range: {
            width: '90%',
            '-webkit-appearance': 'none',
            background: 'transparent',
            height: '100%',

            '&:hover::-webkit-slider-thumb': {
                background: 'primary'
            },

            '&:focus': {
                outline: 'none'
            },

            '&::-webkit-slider-runnable-track': {
                width: '100%',
                height: '5px',
                cursor: 'pointer',
                animate: '0.2s',
                color: 'shadow',
                boxShadow: 'none',
                background: 'background3',
                borderRadius: '50px',
                border: 'none',
            },

            '&::-webkit-slider-thumb': {
                boxShadow: '0px 1px 4px',
                border: 'none',
                height: '20px',
                width: '20px',
                borderRadius: '50px',
                background: 'background3',
                cursor: 'pointer',
                '-webkit-appearance': 'none',
                marginTop: '-7.5px',
            }
        },


        rangeInput: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '20px',
            marginTop: '10px',

            '& p': {
                margin: 0,
                marginBottom: '-35px',
                padding: 0,
                fontFamily: 'primary',
                fontSize: '1',
                fontWeight: '600',
                color: 'text'
            }
        },

        range2: {
            width: '100%',
            '-webkit-appearance': 'none',
            background: 'transparent',
            height: '100%',

            '&:hover::-webkit-slider-thumb': {
                background: 'primary',
                cursor: 'pointer',
                animate: '0.2s',
            },

            '&:focus': {
                outline: 'none'
            },

            '&::-webkit-slider-runnable-track': {
                height: '0',
                width: '100%',
            },

            '&::-webkit-slider-thumb': {
                boxShadow: '0px 1px 4px',
                border: 'none',
                height: '20px',
                width: '20px',
                borderRadius: '50px',
                background: 'background3',
                cursor: 'pointer',
                '-webkit-appearance': 'none',
                marginTop: '-7.5px',
            },

            '&:nth-of-type(1)': {
                transform: 'TranslateY(35px)',
            },

            '&:nth-of-type(2)': {
                transform: 'TranslateY(17px)',
            },

            '&:nth-of-type(3)::-webkit-slider-thumb': {
                display: 'none'
            }
        },

        rangeBar: {
            position: 'absolute' as 'absolute',
            left: '0',
            bottom: '0',
            transform: 'Translate(0, 1.5px)',
            width: '100%',
            height: '5px',
            color: 'shadow',
            boxShadow: 'none',
            background: 'background3',
            borderRadius: '50px',
            border: 'none',
            overflow: 'hidden' as 'hidden',

            '& div': {
                position: 'absolute' as 'absolute',
                left: `Calc(2.5px + ${100 * Math.sqrt((Math.min(price1, price2) - minPrice) / (maxPrice - minPrice))}%)`,
                width: `${100 * Math.abs(Math.sqrt((price1 - minPrice) / (maxPrice - minPrice)) - Math.sqrt((price2 - minPrice) / (maxPrice - minPrice)))}%`,
                height: '100%',
                boxShadow: 'none',
                background: 'primary',
                border: 'none',
            }
        },

        rangeInputs: {
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',

            '& input': {
                width: '100px',
            }
        },

        ranges: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '25px'
        }
    }
}

export default styleFunction;