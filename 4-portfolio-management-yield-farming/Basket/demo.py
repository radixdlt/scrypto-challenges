import subprocess
import os

def setup():
    global xrd
    global account
    global ammdex_package
    global basket_package
    global token_A
    global token_B
    global token_C
    global pool_A
    global lp_token_A
    global pool_B
    global lp_token_B
    global pool_C
    global lp_token_C
    global fund
    global fund_admin_badge
    global fund_token
    global fund_stake_receipt
    global buy_auction_A
    global buy_bid_A
    global sell_auction_A
    global sell_bid_A
    global buy_auction_B
    global buy_bid_B
    global sell_auction_B
    global sell_bid_B
    global buy_auction_C
    global buy_bid_C
    global sell_auction_C
    global sell_bid_C

    subprocess.run('resim reset')
    xrd = '030000000000000000000000000000000000000000000000000004'
    account = subprocess.run('resim new-account', stdout=subprocess.PIPE, universal_newlines=True).stdout.split('Account component address: ')[1][0:54]
    ammdex_package = subprocess.run('resim publish ./ammdex', stdout=subprocess.PIPE, universal_newlines=True).stdout.split('Package: ')[1][0:54]
    basket_package = subprocess.run('resim publish ./basket', stdout=subprocess.PIPE, universal_newlines=True).stdout.split('Package: ')[1][0:54]

    env = {
        **os.environ,
        'ACCOUNT': account,
    }
    stdout = subprocess.run('resim run ./transactions/0_setup_tokens.rtm', env=env, stdout=subprocess.PIPE, universal_newlines=True).stdout
    token_A = stdout.split('Resource: ')[1][0:54]
    token_B = stdout.split('Resource: ')[2][0:54]
    token_C = stdout.split('Resource: ')[3][0:54]

    env = {
        **os.environ,
        'ACCOUNT': account,
        'AMM_PACKAGE': ammdex_package,
        'AMOUNT_XRD': str(10000),
        'XRD': xrd,
        'AMOUNT_A': str(100000),
        'TOKEN_A': token_A,
        'AMOUNT_B': str(300000),
        'TOKEN_B': token_B,
        'AMOUNT_C': str(500000),
        'TOKEN_C': token_C,
    }
    stdout = subprocess.run('resim run ./transactions/1_setup_pools.rtm', env=env, stdout=subprocess.PIPE, universal_newlines=True).stdout
    pool_A = stdout.split('Component: ')[1][0:54]
    pool_B = stdout.split('Component: ')[2][0:54]
    pool_C = stdout.split('Component: ')[3][0:54]
    lp_token_A = stdout.split('Resource: ')[2][0:54]
    lp_token_B = stdout.split('Resource: ')[4][0:54]
    lp_token_C = stdout.split('Resource: ')[6][0:54]
    
    env = {
        **os.environ,
        'ACCOUNT': account,
        'BASKET_PACKAGE': basket_package,
        'NAME': 'XRD Fund',
        'TOKEN_NAME': 'Fund Token',
        'TOKEN_SYMBOL': 'FUND',
        'DENOMINATOR_TOKEN': xrd,
        'AUCTION_DELAY': str(20),
        'FEE_PERCENT': str(5),
    }
    stdout = subprocess.run('resim run ./transactions/2_setup_fund.rtm', env=env, stdout=subprocess.PIPE, universal_newlines=True).stdout
    fund = stdout.split('Component: ')[1][0:54]
    fund_admin_badge = stdout.split('Resource: ')[1][0:54]
    fund_token = stdout.split('Resource: ')[3][0:54]
    fund_stake_receipt = stdout.split('Resource: ')[4][0:54]

    env = {
        **os.environ,
        'ACCOUNT': account,
        'FUND_ADMIN_BADGE': fund_admin_badge,
        'FUND': fund,
        'POOL_A': pool_A,
        'POOL_B': pool_B,
        'POOL_C': pool_C,
    }
    stdout = subprocess.run('resim run ./transactions/3_setup_investments.rtm', env=env, stdout=subprocess.PIPE, universal_newlines=True).stdout
    buy_auction_A = stdout.split('Component: ')[1][0:54]
    sell_auction_A = stdout.split('Component: ')[2][0:54]
    buy_auction_B = stdout.split('Component: ')[3][0:54]
    sell_auction_B = stdout.split('Component: ')[4][0:54]
    buy_auction_C = stdout.split('Component: ')[5][0:54]
    sell_auction_C = stdout.split('Component: ')[6][0:54]
    buy_bid_A = stdout.split('Resource: ')[3][0:54]
    sell_bid_A = stdout.split('Resource: ')[6][0:54]
    buy_bid_B = stdout.split('Resource: ')[9][0:54]
    sell_bid_B = stdout.split('Resource: ')[12][0:54]
    buy_bid_C = stdout.split('Resource: ')[15][0:54]
    sell_bid_C = stdout.split('Resource: ')[18][0:54]

