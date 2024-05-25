function ExchangeRatePic() {
    const RadixLogo = () => {
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

    const SuperYieldNFTLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/png/tp/white/yield_nft.png"
                    alt="SuperYield"
                />

                <h2>SUPER Yield NFT </h2>

            </span>
        )
    }

    const SuperLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/svg/bg/white/super_s.svg"
                    alt="SuperYieldNFT"
                />

                <h2> 10 SUPER </h2>

            </span>
        )
    }

    const SuperTrustLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/svg/bg/white/super_t.svg"
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