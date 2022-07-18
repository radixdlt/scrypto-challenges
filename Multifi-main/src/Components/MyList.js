import React from 'react';
import { ListItem, ListItemIcon, ListItemText, ListSubheader, Divider } from '@material-ui/core';
import { DashboardOutlined,  StoreOutlined, ReceiptOutlined, GroupOutlined, CategoryOutlined, MonetizationOnOutlined, WebAsset, NotificationsNone, PhoneAndroid, AddCircleOutlineOutlined, BusinessOutlined, ImportExportOutlined, SyncAltOutlined, AccountBalanceOutlined, CreditCardOutlined, AccountBalanceWalletOutlined } from '@material-ui/icons';

function MyList(props) {
  const { setSection, section } = props;

  return (
    <div>
      <br />
      <ListSubheader inset className='list-item-header'>Dashboard</ListSubheader>
      <ListItem button onClick={() => setSection('Dashboard')} className={section === 'Dashboard' ? 'highlight' : ''}>
        <ListItemIcon>
          <DashboardOutlined />
        </ListItemIcon>
        <ListItemText primary="Dashboard" />
      </ListItem>
      <ListSubheader inset className='list-item-header'>Bank</ListSubheader>
      <ListItem button onClick={() => setSection('Deposit')} className={section === 'Deposit' ? 'highlight' : ''}>
        <ListItemIcon>
          <AccountBalanceOutlined />
        </ListItemIcon>
        <ListItemText primary="Deposit" />
      </ListItem>
      <ListItem button onClick={() => setSection('Borrow')} className={section === 'Borrow' ? 'highlight' : ''}>
        <ListItemIcon>
          <CreditCardOutlined />
        </ListItemIcon>
        <ListItemText primary="Borrow" />
      </ListItem>
      <ListItem button onClick={() => setSection('Stake')} className={section === 'Stake' ? 'highlight' : ''}>
        <ListItemIcon>
          <AccountBalanceWalletOutlined />
        </ListItemIcon>
        <ListItemText primary="Stake" />
      </ListItem>
      <ListSubheader inset className='list-item-header'>Utilities</ListSubheader>
      <ListItem button onClick={() => setSection('Bridge')} className={section === 'Bridge' ? 'highlight' : ''} id="polyBridge">
        <ListItemIcon>
          <SyncAltOutlined />
        </ListItemIcon>
        <ListItemText primary="Bridge" />
      </ListItem>
    </div>
  )
}

export default MyList;