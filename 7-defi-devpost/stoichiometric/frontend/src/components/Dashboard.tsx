/** @jsxImportSource theme-ui */
import { useContext, useEffect, useState } from "react";

import { NavLink } from "react-router-dom";

import { ResponsiveContext } from 'contexts/ResponsiveContext';
import { BurgerContext } from 'contexts/BurgerContext';
import { UserContext } from 'contexts/UserContext';


import ConnectWallet from 'components/ConnectWallet';

export interface Props {
    children?: any;
    page: string;
}

const pages = ["swap", "liquidity", "lend", "liquidate", "dao", "card"];

const Dashboard: React.FC<Props> = (props) => {
    const { device } = useContext(ResponsiveContext);
    const { burgerOpen, toggleBurger } = useContext(BurgerContext);

    const { user } = useContext(UserContext);

    const style = {
        navlink: {
            zIndex: '1000',
            textDecoration: 'none',
        },

        navlinkContainer: {
            '&:nth-child(2)': {
                marginTop: '70px'
            },
        },

        mobileNavlinkContainer: {

        },

        top: {
            position: 'absolute' as 'absolute',
            left: '0',
            top: '0',

            width: 'calc(100% - 20px - 30px)',
            borderRadius: '10px',
            background: 'background2',

            margin: '10px 10px',
            padding: '15px',

            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',

            boxShadow: '0 1px 4px',

            color: 'shadow',
            zIndex: '2000',
        },

        row: {
            display: 'flex',
            alignItems: 'center',
            width: '100%',
            position: 'relative' as 'relative'
        },

        mobileMenu: {
            zIndex: "2000",

            width: 'calc(100% - 30px)',
            height: `${burgerOpen ? 'auto' : '0'}`,
            overflow: 'hidden',
            borderRadius: '0 0 10px 10px',
            background: 'background2',

            opacity: `${burgerOpen ? '1' : '0'}`,

            transition: 'opacity .1s',

            padding: `${burgerOpen ? '15px' : '0 15px'}`,

            display: 'flex',
            alignItems: 'center',

            justifyContent: 'space-between',
        },

        left: {
            position: 'absolute' as 'absolute',
            left: '0',
            top: '0',

            height: 'calc(100% - 60px - 30px)',
            borderRadius: '10px',
            background: 'background2',

            margin: '30px 20px',
            padding: '15px',

            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            boxShadow: '0 1px 4px',
            color: 'shadow'
        },

        beaker: {
            fontFamily: 'primary',
            textDecoration: "none",
            fontSize: 4,
            color: 'text',
            fontWeight: '600',

            margin: '0',
            padding: '0'
        },

        beakerTablet: {
            fontFamily: 'primary',
            textDecoration: "none",
            fontSize: 3,
            color: 'text',
            fontWeight: '600',

            margin: '0',
            padding: '0'
        },

        fi: {
            fontFamily: 'primary',
            fontSize: 1,
            color: 'primary',

            margin: '0',
            padding: '0'
        },

        linkContainer: {
            height: '45px',
            minHeight: '45px',
            aspectRatio: '1',
            borderRadius: '1000px',
            position: 'relative' as 'relative',
            marginBottom: '50px',
            '&:nth-child(2)': {
                marginTop: '70px'
            },
            '&:hover div': {
                opacity: '1 !important'
            },
            '&:hover span': {
                opacity: '.8'
            }
        },

        mobileLinkContainer: {
            height: '40px',
            aspectRatio: '1',
            borderRadius: '1000px',
            position: 'relative' as 'relative',
            '&:hover div': {
                opacity: '1 !important'
            },
            '&:hover span': {
                opacity: '.8'
            }
        },

        active: {
            background: 'primary',
        },

        link: {
            position: 'absolute' as 'absolute',
            left: '50%',
            top: '50%',
            transform: 'Translate(-50%, -50%)',

            background: 'text',
            height: '22px',
            aspectRatio: '1',
            opacity: '.3',

            margin: '0',
            padding: '0',

            zIndex: '5'
        },

        mobileLink: {
            position: 'absolute' as 'absolute',
            left: '50%',
            top: '50%',
            transform: 'Translate(-50%, -50%)',

            background: 'text',
            height: '19px',
            aspectRatio: '1',
            opacity: '.3',

            margin: '0',
            padding: '0',

            zIndex: '5'
        },

        linkActive: {
            opacity: '1 !important',
            background: 'white'
        },

        bottom: {
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '30px',
            transform: 'TranslateX(-50%)',
            zIndex: "2000"
        },

        burgerMenu: {
            width: '22px',
            height: '15px',
            position: 'relative' as 'relative',
            '-webkit-transform': 'rotate(0deg)',
            '-moz-transform': 'rotate(0deg)',
            '-o-transform': 'rotate(0deg)',
            transform: 'rotate(0deg)',
            cursor: 'pointer',
            opacity: '.6',
            marginRight: '10px',
            '&:hover': {
                opacity: '1'
            },

            '& span': {
                display: 'block',
                position: 'absolute' as 'absolute',
                height: '3px',
                width: '100%',
                background: 'text',
                borderRadius: '1000px',
                opacity: '1',
                left: '0',
                '-webkit-transform': 'rotate(0deg)',
                '-moz-transform': 'rotate(0deg)',
                '-o-transform': 'rotate(0deg)',
                transform: 'rotate(0deg)',
                '-webkit-transition': '.25s ease-in-out',
                '-moz-transition': '.25s ease-in-out',
                '-o-transition': '.25s ease-in-out',
                transition: '.25s ease-in-out',

                '&:nth-of-type(1)': {
                    top: `${burgerOpen ? "50%" : "0"}`,
                    width: `${burgerOpen ? "0%" : "100%"}`,
                    left: `${burgerOpen ? "50%" : ""}`,
                    transform: 'TranslateY(-50%)'
                },
                '&:nth-of-type(2)': {
                    top: '50%',
                    '-webkit-transform': `${burgerOpen ? 'TranslateY(-50%) rotate(45deg)' : 'TranslateY(-50%)'}`,
                    '-moz-transform': `${burgerOpen ? 'TranslateY(-50%) rotate(45deg)' : 'TranslateY(-50%)'}`,
                    '-o-transform': `${burgerOpen ? 'TranslateY(-50%) rotate(45deg)' : 'TranslateY(-50%)'}`,
                    transform: `${burgerOpen ? 'TranslateY(-50%) rotate(45deg)' : 'TranslateY(-50%)'}`,
                },
                '&:nth-of-type(3)': {
                    top: '50%',
                    '-webkit-transform': `${burgerOpen ? 'TranslateY(-50%) rotate(-45deg)' : 'TranslateY(-50%)'}`,
                    '-moz-transform': `${burgerOpen ? 'TranslateY(-50%) rotate(-45deg)' : 'TranslateY(-50%)'}`,
                    '-o-transform': `${burgerOpen ? 'TranslateY(-50%) rotate(-45deg)' : 'TranslateY(-50%)'}`,
                    transform: `${burgerOpen ? 'TranslateY(-50%) rotate(-45deg)' : 'TranslateY(-50%)'}`,
                },
                '&:nth-of-type(4)': {
                    top: `${burgerOpen ? "50%" : "100%"}`,
                    width: `${burgerOpen ? "0%" : "100%"}`,
                    left: `${burgerOpen ? "50%" : ""}`,
                    transform: 'TranslateY(-50%)'
                },
            }
        },

        right: {
            position: 'absolute' as 'absolute',
            right: '0px',
            top: '50%',
            transform: 'TranslateY(-50%)',
            zIndex: '4000'
        },
    }

    function icon(x: string) {
        return (
            {
                "-webkit-mask": `url('/pages/${x}.svg') center/contain no-repeat`,
                mask: `url('/pages/${x}.svg') center/contain no-repeat`,
            })
    }

    return (
        <div>
            <div>
                {device == "mobile" ?
                    <div>
                        <div sx={style.top}>
                            <div sx={style.row}>
                                <div sx={style.burgerMenu} onClick={toggleBurger}>
                                    <span></span>
                                    <span></span>
                                    <span></span>
                                    <span></span>
                                </div>
                                <NavLink sx={style.navlink} to={"/"}>
                                    <h1 sx={style.beaker}>
                                        Stoic<span sx={style.fi}>.fi</span>
                                    </h1>
                                </NavLink>
                                <div sx={style.right}>
                                    <ConnectWallet />
                                </div>
                            </div>
                            <div sx={style.mobileMenu}>
                                {
                                    pages.map((x, index) => {
                                        if (x == props.page) return (
                                            <div key={"dashboard_mobile" + index} sx={{ ...style.mobileLinkContainer, ...style.active }}>
                                                <div sx={{ ...style.mobileLink, ...style.linkActive, ...icon(x) }} />
                                            </div>
                                        )
                                        else {
                                            return (
                                                <div key={"dashboard_mobile" + index} sx={style.mobileNavlinkContainer}>
                                                    <NavLink to={`/${x}`} sx={style.navlink}>
                                                        <div sx={style.mobileLinkContainer}>
                                                            <div sx={{ ...style.mobileLink, ...icon(x) }} />
                                                        </div>
                                                    </NavLink>
                                                </div>
                                            )
                                        }
                                    })
                                }
                            </div>
                        </div>
                    </div>
                    : <div sx={style.left}>
                        <NavLink sx={style.navlink} to={"/"}>
                            {device == "laptop" || device == "desktop" ?
                                <h1 sx={style.beaker}>
                                    Stoic<span sx={style.fi}>.fi</span>
                                </h1>
                                : <h1 sx={style.beaker}><span sx={style.beakerTablet}>Stc</span></h1>
                            }
                        </NavLink>
                        {
                            pages.map((x, index) => {
                                if (x == props.page) return (
                                    <div key={"dashboard" + index} sx={{ ...style.linkContainer, ...style.active }}>
                                        <div sx={{ ...style.link, ...style.linkActive, ...icon(x) }} />
                                    </div>
                                )
                                else {
                                    return (
                                        <div key={"dashboard" + index} sx={style.navlinkContainer}>
                                            <NavLink to={`/${x}`} sx={style.navlink}>
                                                <div sx={style.linkContainer}>
                                                    <div sx={{ ...style.link, ...icon(x) }} />
                                                </div>
                                            </NavLink>
                                        </div>
                                    )
                                }
                            })
                        }
                        <div sx={style.bottom}>
                            <ConnectWallet />
                        </div>
                    </div>
                }
            </div>
            {props.children}
        </div>
    )
}

export default Dashboard;