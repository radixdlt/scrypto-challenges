
import React, { useMemo } from 'react';
import { useTable } from 'react-table';
import { COLUMNS } from '../components/columnsb';
import MOCK_DATA from '../components/MOCK_DATAb.json';
import ReactTable from "react-table-6";  
import "react-table-6/react-table.css";
import Header from "./Header";


const BuyerTable = props => {
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
    <div className="table-container">
            <Header />
            <div className="pr-lg-5">
              <p className="text-uppercase text-primary font-weight-medium f-14 mb-4">Buyer Page</p>
              <h1 className="mb-4 font-weight-normal line-height-1_4">Choose a <span className="text-primary font-weight-medium">bond</span> to buy from the table below</h1>
              <a href="/" className="btn btn-dark">
                Issue or Sell instead <span className="ml-2 right-icon">&#8594;</span>
              </a>
            </div>
      <h2> </h2>
      <table {...getTableProps()}>
        <thead>
          {headerGroups.map(headerGroup => (
            <tr {...headerGroup.getHeaderGroupProps()}>
              {headerGroup.headers.map(column => (
                <th {...column.getHeaderProps()}>{column.render('Header')}</th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody {...getTableBodyProps()}>
          {rows.map(row => {
            prepareRow(row);

            return (
              <tr {...row.getRowProps()}>
                {row.cells.map(cell => {
                  return (
                    <td {...cell.getCellProps()}>{cell.render('Cell')}</td>
                  );
                })}
              </tr>
            );
          })}
        </tbody>
        
      </table>
    </div>
  );
};

export default BuyerTable;
