function styleFunction(device: string) {
    return {
        navlink: {
            zIndex: '1000'
        },

        top: {
            position: 'absolute' as 'absolute',
            left: '0',
            top: '0',

            padding: `0 ${device == "mobile" ? "20px" : "50px"}`,
            width: `calc(100% - ${device == "mobile" ? "40px" : "100px"})`,
            height: `${device == "mobile" ? "70px" : "100px"}`,

            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between'
        },

        beaker: {
            fontFamily: 'primary',
            fontSize: 5,
            color: 'text',
            fontWeight: '600',

            margin: '0',
            padding: '0'
        },

        fi: {
            fontFamily: 'primary',
            fontSize: 2,
            color: 'primary',

            margin: '0',
            padding: '0'
        },

        right: {
            display: 'flex',
            alignItems: 'center'
        },

        social: {
            height: '20px',
            aspectRatio: '1',
            background: 'text',
            zIndex: '1000',
            cursor: 'pointer',
            marginRight: '20px',
            '&:hover': {
                background: 'primary'
            },
            '&:nth-of-type(3)': {
                marginRight: '40px'
            }
        },

        telegram: {
            "-webkit-mask": "url('/social/telegram.svg') center/contain",
            mask: "url('/social/telegram.svg') center/contain",
        },

        twitter: {
            "-webkit-mask": "url('/social/twitter.svg') center/contain",
            mask: "url('/social/twitter.svg') center/contain",
        },

        discord: {
            "-webkit-mask": "url('/social/discord.svg') center/contain",
            mask: "url('/social/discord.svg') center/contain",
        },

        link: {
            color: 'text',
            marginRight: '40px',
            fontSize: 1,
            fontWeight: 500,
            opacity: .6,
            cursor: 'pointer',
            zIndex: '1000',
            '&:hover': {
                opacity: 1
            }
        },

        center: {
            width: 'calc(100% - 40px)',
            padding: "0 20px",
            left: '50%',
            marginTop: `${device == "mobile" ? "150px" : "200px"}`,
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: `${device == "mobile" ? "start" : "center"}`
        },

        catchphrase: {
            fontFamily: 'primary',
            fontSize: `${device == "mobile" ? 7 : 9}`,
            color: 'text',
            fontWeight: '600',
            textTransform: 'capitalize' as 'capitalize',
            width: `${device == "mobile" ? "80%" : "auto"}`,

            margin: '0',
            padding: '0'
        },

        subtitle: {
            fontFamily: 'primary',
            fontSize: `${device == "mobile" ? 2 : 10}`,
            width: `${device == "mobile" ? "70%" : "auto"}`,
            color: 'text',
            fontWeight: '300',

            margin: '0',
            padding: '0',
        },

        socialRow: {
            display: 'flex',
            alignItems: 'center',
            marginTop: '15px',
        },

        highlight: {
            color: 'primary'
        },

        bitcoin: {
            height: '150px',
            aspectRatio: '1',
            position: 'absolute' as 'absolute',
            right: '30%',
            bottom: '30%',
            zIndex: '-1',
            filter: 'grayscale(100%)',
            opacity: '.15',
            animation: 'oscilate 10s ease-in-out infinite'
        },

        ethereum: {
            height: '150px',
            aspectRatio: '1',
            position: 'absolute' as 'absolute',
            left: '20%',
            bottom: '50%',
            zIndex: '-1',
            filter: 'grayscale(100%)',
            opacity: '.15',
            animation: 'oscilate-mirror 15s ease-in-out infinite'
        },
    }
}

export default styleFunction;