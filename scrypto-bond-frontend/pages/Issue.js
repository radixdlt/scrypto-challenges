
import React, { useMemo } from 'react';
import { useTable } from 'react-table';
import { COLUMNS } from '../components/columnsi';
import MOCK_DATA from '../components/MOCK_DATAi.json';
import ReactTable from "react-table-6";  
import "react-table-6/react-table.css";

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
    // <div className="table-container">
    //   <h2>Issuer/Seller Table</h2>
    //   <table {...getTableProps()}>
    //     <thead>
    //       {headerGroups.map(headerGroup => (
    //         <tr {...headerGroup.getHeaderGroupProps()}>
    //           {headerGroup.headers.map(column => (
    //             <th {...column.getHeaderProps()}>{column.render('Header')}</th>
    //           ))}
    //         </tr>
    //       ))}
    //     </thead>
    //     <tbody {...getTableBodyProps()}>
    //       {rows.map(row => {
    //         prepareRow(row);

    //         return (
    //           <tr {...row.getRowProps()}>
    //             {row.cells.map(cell => {
    //               return (
    //                 <td {...cell.getCellProps()}>{cell.render('Cell')}</td>
    //               );
    //             })}
    //           </tr>
    //         );
    //       })}
    //     </tbody>
    //     <tfoot>
    //       {footerGroups.map(footerGroup => (
    //         <tr {...footerGroup.getFooterGroupProps()}>
    //           {footerGroup.headers.map(column => (
    //             <td {...column.getFooterProps()}>{column.render('Footer')} </td>
    //           ))}
    //         </tr>
    //       ))}
    //     </tfoot>
    //   </table>
    // </div>

<section className="section position-relative">
      <Container>
        <Row className="align-items-center">
          <Col lg={6}>
            <div className="pr-lg-5">
              <p className="text-uppercase text-primary font-weight-medium f-14 mb-4">Issuer Page</p>
              <h1 className="mb-4 font-weight-normal line-height-1_4">Fill in the form to issue a bond <span className="text-primary font-weight-medium">Name</span></h1>
              <a href="#" className="btn btn-warning">
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
                        //autocomplete="name"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Maturity Epoch</label>
                      <input
                        className="mb-4 border-b-2"
                        id="maturity_epoch"
                        name="maturity_epoch"
                        type="text"
                        //autocomplete="name"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Coupon Rate</label>
                      <input
                        className="mb-4 border-b-2"
                        id="coupon_rate"
                        name="coupon_rate"
                        type="text"
                        //autocomplete="name"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Issue Rate</label>
                      <input
                        className="mb-4 border-b-2"
                        id="issue_rate"
                        name="issue_rate"
                        type="text"
                        //autocomplete="name"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Issue Epoch</label>
                      <input
                        className="mb-4 border-b-2"
                        id="issue_epoch"
                        name="issue_epoch"
                        type="text"
                        //autocomplete="name"
                        required
                      />
                    <label htmlFor="name" className="mb-2 italic">Supply</label>
                      <input
                        className="mb-4 border-b-2"
                        id="supply"
                        name="supply"
                        type="text"
                        //autocomplete="name"
                        required
                      />
                    <button
                      type="submit"
                      className="px-4 py-2 font-bold text-white bg-blue-500 rounded-full hover:bg-blue-700"
                      >
                        Issue
                    </button>
                  </form>
                </div>
              </div> 
          </Col>
        </Row>
      </Container>
    </section>




  );
};

export default IssuerTable;






/*      OLD ISSUER/SELLER PAGE



import React from 'react';
import { Container, Row, Col } from "reactstrap";
const FeatureBox = (props) => {
  return (
    <>
    {
      props.features.map((feature, key) =>
      (feature.id % 2 !== 0) ?
        <Row key={key} className={feature.id === 1 ? "align-items-center" : "align-items-center mt-5"}>
          <Col md={5} >
            <div>
              <img src={feature.img} alt="" className="img-fluid d-block mx-auto"/>
            </div>
          </Col>
            <Col md={{size:6, offset:1}}>
              <div className="mt-5 mt-sm-0 mb-4">
                <div className="my-4">
                  <i className={feature.icon}></i>
                </div>
                <h5 className="text-dark font-weight-normal mb-3 pt-3">{feature.title}</h5>
                <p className="text-muted mb-3 f-15">{feature.desc}</p>
                <a href={feature.link} className="f-16 text-warning">Read More <span className="right-icon ml-2">&#8594;</span></a>
              </div>
            </Col>
        </Row>
      :
      <Row key={key} className="align-items-center mt-5">
        <Col md={6}>
          <div className="mb-4">
            <div className="my-4">
              <i className="mdi mdi-account-group"></i>
            </div>
            <h5 className="text-dark font-weight-normal mb-3 pt-3">{feature.title}</h5>
            <p className="text-muted mb-3 f-15">{feature.desc}</p>
            <a href={feature.link} className="f-16 text-warning">Read More <span className="right-icon ml-2">&#8594;</span></a>
          </div>
        </Col>
        <Col md={{size:5, offset:1}} className="mt-5 mt-sm-0">
          <div>
            <img src={feature.img} alt="" className="img-fluid d-block mx-auto"/>
          </div>
        </Col>
      </Row>
      )
    }
    </>
  );
}
const Feature = () => {
const features = [
    {id : 1, img : "https://mandmarblestone.files.wordpress.com/2012/05/hot-button2.jpg", title : "ISSUER", desc : "*text*", link : "/"},
    {id : 2, img : "https://mandmarblestone.files.wordpress.com/2012/05/hot-button2.jpg", title : "SELLER", desc : "*text*", link : "/"},
  ];
return (
    <section className="section" id="feature">
      <Container>
        <Row className="justify-content-center">
          <Col lg={6} md={8}>
            <div className="title text-center mb-5">
              <h3 className="font-weight-normal text-dark"><span className="text-warning">Header</span></h3>
              <p className="text-muted">*text*</p>
            </div>
          </Col>
        </Row>
        <FeatureBox features={features} />
      </Container>
    </section>
  );
}
export default Feature; 
*/