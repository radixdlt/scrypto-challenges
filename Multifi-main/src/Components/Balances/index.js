import React, { useState, useEffect } from 'react';
import { makeStyles } from '@material-ui/core/styles';
import { Paper, Typography, Grid, Button } from '@material-ui/core';

import Loading from '../Loading';

import polygonIcon from "../../Assets/polygon.svg";
import daiIcon from "../../Assets/dai.svg";
import maticIcon from "../../Assets/matic.svg";
import mDaiIcon from "../../Assets/mDai.svg";
import mIcon from "../../Assets/m.svg";

import { convertFromWei } from '../helper';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
    marginTop: '25px',
    cursor: 'default',
  },
  paper: {
    padding: theme.spacing(1),
    textAlign: 'center',
    color: theme.palette.text.secondary,
  },
}));

function Balances(props) {
  const { account, dai, mDai, multi, market } = props;
  
  const [isLoading, setLoading] = useState(true);
  
  const [mSize, setMSize] = useState(null);
  const [totalB, setTB] = useState(null);
  const [cR, setCR] = useState(null);

  const [withdrawable, setWithdrawable] = useState(null);
  const [withdrawn, setWithdrawn] = useState(null);
  const [deposited, setDeposited] = useState(null);
  const [borrowed, setBorrowed] = useState(null);
  
  const [daiB, setDaiB] = useState(null);
  const [maticB, setMaticB] = useState(null);
  const [mltB, setMltB] = useState(null);
  const [mDaiB, setMDaiB] = useState(null);

  const [daiP, setDaiP] = useState(null);
  const [maticP, setMaticP] = useState(null);

  const fetchData = async () => {
    if(market) {
      setMSize(await market.methods.marketSizeDai().call());
      setTB(await market.methods.totalBorrowedDai().call());
      setCR(await market.methods.maticCollateralFactor().call());

      setDaiP(await market.methods.getDAIUSDPrice().call());
      setMaticP(await market.methods.getMATICUSDPrice().call());
    }

    if(account && account !== '' && dai && mDai && multi) {
      setDaiB(await dai.methods.balanceOf(account).call());
      const mDaiT = await mDai.methods.balanceOf(account).call();
      setMDaiB(mDaiT);
      if(market && mDaiT) {
        setWithdrawable(await market.methods.estimateWithdrawAmount(mDaiT).call());
      }
      setMltB(await multi.methods.balanceOf(account).call());
      setMaticB(await window.web3.eth.getBalance(account));
    }

    if(market && account && dai && mDai) {
      const vault = await market.methods.getVault(account).call();

      setDeposited(vault.depositAmount);
      setBorrowed(vault.debtAmount);
      setWithdrawn(vault.withdrawnAmount);
    }

    setLoading(false);
  }

  useEffect(() => {
    fetchData();
  }, [account])

  const classes = useStyles();

  return(
    <>
       {
        isLoading ? <Loading /> :
        <>
          <div className={classes.root}>
            <Grid container spacing={4}>
              {/* Pool Stats */}
              <Grid item xs={12}>
                <Paper className={classes.paper} style={{ background: 'rgb(56, 61, 81)', color: 'rgb(255, 255, 255)' }}>
                  <Grid container spacing={3}>
                    <Grid item xs={12} sm={3}>
                      <p className='poolStatsTitle'>Total Market Size</p>
                      <p className='poolStatsSubTitle'>{mSize && convertFromWei(mSize)} DAI</p>
                    </Grid>
                    <Grid item xs={12} sm={3}>
                      <p className='poolStatsTitle'>Total Borrowed</p>
                      <p className='poolStatsSubTitle'>{totalB && convertFromWei(totalB)} DAI</p>
                    </Grid>
                    <Grid item xs={12} sm={3}>
                      <p className='poolStatsTitle'>Collateral Rate</p>
                      <p className='poolStatsSubTitle'>{cR}%</p>
                    </Grid>
                    <Grid item xs={12} sm={3}>
                      <Button
                        variant="contained"
                        // className="nav-link color-border"
                        disabled
                        style={{ background: '#fff', color: "#000", padding: '0 0 0 10px'
                        , borderRadius: 0, textTransform: 'none', border: '1px solid #fff', marginTop: '15px'
                        }} 
                      >
                        <div>
                          <span id="marketTitle">market</span>
                          {/* <span id="marketSubTitle">market</span> */}
                        </div>
                        <img
                          src={polygonIcon}
                          alt="polygon-icon"
                          style={{ width: "3.2rem", margin: "0 0 0 0.5rem" }}
                        />
                      </Button>
                    </Grid>
                  </Grid>
                </Paper>
              </Grid>

              {/* Overview */}
              <Grid item xs={12}>
                <Typography component="h1" variant="h6" color="inherit" noWrap>
                  Overview
                </Typography>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <div className="overview-data">
                    <p>{withdrawable && convertFromWei(withdrawable,5).toFixed(5)} DAI</p>
                    <p>${withdrawable && daiP && (convertFromWei(withdrawable,5)*convertFromWei(daiP)).toFixed(5)}</p>
                    <p>Claimable</p>
                  </div>
                  <svg fill="none" width="45" height="45" className='img-overview' viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg" class="h-full" style={{ marginRight: '30px' }}><radialGradient id="dai_svg__a" cx="0" cy="0" gradientTransform="rotate(54.17 -42.31 263.4) scale(497.082)" gradientUnits="userSpaceOnUse" r="1"><stop offset="0" stop-color="#fffac8"></stop><stop offset="1" stop-color="#f9a806"></stop></radialGradient><path d="M300 600a298.1 298.1 0 01-116.74-23.58A298.32 298.32 0 0187.9 512.1a300.3 300.3 0 01-64.32-95.36C7.96 379.8 0 340.52 0 300s7.96-79.8 23.58-116.74A298.3 298.3 0 0187.9 87.9a300.3 300.3 0 0195.36-64.32A298.63 298.63 0 01300 0c40.52 0 79.8 7.96 116.74 23.58A298.3 298.3 0 01512.1 87.9a300.32 300.32 0 0164.32 95.36A298.64 298.64 0 01600 300c0 40.52-7.96 79.8-23.58 116.74-15.12 35.7-36.73 67.83-64.32 95.36s-59.65 49.2-95.36 64.32A298.1 298.1 0 01300 600z" fill="#fef9c2"></path><path d="M300 600c165.69 0 300-134.31 300-300S465.69 0 300 0 0 134.31 0 300s134.31 300 300 300z" fill="url(#dai_svg__a)" fill-opacity=".35"></path><path d="M464.42 254.28H431.5c-18.06-50.3-66.82-84.8-131.02-84.8h-105.7v84.73h-36.72v30.43h36.73v31.9h-36.73v30.43h36.73v83.76h105.63c63.5 0 111.85-34.17 130.45-83.76h33.55v-30.44h-26.14c.62-5.4 1.03-10.93 1.03-16.46v-.76c0-4.98-.27-9.9-.76-14.67h26.01v-30.43zm-240.02-57.7h76.08c47.18 0 82.18 23.18 98.37 57.63H224.4zm76.08 206.76H224.4V346.9h174.24c-16.26 33.82-51.19 56.44-98.16 56.44zm108.26-102.58c0 5.4-.35 10.65-1.1 15.7H224.4v-31.89h183.3c.62 4.98 1.04 10.17 1.04 15.43z" fill="#fa0"></path></svg>
                </Paper>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <div className="overview-data">
                    <p>{deposited && convertFromWei(deposited,5).toFixed(5)} DAI</p>
                    <p>${deposited && daiP && (convertFromWei(deposited)*convertFromWei(daiP)).toFixed(5)}</p>
                    <p>Total Deposited</p>
                  </div>
                  <svg width="48" height="48" fill="none" xmlns="http://www.w3.org/2000/svg" className='img-overview'><circle opacity=".1" cx="24" cy="24" r="24" fill="#2EBDC2"></circle><g clip-path="url(#incoming_svg__clip0)" fill="#2EBDC2"><path d="M14.4 21.334c-.589 0-1.066.477-1.066 1.066v10.667c0 .589.477 1.066 1.066 1.066h19.2c.589 0 1.067-.477 1.067-1.066V22.4c0-.589-.478-1.066-1.067-1.066H14.4zm2.04 2.133h15.12a1.6 1.6 0 00.973.973v6.587a1.6 1.6 0 00-.972.973H16.44a1.598 1.598 0 00-.973-.973V24.44a1.598 1.598 0 00.973-.973zM24 24.534a3.2 3.2 0 100 6.4 3.2 3.2 0 000-6.4zm-5.333 2.133a1.066 1.066 0 100 2.132 1.066 1.066 0 000-2.132zm10.666 0a1.066 1.066 0 100 2.133 1.066 1.066 0 000-2.133z"></path><path d="M20.688 16.075l2.812 2.813a.69.69 0 00.5.212.69.69 0 00.5-.213l2.812-2.812a.694.694 0 00-.002-1.002.721.721 0 00-.497-.21.721.721 0 00-.5.212l-1.6 1.6V12.95a.702.702 0 00-.713-.712.702.702 0 00-.712.712v3.726l-1.6-1.6a.721.721 0 00-.5-.213.721.721 0 00-.5.212.694.694 0 000 1z" stroke="#2EBDC2" stroke-width=".3"></path></g><defs><clipPath id="incoming_svg__clip0"><path fill="#fff" transform="translate(12 12)" d="M0 0h24v24H0z"></path></clipPath></defs></svg>
                </Paper>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <div className="overview-data">
                    <p>{borrowed && convertFromWei(borrowed,5).toFixed(5)} DAI</p>
                    <p>${borrowed && daiP && (convertFromWei(borrowed)*convertFromWei(daiP)).toFixed(5)}</p>
                    <p>Total Borrowed</p>
                  </div>
                  <svg width="48" height="48" fill="none" xmlns="http://www.w3.org/2000/svg" className='img-overview'><circle opacity=".1" cx="24" cy="24" r="24" fill="#5242A2"></circle><g clip-path="url(#outgoing_svg__clip0)" fill="#5242A2"><path d="M27.206 15.169l-2.812-2.813a.54.54 0 00-.394-.168.54.54 0 00-.394.168l-2.812 2.813a.544.544 0 000 .787.572.572 0 00.393.169.572.572 0 00.394-.169l1.857-1.856v4.088c0 .318.243.562.562.562a.553.553 0 00.563-.563V14.1l1.856 1.856a.572.572 0 00.393.169.572.572 0 00.394-.169.544.544 0 000-.787zM14.4 21.334c-.589 0-1.066.477-1.066 1.066v10.667c0 .589.477 1.066 1.066 1.066h19.2c.589 0 1.067-.477 1.067-1.066V22.4c0-.589-.478-1.066-1.067-1.066H14.4zm2.04 2.133h15.12a1.6 1.6 0 00.973.973v6.587a1.6 1.6 0 00-.972.973H16.44a1.598 1.598 0 00-.973-.973V24.44a1.598 1.598 0 00.973-.973zM24 24.534a3.2 3.2 0 100 6.4 3.2 3.2 0 000-6.4zm-5.333 2.133a1.066 1.066 0 100 2.132 1.066 1.066 0 000-2.132zm10.666 0a1.066 1.066 0 100 2.133 1.066 1.066 0 000-2.133z"></path></g><defs><clipPath id="outgoing_svg__clip0"><path fill="#fff" transform="translate(12 12)" d="M0 0h24v24H0z"></path></clipPath></defs></svg>
                </Paper>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <div className="overview-data">
                    <p>{withdrawable && deposited && withdrawn && convertFromWei((parseInt(withdrawable)+parseInt(withdrawn)-parseInt(deposited)).toString(),5).toFixed(5)} DAI</p>
                    <p>${withdrawable && deposited && withdrawn && daiP && (convertFromWei((parseInt(withdrawable)+parseInt(withdrawn)-parseInt(deposited)).toString(),5)*convertFromWei(daiP)).toFixed(5)}</p>
                    <p>Interest Earned</p>
                  </div>
                  <svg width="48" height="48" fill="none" className='img-overview'  xmlns="http://www.w3.org/2000/svg"><circle opacity=".1" cx="24" cy="24" r="24" fill="#CA8700"></circle><g clip-path="url(#balance_svg__clip0)" fill="#CA8700"><path d="M14.4 21.334c-.589 0-1.066.477-1.066 1.066v10.667c0 .589.477 1.066 1.066 1.066h19.2c.589 0 1.067-.477 1.067-1.066V22.4c0-.589-.478-1.066-1.067-1.066H14.4zm2.04 2.133h15.12a1.6 1.6 0 00.973.973v6.587a1.6 1.6 0 00-.972.973H16.44a1.598 1.598 0 00-.973-.973V24.44a1.598 1.598 0 00.973-.973zM24 24.534a3.2 3.2 0 100 6.4 3.2 3.2 0 000-6.4zm-5.333 2.133a1.066 1.066 0 100 2.132 1.066 1.066 0 000-2.132zm10.666 0a1.066 1.066 0 100 2.133 1.066 1.066 0 000-2.133z"></path><rect x="21" y="16" width="7" height="1.5" rx=".75"></rect><rect x="21" y="13" width="7" height="1.5" rx=".75"></rect></g><defs><clipPath id="balance_svg__clip0"><path fill="#fff" transform="translate(12 12)" d="M0 0h24v24H0z"></path></clipPath></defs></svg>
                </Paper>
              </Grid>

              {/* Wallet Balances */}
              <Grid item xs={12} style={{ marginTop: "10px" }}>
                <Typography component="h1" variant="h6" color="inherit" noWrap>
                  Wallet Balances
                </Typography>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <img src={daiIcon} alt="dai-icon" className='img-balance' style={{ maxWidth: '65px' }} />
                  <div className="coin-data">
                    <p>{daiB && convertFromWei(daiB,2)} DAI</p>
                    <p>${daiB && daiP && (convertFromWei(daiB)*convertFromWei(daiP)).toFixed(2)}</p>
                  </div>
                </Paper>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <img src={maticIcon} alt="matic-icon" className='img-balance' />
                  <div className="coin-data">
                    <p>{maticB && convertFromWei(maticB,2)} MATIC</p>
                    <p>${maticB && maticP && (convertFromWei(maticB)*convertFromWei(maticP)).toFixed(2)}</p>
                  </div>
                </Paper>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <img src={mIcon} alt="m-icon" className='img-balance' />
                  <div className="coin-data">
                    <p>{mltB && convertFromWei(mltB,2)} MLT</p>
                    <p>-</p>
                  </div>
                </Paper>
              </Grid>
              <Grid item xs={12} sm={3}>
                <Paper className="card" elevation={2}>
                  <img src={mDaiIcon} alt="mDai-icon" className='img-balance' style={{ maxWidth: '68px' }} />
                  <div className="coin-data">
                    <p>{mDaiB && convertFromWei(mDaiB,2)} MDAI</p>
                    <p>-</p>
                  </div>
                </Paper>
              </Grid>
            </Grid>
          </div>
        </>
      }
    </>
  )
}

export default Balances;