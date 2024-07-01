/**
 * ExchangeRatePic component visually represents the exchange rate formula for converting XRD
 * to SUPER tokens and SUPER NFTs. It includes sub-components to display the Radix token,
 * SUPER token, SUPERt token, and SUPER Yield NFT.
 *
 * @returns {JSX.Element} The rendered ExchangeRatePic component.
 */
function ExchangeRatePic() {

    /**
     * RadixLogo component displays the Radix token image and its value.
     *
     * @returns {JSX.Element} The rendered RadixLogo component.
     */    const RadixLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets-global.website-files.com/6053f7fca5bf627283b582c2/6266da23999171a63bcbb2a7_Radix-Icon-Round-200x200.svg"
                    alt="Radix Token"
                />

                <h2> 10 XRD </h2>

            </span>
        );
    };

    /**
     * SuperYieldNFTLogo component displays the SUPER Yield NFT image and its value.
     *
     * @returns {JSX.Element} The rendered SuperYieldNFTLogo component.
     */
    const SuperYieldNFTLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/png/bg/yield_nft.png"
                    alt="SuperYield"
                />

                <h2>SUPER Yield NFT </h2>

            </span>
        )
    }

    /**
     * SuperLogo component displays the SUPER token image and its value.
     *
     * @returns {JSX.Element} The rendered SuperLogo component.
     */
    const SuperLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/svg/bg/super_s.svg"
                    alt="SuperYieldNFT"
                />

                <h2> 10 SUPER </h2>

            </span>
        )
    }

    /**
     * SuperTrustLogo component displays the SUPERt token image and its value.
     *
     * @returns {JSX.Element} The rendered SuperTrustLogo component.
     */
    const SuperTrustLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/svg/bg/super_t.svg"
                    alt="SuperYield"
                />

                <h2>6 SUPERt</h2>

            </span>

        )
    }

    return (
        <span className="ExchangeRateFormula">

            <RadixLogo/>
            <h2 className='exchange-symbols'>=</h2>
            <SuperLogo/>
            <h2 className='exchange-symbols'>+</h2>
            <SuperTrustLogo/>
            <h2 className='exchange-symbols'>+</h2>
            <SuperYieldNFTLogo/>

        </span>
    )

}

export default ExchangeRatePic;