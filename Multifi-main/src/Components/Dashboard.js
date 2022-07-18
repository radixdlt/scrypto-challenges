import React, { useState, useEffect } from 'react';
import clsx from 'clsx';
import Web3 from 'web3';
import { CssBaseline, Drawer, Box, AppBar, Toolbar, List, Typography, Divider, IconButton, Badge, Container, Grid, Paper, ListItem, ListItemIcon, Button, ListItemText, Avatar } from '@material-ui/core';
import { Menu, ChevronLeft, PowerSettingsNew } from '@material-ui/icons';
import { useStyles } from './styles';
import MyList from "./MyList";
import metamaskIcon from "../Assets/metamask.svg";

import Dai from '../Abis/Dai.json'
import MDai from '../Abis/MDai.json'
import MLTToken from '../Abis/MLTToken.json'
import Market from '../Abis/Market.json'

import { daiA, mDaiA, mltA, marketA } from '../addresses';

import Balances from './Balances';
import Deposit from './Deposit';
import Borrow from './Borrow';
import Stake from './Stake';
import Bridge from './Bridge';

export default function Dashboard() {
  const classes = useStyles();

  const [account, setAccount] = useState(null);
  
  const [dai, setDai] = useState(null);
  const [mDai, setMDai] = useState(null);
  const [multi, setMulti] = useState(null);
  const [market, setMarket] = useState(null);
  const [section, setSection] = useState('Dashboard');

  const [open, setOpen] = useState(true);
  const handleDrawerOpen = () => {
    setOpen(true);
  };
  const handleDrawerClose = () => {
    setOpen(false);
  };

  const loadBlockchainData = async () => {
    const web3 = window.web3;

    // Load account
    const accounts = await web3.eth.getAccounts();
    setAccount(accounts[0]);    
    
    // Contracts
    const daiC = new web3.eth.Contract(Dai, daiA);
    setDai(daiC);
    const mdaiC = new web3.eth.Contract(MDai, mDaiA);
    setMDai(mdaiC);
    const mltc = new web3.eth.Contract(MLTToken, mltA);
    setMulti(mltc);
    const marketc = new web3.eth.Contract(Market, marketA);
    setMarket(marketc);
  }

  const loadWeb3 = async () => {
    if (window.ethereum) {
      window.web3 = new Web3(window.ethereum)

      window.ethereum.on('accountsChanged', function () {
        loadBlockchainData();
      })
    }
    else if (window.web3) {
      window.web3 = new Web3(window.web3.currentProvider)
      
      window.ethereum.on('accountsChanged', function () {
        loadBlockchainData();
      })
    }
    else {
      window.alert('Non-Ethereum browser detected. You should consider trying MetaMask!')
    }
  }

  const connectToMetaMask = async () => {
    const accounts = await window.ethereum.request({ method: 'eth_requestAccounts'});
    setAccount(accounts[0]);
  }
 
  useEffect(() => {
    const Load = async () => {
      await loadWeb3();
      await loadBlockchainData();
    }
    Load();
  }, [account])

  return (
    <div className={classes.root}>
      <CssBaseline />
      <AppBar position="absolute" className={clsx(classes.appBar, open && classes.appBarShift)}>
        <Toolbar className={classes.toolbar}>
          <IconButton
            edge="start"
            color="inherit"
            aria-label="open drawer"
            onClick={handleDrawerOpen}
            className={clsx(classes.menuButton, open && classes.menuButtonHidden)}
          >
            <Menu />
          </IconButton>
          <Typography component="h1" variant="h6" color="inherit" noWrap className={classes.title} id="mainTitle">
            MultiFi
          </Typography>
          <Button
            onClick={() => connectToMetaMask()}
            variant="contained"
            className="nav-link color-border"
            style={{ marginRight: "20px", background: 'transparent', color: "#fff"  }} 
            //  style={{ fontSize: "0.9rem", letterSpacing: "0.14rem" }}
          >
          <img
            src={metamaskIcon}
            alt="metamask-icon"
            style={{ width: "1.8rem", marginRight: "0.5rem", padding: "3px" }}
          />
          {account ? account && account.substring(0,4)+'...'+account.substring(account.length-3,account.length) : `Connect  `}
        </Button>
        </Toolbar>
      </AppBar>
      <Drawer
        variant="permanent"
        classes={{
          paper: clsx(classes.drawerPaper, !open && classes.drawerPaperClose),
        }}
        open={open}
      >
        <div className={classes.toolbarIcon}>
          <IconButton onClick={handleDrawerClose}>
            <ChevronLeft />
          </IconButton>
        </div>
        <Divider />
        <List>
          <MyList setSection={setSection} section={section} />
        </List>
      </Drawer>
      <main className={classes.content}>
        <div className={classes.appBarSpacer} />
          <Container maxWidth="lg" className={classes.container}>
            
          {
            section === 'Dashboard' && 
            <Balances 
              account={account} 
              dai={dai} 
              mDai={mDai} 
              multi={multi} 
              market={market} 
            />
          }
          {
            section === 'Deposit' && 
            <Deposit
              account={account} 
              dai={dai} 
              mDai={mDai} 
              multi={multi} 
              market={market}
            />
          }
          {
            section === 'Borrow' && 
            <Borrow
              account={account} 
              dai={dai} 
              mDai={mDai} 
              multi={multi} 
              market={market}
            />
          }
          {
            section === 'Stake' && 
            <Stake
              account={account} 
              dai={dai} 
              mDai={mDai} 
              multi={multi} 
              market={market}
            />
          }
          {
            section === 'Bridge' && 
            <Bridge />
          }
        </Container>
      </main>
    </div>
  );
}