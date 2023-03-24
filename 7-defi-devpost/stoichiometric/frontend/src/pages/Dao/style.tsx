function styleFunction(device: string, expand: boolean) {
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

        voteContainer: {
            width: '700px',
            display: 'flex',
            justifyContent: 'space-between',
            marginBottom: '20px',
        },

        vote: {
            width: '80%',
            marginRight: '20px',
            padding: '0 20px',
            background: 'background2',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            position: 'relative' as 'relative',
        },

        column: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            marginTop: '20px',

            '& *': {
                color: 'text',
                fontFamily: 'primary',
                margin: '0',
                padding: '0'
            },

            '& h4': {
                fontsize: 1,
                opacity: '.7'
            }
        },

        caracteristics: {
            display: 'flex',
            flexDirection: 'column' as 'column',

            marginTop: '10px',
            marginBottom: '20px',
            marginLeft: '20px',

            '& li': {
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1
            }
        },

        score: {
            '& p': {
                color: 'text',
                fontSize: 3,
                fontFamily: 'primary',
                fontWeight: '600'
            }
        },

        voteButtons: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            justifyContent: 'space-between',
            padding: '20px 0',

            '& button': {
                background: 'primary',
                color: 'white',
                fontFamily: 'primary',
                fontSize: 4,
                fontweight: '600',
                border: 'none',
                borderRadius: '5px',
                width: '35px',
                cursor: 'pointer',

                '&:hover': {
                    opacity: '.8'
                }
            }
        },

        add: {
            border: 'none',
            background: 'primary',
            borderRadius: '5px',
            padding: '10px 20px',
            color: 'white',
            fontSize: 1,
            fontFamily: 'primary',
            cursor: 'pointer',
            marginBottom: '20px',

            '&:hover': {
                opacity: '.8',
            }
        },

        votesContainer: {
            height: 'calc(100vh - 150px)',
            overflow: 'scroll'
        },

        addProposalZone: {
            background: 'background2',
            marginBottom: '20px',
            borderRadius: '5px',
            padding: '20px',

            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            width: '660px',

            '& *': {
                color: 'text',
                fontFamily: 'primary',
            },

            '& label': {
                fontSize: 2,
                fontWeight: '600',
                marginBottom: '10px',
                width: '100%',
                cursor: 'pointer',
            },

            '& input, textarea': {
                background: 'transparent',
                width: 'calc(100% - 20px)',
                border: 'solid 1px',

                borderColor: 'background3',
                borderRadius: '5px',
                fontSize: 1,
                marginBottom: '20px',
                padding: '5px 10px',
            },

            '& textarea': {
                height: '200px',
                resize: 'none' as 'none'
            },

        },

        send: {
            background: 'primary',
            border: 'none',
            borderRadius: '5px',
            fontSize: '1',
            padding: '10px 20px',
            cursor: 'pointer',

            '&:hover': {
                opacity: '.8'
            }
        },

        property: {
            fontSize: 1,
            color: 'text',
            fontFamily: 'primary',
            border: 'solid 1px',
            borderColor: 'background3',
            borderRadius: '5px',
            padding: '5px 20px',
            position: 'relative' as 'relative',
            paddingRight: '40px',
            marginBottom: `${expand ? '10px' : '20px'}`,
            cursor: 'pointer',

            '&:hover': {
                borderColor: 'text',

                '& div': {
                    opacity: '1'
                }
            }
        },

        expand: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask": `url('/icons/expand.svg') center/contain no-repeat`,
            mask: `url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%) Rotate(90deg)',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '50%',
        },

        expand2: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask": `url('/icons/expand.svg') center/contain no-repeat`,
            mask: `url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%) Rotate(-90deg)',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '50%',
        },

        date: {
            position: 'absolute' as 'absolute',
            right: '20px',
            top: '20px',
            fontFamily: 'primary',
            fontSize: 1,
            color: 'text2',

            '& span': {
                fontWeight: '600',
                color: 'text'
            }
        },

        approved: {
            color: 'green !important'
        },

        declined: {
            color: 'red !important'
        },

        possibleChoices: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            marginBottom: '40px',
            border: 'solid 1px',
            borderColor: 'background3',
            borderRadius: '5px',
            padding: '5px 10px',

            '& p': {
                fontFamily: 'primary',
                fontSize: 1,
                color: 'text',
                margin: '0',
                padding: '0',
                marginBottom: '5px',
                cursor: 'pointer',
                opacity: '.7',
                '&: hover': {
                    opacity: 1
                }
            }
        }
    }
}

export default styleFunction