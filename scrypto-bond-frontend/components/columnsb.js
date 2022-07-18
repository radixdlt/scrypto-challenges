import { format } from 'date-fns';
import ColumnFilter from './ColumnFilter';

export const COLUMNS = [
  {
    Header: 'Id  ',
    accessor: 'id',
    Footer: 'Id',
    disableFilters: true
  },
  {
    Header: 'Coupon Epoch  ',
    accessor: 'coupon_epoch',
    Footer: 'Coupon Epoch'
  },
  {
    Header: 'Maturity Epoch  ',
    accessor: 'maturity_epoch',
    Footer: 'Maturity Epoch'
  },
  {
    Header: 'Coupon Rate  ',
    accessor: 'coupon_rate',
    Footer: 'Coupon Rate'
  },
  {
    Header: 'Issue Price  ',
    accessor: 'issue_price',
    Footer: 'Issue Price'
  },
  {
    Header: 'Buy',
    Footer: 'Buy',
		//Cell: ({value}) => (<a >Buy</a>)
    //Cell: ({value}) => (<a onClick={()=>{console.log('clicked value', value)}}>Buy</a>)
    Cell: ({value}) => (<button onClick={console.log('clicked value', value)}>Buy</button>)
    
    // Cell: ({ cell }) => (
    //   <button value={cell.row.values.name} onClick={(this.props).handleClick}>
    //     Button
    //   </button>
    // )

  }
  
];
console.log('sub columns: ', COLUMNS);

export const GROUPED_COLUMNS = [
  {
    Header: 'Id',
    accessor: 'id',
    Footer: 'Id'
  },
  {
    Header: 'Info',
    Footer: 'Info',
    columns: [
      { Header: 'Coupon Epoch', accessor: 'coupon_epoch', Footer: 'Coupon Epoch' },
      { Header: 'Maturity Epoch', accessor: 'maturity_epoch', Footer: 'Maturity Epoch' },
      { Header: 'Coupon Rate', accessor: 'coupon_rate', Footer: 'Coupon Rate' },
      { Header: 'Issue Price', accessor: 'issue_price', Footer: 'Issue Price' },
    ]
  }
];