def show(address):
    subprocess.run('resim show {}'.format(address))

def set_epoch(epoch):
    subprocess.run('resim set-current-epoch {}'.format(epoch))

def mint(amount):
    env = {
        **os.environ,
        'AMOUNT': str(amount),
        'ACCOUNT': account,
        'XRD': xrd,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/mint.rtm', env=env)

def redeem(amount):
    env = {
        **os.environ,
        'AMOUNT': str(amount),
        'ACCOUNT': account,
        'FUND_TOKEN': fund_token,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/redeem.rtm', env=env)

def redeem_for_tokens(amount):
    env = {
        **os.environ,
        'AMOUNT': str(amount),
        'ACCOUNT': account,
        'FUND_TOKEN': fund_token,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/redeem_for_tokens.rtm', env=env)

def stake(amount, investment):
    env = {
        **os.environ,
        'AMOUNT': str(amount),
        'INVESTMENT': str(investment),
        'ACCOUNT': account,
        'FUND_TOKEN': fund_token,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/stake.rtm', env=env)

def unstake(id):
    env = {
        **os.environ,
        'FUND_STAKE_RECEIPT_ID': str(id),
        'ACCOUNT': account,
        'FUND_STAKE_RECEIPT': fund_stake_receipt,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/unstake.rtm', env=env)

def collect_unstaked(id):
    env = {
        **os.environ,
        'FUND_STAKE_RECEIPT_ID': str(id),
        'ACCOUNT': account,
        'FUND_STAKE_RECEIPT': fund_stake_receipt,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/collect_unstaked.rtm', env=env)

def process_stakes():
    env = {
        **os.environ,
        'FUND': fund,
    }
    subprocess.run('resim run ./transactions/process_stakes.rtm', env=env)

def amm_swap(amount, token, pool):
    env = {
        **os.environ,
        'AMOUNT': str(amount),
        'TOKEN': str(token),
        'POOL': str(pool),
        'ACCOUNT': account,
    }
    subprocess.run('resim run ./transactions/amm_swap.rtm', env=env)

def amm_remove_liquidity(amount, lp_token, pool):
    env = {
        **os.environ,
        'AMOUNT': str(amount),
        'LP_TOKEN': str(lp_token),
        'POOL': str(pool),
        'ACCOUNT': account,
    }
    subprocess.run('resim run ./transactions/amm_remove_liquidity.rtm', env=env)

def create_bid(auction, amount, token, price):
    env = {
        **os.environ,
        'AUCTION': str(auction),
        'AMOUNT': str(amount),
        'TOKEN': str(token),
        'PRICE': str(price),
        'ACCOUNT': account,
    }
    subprocess.run('resim run ./transactions/create_bid.rtm', env=env)
    
def close_bid(auction, id, bid_type):
    env = {
        **os.environ,
        'AUCTION': str(auction),
        'AUCTION_BID_ID': str(id),
        'AUCTION_BID': str(bid_type),
        'ACCOUNT': account,
    }
    subprocess.run('resim run ./transactions/close_bid.rtm', env=env)

if __name__ == '__main__':
    setup()

    create_bid(buy_auction_A, 100000, token_A, 0.1)
    create_bid(sell_auction_A, 10000, xrd, 10)
    create_bid(buy_auction_B, 300000, token_B, 0.033)
    create_bid(sell_auction_B, 10000, xrd, 30)
    create_bid(buy_auction_C, 500000, token_C, 0.02)
    create_bid(sell_auction_C, 10000, xrd, 50)
    
    mint(1000)
    stake(30, 0)
    set_epoch(30)
    process_stakes()
    
    show(fund)
    show(account)