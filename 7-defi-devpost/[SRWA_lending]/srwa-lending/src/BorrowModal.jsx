import Modal from 'react-bootstrap/Modal';
import { useState } from 'react';

function BorrowModal(props) {
  const { show, handleClose } = props;
  const [tabActive, setTabActive] = useState('Borrow');

  return (
    <div className="modal show" style={{ display: 'block', position: 'initial' }}>
      <Modal show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>
            <h5 className="modal-title">Borrow</h5>
          </Modal.Title>
        </Modal.Header>

        <Modal.Body>
          <nav>
            <div className="nav nav-tabs" id="nav-tab">
              <button
                onClick={() => setTabActive('Borrow')}
                className={`nav-link ${tabActive === 'Borrow' ? 'active' : ''}`}
                type="button"
              >
                BORROW
              </button>
              <button
                onClick={() => setTabActive('Repay')}
                className={`nav-link ${tabActive === 'Repay' ? 'active' : ''}`}
                type="button"
              >
                REPAY
              </button>
            </div>
          </nav>

          <div className="tab-content">
            <div className={`tab-pane fade ${tabActive === 'Borrow' ? 'show active' : ''}`}>
              <div className="supplyaypi">
                <div className="row">
                  <div className="col-12 text-center">
                    <div className="block-title smaller">
                      Supply APY: <span className="text-success">+5%</span>
                    </div>
                  </div>
                </div>
              </div>
              <div className="amount-block">
                <div className="row">
                  <div className="col-6 text-center">
                    <div className="amont-value">100.00</div>
                  </div>
                  <div className="col-6 text-center">
                    <div className="amount-label">MAX</div>
                  </div>
                </div>
              </div>
              <div className="balance">
                Wallet balance: <strong>100.00 XRD</strong>
              </div>
              <div className="suplay-ballance">
                <div className="suplay-ballance-label">Supply Balance [XRD]:</div>
                <div className="suplay-ballance-value">1,000 &gt; 1,100</div>
              </div>
              <hr />
              <div className="block-title mb-2 smaller" style={{ color: '#036d9c' }}>
                Change in Loan Limit
              </div>

              <div className="row">
                <div className="col-4">
                  <div className="column-label-simple">Loan:</div>
                  <div className="column-value-simple">$400,000.00</div>
                </div>
                <div className="col-4">
                  <div className="column-label-simple">Current Limit:</div>
                  <div className="column-value-simple">$280,000.00</div>
                </div>

                <div className="col-4">
                  <div className="column-label-simple">New Limit:</div>
                  <div className="column-value-simple">$180,000.00</div>
                </div>
              </div>
              <div className="modal-footer">
                <button type="button" className="btn btn-primary m-auto">
                  BORROW
                </button>
              </div>
            </div>

            <div className={`tab-pane fade ${tabActive === 'Repay' ? 'show active' : ''}`}>
              <div className="supplyaypi">
                <div className="row">
                  <div className="col-12 text-center">
                    <div className="block-title smaller">
                      Supply APY: <span className="text-success">+5%</span>
                    </div>
                  </div>
                </div>
              </div>
              <div className="amount-block">
                <div className="row">
                  <div className="col-6 text-center">
                    <div className="amont-value">100.00</div>
                  </div>
                  <div className="col-6 text-center">
                    <div className="amount-label">MAX</div>
                  </div>
                </div>
              </div>
              <div className="balance">
                Withdrawable amount: <strong>100.00 XRD</strong>
              </div>
              <div className="suplay-ballance">
                <div className="suplay-ballance-label">Supply Balance [XRD]:</div>
                <div className="suplay-ballance-value">1,000 &gt; 1,100</div>
              </div>
              <hr />
              <div className="block-title mb-2 smaller" style={{ color: '#036d9c' }}>
                Change in Loan Limit
              </div>

              <div className="row">
                <div className="col-4">
                  <div className="column-label-simple">Loan:</div>
                  <div className="column-value-simple">$400,000.00</div>
                </div>
                <div className="col-4">
                  <div className="column-label-simple">Current Limit:</div>
                  <div className="column-value-simple">$280,000.00</div>
                </div>

                <div className="col-4">
                  <div className="column-label-simple">New Limit:</div>
                  <div className="column-value-simple">$180,000.00</div>
                </div>
              </div>

              <div className="modal-footer">
                <button type="button" className="btn btn-primary m-auto">
                  REPAY
                </button>
              </div>
            </div>
          </div>
        </Modal.Body>
      </Modal>
    </div>
  );
}

export default BorrowModal;
