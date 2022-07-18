import React, { useState, useEffect } from 'react';
import abiDecoder from 'abi-decoder';
import axios from 'axios';
import { makeStyles } from '@material-ui/core/styles';
import { Chip, Paper, TableRow, TableHead, TableContainer, TableCell, TableBody, Table, List, ListItem, ListItemText, Typography, Tooltip, ListItemSecondaryAction, IconButton, Tabs, Tab, Grid, Card, CardContent, Button, Icon, TextField, InputAdornment } from '@material-ui/core';
import { AccountBalanceWalletOutlined, GetAppOutlined, LockOutlined, MonetizationOnOutlined, ScheduleOutlined } from '@material-ui/icons';

import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

import MDai from '../../Abis/MDai.json';
import MLTToken from '../../Abis/MLTToken.json';

import { mDaiA, mltA } from '../../addresses';

import Loading from '../Loading';

import mDaiIcon from "../../Assets/mDai.svg";
import mIcon from "../../Assets/m.svg";

import { downloadCSV, convertFromWei, convertToWei } from '../helper';

const useStyles = makeStyles((theme) => ({
  table: {
    minWidth: 650,
  },
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

function Stake(props) {
  const { account, dai, mDai, multi, market } = props;

  const [isLoading, setLoading] = useState(true);
  const tokens = ['mlt', 'mDai'];
  const [token, setToken] = useState('mlt');
  const [transactions, setTransactions] = useState([]);

  const [balance, setBalance] = useState(null);
  const [balanceS, setBalanceS] = useState(null);
  const [balanceR, setBalanceR] = useState(null);
  
  const [value, setValue] = useState(0);

  const [stakeAmount, setStakeAmount] = useState(0);

  const notify = (success, msg) => {
    setLoading(false);
    if(success) {
      toast.success(`${msg}`, {
        position: "top-right",
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
      });
      setTimeout(() => {
        fetchTransactions();
      }, [25000])
    }
    else {
      toast.error(` ${msg}!`, {
        position: "top-right",
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
      });
    }
  }

  const handleChange = (event, newValue) => {
    setValue(newValue);
    setToken(tokens[newValue]);
  };

  const handleStake = async () => {
    if(!stakeAmount || stakeAmount <= 0) {
      // > 0
      notify(false, ` Amount must be greater than 0!`);
    }
    else if(parseInt(convertToWei(stakeAmount)) > parseInt(balance)) {
      // Exceeds Balance
      notify(false, ` Amount exceeds wallet balance!`);
    }
    else {
      // Success
      setLoading(true); 
      const searchA = token === 'mDai' ? mDaiA : mltA;
      const searchAbi = token === 'mDai' ? mDai : multi;
      
      await searchAbi.methods
        .approve(searchA, convertToWei(stakeAmount))
        .send({ from: account })
        .on('transactionHash', async (hash) => {
          await searchAbi.methods
            .stake(convertToWei(stakeAmount))
            .send({ from: account })
            .on('transactionHash', (hash) => {
              notify(true, `Successfully Staked!`);
            })
            .on('error', (e) =>{
              notify(false, e.message);
            })
        })
        .on('error', (e) =>{
          notify(false, e.message);
        })
    }
  }

  const handleUnstake = async () => {
    if(!stakeAmount || stakeAmount <= 0) {
      // > 0
      notify(false, ` Amount must be greater than 0!`);
    }
    else if(parseInt(convertToWei(stakeAmount)) > parseInt(balanceS)) {
      // Exceeds Balance
      notify(false, ` Amount exceeds staked balance!`);
    }
    else {
      // Success
      setLoading(true); 
      const searchAbi = token === 'mDai' ? mDai : multi;
      
      await searchAbi.methods
        .withdraw(convertToWei(stakeAmount))
        .send({ from: account })
        .on('transactionHash', (hash) => {
          notify(true, `Successfully Unstaked!`);
        })
        .on('error', (e) =>{
          notify(false, e.message);
        })
    }
  }

  const handleGetRewards = async () => {
    // Success
    setLoading(true); 
    const searchAbi = token === 'mDai' ? mDai : multi;
    
    await searchAbi.methods
      .getReward()
      .send({ from: account })
      .on('transactionHash', (hash) => {
        notify(true, `Successfully Claimed!`);
      })
      .on('error', (e) =>{
        notify(false, e.message);
      })
  }
  
  const fetchTransactions = async () => {
    const searchA = token === 'mDai' ? mDaiA : mltA;
    const searchAbi = token === 'mDai' ? MDai : MLTToken;
    const searchS = token === 'mDai' ? mDai : multi;
   
    const res = await axios.get(`https://api-testnet.polygonscan.com/api?module=account&action=txlist&address=${searchA}&startblock=0&endblock=latest&page=1&offset=100&sort=desc&apikey=YourApiKeyToken`);
    
    if(res && res.data && res.data.result) {
      abiDecoder.addABI(searchAbi);
      let txs = [];
      res.data.result.forEach((tx) => {
        if(tx.to !== "" && tx.from && tx.from.toLowerCase() === account.toLowerCase()) txs.push({
          hash: tx.hash,
          method: abiDecoder.decodeMethod(tx.input).name,
          age: (new Date(tx.timeStamp*1000)).toUTCString().replace(" GMT", ""),
          value: tx.value/1e18,
          fee: ((tx.gasPrice*tx.gasUsed)/1e18)
        })
      });
      setTransactions(txs.filter((tx) => {
        return tx.method !== 'setAllowedContract';
      }));

    }
    if(account && account !== '' && searchS) {
      setBalance(await searchS.methods.balanceOf(account).call());
      setBalanceS(await searchS.methods._balances(account).call());
      setBalanceR(await searchS.methods.earned(account).call());
    }

    setLoading(false);
    // Txn Hash => hash
    // Method => abiDecoder.decodeMethod(tx.input.name)
    // Age => timestamp
    // Value => value
    // Txn Fee => (gasPrice*gasUsed)/1e18
  
    // Block => blockNumber
    // From => from
    // To => to
  }

  useEffect(() => {
    if(token) fetchTransactions();
  }, [token, account])

  const classes = useStyles();

  return(
    <>
      {
        isLoading ? <Loading /> :
        <>
          <Paper square style={{ maxWidth: '320px', margin: '20px auto 30px auto' }}>
            <Tabs
              value={value}
              onChange={handleChange}
              indicatorColor="primary"
              textColor="primary"
              centered
            >
              <Tab label="Stake MLT" />
              <Tab label="Stake MDAI" />
            </Tabs>
          </Paper>

              <Grid container spacing={4} style={{ cursor: 'default' }}>
                <Grid item xs={12} sm={3}>
                  <Paper className="card" elevation={2}>
                    <Icon className='icon-stake' fontSize='large' color='primary'>
                      <MonetizationOnOutlined fontSize='large' />
                    </Icon>
                    <div className="coin-data">
                      <p>{balanceR && convertFromWei(balanceR,5)} {token.toUpperCase()}</p>
                      <p>Claimable Tokens</p>
                    </div>
                  </Paper>
                </Grid>
                <Grid item xs={12} sm={3}>
                  <Paper className="card" elevation={2}>
                    <Icon className='icon-stake' fontSize='large' color='primary'>
                      <LockOutlined fontSize='large' />
                    </Icon>
                    <div className="coin-data">
                      <p>{balanceS && convertFromWei(balanceS,2)} {token.toUpperCase()}</p>
                      <p>Staked Balance</p>
                    </div>
                  </Paper>
                </Grid>
                <Grid item xs={12} sm={3}>
                  <Paper className="card" elevation={2}>
                    <Icon className='icon-stake' fontSize='large' color='primary'>
                      <AccountBalanceWalletOutlined fontSize='large' />
                    </Icon>
                    <div className="coin-data">
                      <p>{balance && convertFromWei(balance,2)} {token.toUpperCase()}</p>
                      <p>Wallet Balance</p>
                    </div>
                  </Paper>
                </Grid>
                <Grid item xs={12} sm={3}>
                  <Paper className="card" elevation={2}>
                    <Icon className='icon-stake' fontSize='large' color='primary'>
                      <ScheduleOutlined fontSize='large' />
                    </Icon>
                    <div className="coin-data">
                      <p>7 Days</p>
                      <p>Unstaking Period</p>
                    </div>
                  </Paper>
                </Grid>

                <Grid item xs={12} sm={4} style={{ margin: '0 auto 20px auto' }}>
                  <Card style={{ padding: '5px' }}>
                    <CardContent>
                      <TextField
                        fullWidth
                        variant="outlined"
                        placeholder='Amount to Stake/Unstake'
                        InputProps={{
                          endAdornment: (
                            <InputAdornment position="end">
                              <img src={token === 'mDai' ? mDaiIcon : mIcon} alt="stake-icon" height="35px" />
                            </InputAdornment>
                          ),
                        }}
                        value={stakeAmount}
                        onChange={(e) => setStakeAmount(e.target.value)}
                      />
                      <div style={{ margin: '25px auto 15px auto' }}>
                        <Button
                          onClick={() => handleStake()}
                          variant="contained"
                          color="primary"
                          style={{ width: '48%', marginRight: '4%', textTransform: 'none' }}
                        >
                          Stake
                        </Button>
                        <Button
                          onClick={() => handleUnstake()}
                          variant="contained"
                          color="primary"
                          style={{ width: '48%', textTransform: 'none' }}
                        >
                          Unstake
                        </Button>
                      </div>
                      <Button
                        onClick={() => handleGetRewards()}
                        variant="outlined"
                        color="primary"
                        fullWidth
                        style={{ textTransform: 'none' }}
                      >
                        Claim Rewards
                      </Button>
                    </CardContent>
                  </Card>
                </Grid>
              </Grid>

          {/* Download Transactions */}
          <List>
            <ListItem>
              <ListItemText
                disableTypography
                primary={<Typography variant="h6" style={{cursor: 'default'}}>Number of Transactions: {transactions.length}</Typography>}
              />
                <ListItemSecondaryAction>
                  <Tooltip title="CSV Export" aria-label="download">
                    <IconButton edge="end" style={{border:'none',outline:'none'}}
                      onClick={() => downloadCSV(transactions)}
                    >
                      <GetAppOutlined fontSize="large" />
                    </IconButton>
                  </Tooltip>
                </ListItemSecondaryAction>
            </ListItem>      
          </List>

          {/* Transactions */}
          <TableContainer component={Paper}>
            <Table className={classes.table} aria-label="simple table">
              <TableHead style={{ backgroundColor: '#f8fafd' }}>
                <TableRow>
                  <TableCell className='tableHeading'>Txn Hash</TableCell>
                  <TableCell className='tableHeading'>Method</TableCell>
                  <TableCell className='tableHeading'>Date Time (UTC)</TableCell>
                  <TableCell className='tableHeading'>Value</TableCell>
                  <TableCell className='tableHeading'>[Txn Fee]</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {transactions.map((tx) => (
                  <TableRow key={tx.hash}>
                    <TableCell onClick={() => window.open(`https://mumbai.polygonscan.com/tx/${tx.hash}`)} style={{ color: '#3f51b5', cursor: 'pointer' }}>
                      {tx.hash && tx.hash.substring(0,30)+'...'}
                    </TableCell>
                    <TableCell>
                      <Chip label={tx.method} style={{ backgroundColor: 'rgba(52,152,219,.1)' }} />
                    </TableCell>
                    <TableCell>{tx.age}</TableCell>
                    <TableCell>{tx.value} MATIC</TableCell>
                    <TableCell>{tx.fee}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>

          <ToastContainer
            position="top-right"
            autoClose={5000}
            hideProgressBar={false}
            newestOnTop={false}
            closeOnClick
            rtl={false}
            pauseOnFocusLoss
            draggable
            pauseOnHover
          />
        </>
      }
    </>
  )
}

export default Stake;