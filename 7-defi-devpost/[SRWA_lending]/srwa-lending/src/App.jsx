import React, { useState } from 'react';
import connectImg from './assets/connect.svg';
import logoImg from './assets/logo.svg';
import 'bootstrap/dist/css/bootstrap.min.css';
import './App.css';
import SupplyModal from './SupplyModal.jsx';
import BorrowModal from './BorrowModal.jsx';

function App() {
  const [showSupplyModal, setShowSupplyModal] = useState(false);
  const [showBorrowModal, setShowBorrowModal] = useState(false);

  const handleSupplyModalClose = () => setShowSupplyModal(false);
  const handleSupplyModalShow = () => setShowSupplyModal(true);
  const handleBorrowModalClose = () => setShowBorrowModal(false);
  const handleBorrowModalShow = () => setShowBorrowModal(true);

  return (
    <>
      <div className="d-flex">
        <div className="app-contaienr d-flex w-100 p-3 mx-auto flex-column">
          <header className="mb-3">
            <div>
              <div className="row">
                <div className="col-md-3 text-center d-flex">
                  <div className="header-corner m-auto">
                    <div className="logo">
                      <a href="/">
                        <img src={logoImg} alt="Logo img" />
                      </a>
                    </div>
                    <div className="connectbutton mb-3 mt-4">
                      <radix-connect-button></radix-connect-button>
                    </div>
                  </div>
                </div>

                <div className="col-md-9">
                  <div className="block-section white-label">
                    <div className="block-title mb-3">TVL</div>
                    <div className="row">
                      <div className="col-md-3">
                        <div className="column-label">Total Supply</div>
                        <div className="column-value">$400,000.00</div>
                      </div>
                      <div className="col-md-3">
                        <div className="column-label">Total Borrow</div>
                        <div className="column-value">$280,000.00</div>
                      </div>
                      <div className="col-md-3">
                        <div className="column-label">Total Liquidity</div>
                        <div className="column-value">$120,000.00</div>
                      </div>
                      <div className="col-md-3">
                        <div className="column-label">Total Treasury</div>
                        <div className="column-value">$1.000.00</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </header>

          <main className="px-0">
            <div className="row mb-3">
              <div className="col-12">
                <div className="block-section">
                  <div className="block-title mb-3">My Account</div>
                  <div className="row">
                    <div className="col-md-3 col-6">
                      <div className="column-label" style={{ background: '#f0f8fa' }}>
                        Net APY
                      </div>
                      <div className="column-value">-1.88%</div>
                    </div>
                    <div className="col-md-3 col-6">
                      <div className="column-label" style={{ background: '#f0f8fa' }}>
                        Daily
                      </div>
                      <div className="column-value">$-3.87</div>
                    </div>
                  </div>
                  <hr className="mb-4 opacity-01" />
                  <div className="row">
                    <div className="col-md-3">
                      <div className="column-label">Supply Balance</div>
                      <div className="column-value">$400,000.00</div>
                    </div>
                    <div className="col-md-3">
                      <div className="column-label">Borrow Balance</div>
                      <div className="column-value">$280,000.00</div>
                    </div>
                    <div className="col-md-3">
                      <div className="column-label">Borrow Limit</div>
                      <div className="column-value">$120,000.00</div>
                    </div>
                    <div className="col-md-3">
                      <div className="column-label">Borrow Limit Used</div>
                      <div className="column-value">50.00%</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div className="row mb-3">
              <div className="col-12">
                <div className="block-section">
                  <div className="block-title mb-3">Positions</div>
                  <div className="row">
                    <div className="col-md-6 pr-3">
                      <div className="block-title mb-3 smaller" style={{ color: '#036d9c' }}>
                        Supplied
                      </div>
                      <div className="row supplied-box">
                        <div className="col-md-3">
                          <div className="column-label">Asset</div>
                          <div
                            onClick={handleSupplyModalShow}
                            className="column-value cursor-pointer"
                          >
                            XRD
                          </div>
                        </div>
                        <div className="col-md-3">
                          <div className="column-label">APY</div>
                          <div
                            onClick={handleSupplyModalShow}
                            className="column-value cursor-pointer"
                          >
                            7.90%
                          </div>
                        </div>
                        <div className="col-md-5">
                          <div className="column-label">Balance</div>
                          <div
                            onClick={handleSupplyModalShow}
                            className="column-value cursor-pointer"
                          >
                            $75,000.00 <small>(1.875M XRD)</small>
                          </div>
                        </div>
                      </div>
                    </div>
                    <div className="col-md-6 borrowing-box">
                      <div className="block-title mb-3 smaller" style={{ color: '#cd4d4f' }}>
                        Borrowing
                      </div>
                      <div className="row">
                        <div className="col-md-3">
                          <div className="column-label">USDA</div>
                          <div
                            onClick={handleBorrowModalShow}
                            className="column-value cursor-pointer"
                          >
                            $400,000.00
                          </div>
                        </div>
                        <div className="col-md-2">
                          <div className="column-label">APY</div>
                          <div
                            onClick={handleBorrowModalShow}
                            className="column-value cursor-pointer"
                          >
                            9.78%
                          </div>
                        </div>
                        <div className="col-md-5">
                          <div className="column-label">Borrow Limit</div>
                          <div
                            onClick={handleBorrowModalShow}
                            className="column-value cursor-pointer"
                          >
                            $30,000.00 <small>(30,000.00 USDA) </small>
                          </div>
                        </div>
                        <div className="col-md-2">
                          <div className="column-label">% of Limit</div>
                          <div
                            onClick={handleBorrowModalShow}
                            className="column-value cursor-pointer"
                          >
                            50.00%
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div className="row mb-0">
              <div className="col-12">
                <div className="block-section market">
                  <div className="block-title mb-3">Market</div>

                  <div className="row">
                    <div className="col-1">
                      <div className="column-label">Asset</div>
                      <div className="column-value cursor-pointer" onClick={handleSupplyModalShow}>
                        XRD
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleSupplyModalShow}>
                        USDA
                      </div>
                    </div>

                    <div className="col">
                      <div className="column-label">Total Supply</div>
                      <div className="column-value cursor-pointer" onClick={handleSupplyModalShow}>
                        $300k <small>(7.5M XRD)</small>
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleSupplyModalShow}>
                        $100k <small>(100k USDA)</small>
                      </div>
                    </div>

                    <div className="col">
                      <div className="column-label">Supply APY</div>
                      <div className="column-value cursor-pointer" onClick={handleSupplyModalShow}>
                        7.90%
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleSupplyModalShow}>
                        8.00%
                      </div>
                    </div>

                    <div className="col">
                      <div className="column-label">Total Borrow</div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        $300k <small>(7.5M XRD)</small>
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        $100k <small>(100k USDA)</small>
                      </div>
                    </div>

                    <div className="col">
                      <div className="column-label">Supply APY</div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        -9.00%
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        -9.78%
                      </div>
                    </div>

                    <div className="col">
                      <div className="column-label">Liquidity</div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        $300k <small>(7.5M XRD)</small>
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        $100k <small>(100k USDA)</small>
                      </div>
                    </div>

                    <div className="col-2">
                      <div className="column-label">Colaretalization Factor</div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        80%
                      </div>
                      <div className="column-value cursor-pointer" onClick={handleBorrowModalShow}>
                        80%
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </main>
        </div>
        <footer className="mt-auto"></footer>
      </div>
      <SupplyModal show={showSupplyModal} handleClose={handleSupplyModalClose} />
      <BorrowModal show={showBorrowModal} handleClose={handleBorrowModalClose} />
    </>
  );
}

export default App;
