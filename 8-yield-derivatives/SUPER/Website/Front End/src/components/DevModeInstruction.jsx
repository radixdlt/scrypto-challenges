const DevModeInstruction = () => {
  return (
    <>
      <section className="heading-section">
        <h1>
          Enter Dev Mode<br />
          to Test Super
        </h1>
        <p className="head-text">
          Before connecting your wallet please follow the steps below.
        </p>
      </section> 
      {/* <!-- Heading Section End -->
      <!-- Dev Mode Instructions Start --> */}
      <div className="dev-mode-instruction-container">
        <div className="dev-mode-content-container">
          <div className="dev-mode-steps-col">
            <div className="dev-mode-step-container">
              <p className="step-nums">Step 1</p>
              <h4 className="step-heading">
                Select Dev Mode in the Radix Wallet
              </h4>
              <p className="step-text">
                Open the Radix Wallet, then go to Configurations -{">"} App
                settings -{">"} Dev Mode.
              </p>
            </div>
            <div className="dev-mode-step-container">
              <p className="step-nums">Step 2</p>
              <h4 className="step-heading">Configure the Gateway</h4>
              <p className="step-text">
                Go to App Settings -{">"} Network Gateways -{">"} Add New
                Gateway, add https://babylon-stokenet-gateway.radixdlt.com as a
                gateway and select it.
              </p>
            </div>
            <div className="dev-mode-step-container">
              <p className="step-nums">Step 3</p>
              <h4 className="step-heading">Get Some Test XRD</h4>
              <p className="step-text">
                Once Stokenet Gateway is selected go to any of your accounts,
                click the three dots at the top -{">"} Dev Preferences -{">"}{" "}
                Get XRD Test Tokens.
              </p>
            </div>
            <div className="dev-mode-step-container">
              <p className="step-nums">Step 4</p>
              <h4 className="step-heading">Connect your Radix Wallet</h4>
              <p className="step-text">
                Connect your Radix Wallet in the navigation bar.
              </p>
            </div>
          </div>
          {/* <!-- Dev Mode Gif Start --> */}

          <div className="dev-mode-gif-container">
            <div className="dev-mode-gif">
              <img src="src/assets/dev-mode-setup.gif" alt="dev mode setup" />
            </div>
          </div>

          {/* <!-- Dev Mode Gif End --> */}
        </div>
      </div>
    </>
  );
};

export default DevModeInstruction;
