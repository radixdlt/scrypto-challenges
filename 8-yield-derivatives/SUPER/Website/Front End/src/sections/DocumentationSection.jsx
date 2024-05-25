function DocumentationSection() {

    return (
    // <!-- Explore the Docs Start -->
    <div className="heading-section">
      <h2>Explore further Documentation</h2>
      <p className="head-text semi-bold">
        Find additional resources and detailed guides to help you navigate the
        setup process
      </p>
      <div className="docs-button-container">
        <a
          href="https://docs.radixdlt.com/docs"
          className="btn-radix-blue"
          target="_blank"
          rel="noreferrer"
        >
          View Radix Docs
        </a>
        <a
          href="https://www.npmjs.com/package/@radixdlt/radix-dapp-toolkit"
          className="btn-dark"
          target="_blank"
          rel="noreferrer"
        >
          View dApp Toolkit
        </a>
      </div>
    </div>
    // <!-- Explore the Docs End -->
  );
}

export default DocumentationSection;
