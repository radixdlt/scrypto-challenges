
import React, { useMemo } from 'react';
import { useTable } from 'react-table';
import { COLUMNS } from '../components/columnsi';
import MOCK_DATA from '../components/MOCK_DATAi.json';
import ReactTable from "react-table-6";  
import "react-table-6/react-table.css";

import Header from "./Header";

import { Container, Row, Col } from 'reactstrap';


const IssuerTable = props => {
  const columns = useMemo(() => COLUMNS, []);
  const data = useMemo(() => MOCK_DATA, []);

  const {
    getTableProps,
    getTableBodyProps,
    headerGroups,
    footerGroups,
    rows,
    prepareRow
  } = useTable({ columns, data });

  return (
      <Container>
        <Header />
        <Row className="align-items-center">
          <Col lg={6}>
            <div className="pr-lg-5">
              <p className="text-uppercase text-primary font-weight-medium f-14 mb-4">Issuer Page</p>
              <h1 className="mb-4 font-weight-normal line-height-1_4">Fill in the form to issue a <span className="text-primary font-weight-medium">bond</span></h1>
              <a href="/" className="btn btn-dark">
                Sell or Buy instead <span className="ml-2 right-icon">&#8594;</span>
              </a>
            </div>
          </Col>
          <Col lg={6}>
          
            <div className="max-w-xs my-2 overflow-hidden rounded shadow-lg">
              <div className="px-6 py-4">
                <div className="mb-2 text-xl font-bold">Issue a bond</div>
                  <form className="flex flex-col">
                    <label htmlFor="name" className="mb-2 italic">Coupon Epoch</label>
                      <input
                        className="mb-4 border-b-2"
                        id="coupon_epoch"
                        name="coupon_epoch"
                        type="text"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Maturity Epoch</label>
                      <input
                        className="mb-4 border-b-2"
                        id="maturity_epoch"
                        name="maturity_epoch"
                        type="text"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Coupon Rate</label>
                      <input
                        className="mb-4 border-b-2"
                        id="coupon_rate"
                        name="coupon_rate"
                        type="text"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Issue Rate</label>
                      <input
                        className="mb-4 border-b-2"
                        id="issue_rate"
                        name="issue_rate"
                        type="text"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Issue Epoch</label>
                      <input
                        className="mb-4 border-b-2"
                        id="issue_epoch"
                        name="issue_epoch"
                        type="text"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Supply</label>
                      <input
                        className="mb-4 border-b-2"
                        id="supply"
                        name="supply"
                        type="text"
                        required
                      />
                    <button
                      type="submit"
                      className="px-4 py-2 font-bold text-white bg-dark rounded-full hover:bg-blue-700"
                      >
                        Issue
                    </button>
                  </form>
                </div>
              </div> 
          </Col>
        </Row>
      </Container>
  );
};

export default IssuerTable;
