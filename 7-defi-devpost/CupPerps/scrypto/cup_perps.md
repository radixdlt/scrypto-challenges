# Cup Perps

## tldr

There's two cups, filled with liquid cash, that aim to have liquidity be in equal measure between them. 
One cup represents the bet that believes that a given index will go up, and the other, down.
When balanced, they both represent exposure with a set leverage (for example, 5x)
When out of balance, the cup that lacks liquidity enjoys a funding rate proportional to the difference in liquidity. 

## Definitions

We presume an oracle feed that grants the system access to a tuple of 
oracle = `(unix timestamp, exchange rate)`
which shows the exchange rate of the pair in question at the exact timestamp.

## Constants 

`leverage` 
set on a per-pair basis,
max leverage before funding rate 
(in practice, always less than that)

`funding-coeff`
funding rate coefficient

## Variables 

`long-cup, short-cup`
hold the amount of usd-coins, 
and are used as syntactic sugar for transfering from/to them

`long-cup-lp, short-cup-lp`
track the number of issued lp tokens for each cup

`last-update`
is a unix timestamp of the last system update
equivalent to last interaction with it

`last-exrate`
is the exchange rate at last update

## Methods

### update()

the method may be called as often as the oracle is willing to update

```
assert(oracle.0 >= last-update)
assert(oracle.1 > 0)

if last-exrate == oracle.1 { return }

delta = (oracle.1 / last-exrate - 1) * leverage
long-d = delta * long-cup - long-cup
short-d = delta * short-cup - short-cup

funding = min(short-cup/long-cup, long-cup/short-cup) * funding-coeff
adj-delta = min(|long-d|, |short-d|) * funding

if delta > 0 {
    transfer adj-delta from short-cup to long-cup
} else { 
    transfer adj-delta from long-cup to short-cup
}

last-exrate = oracle.1
last-update = oracle.0

```

### deposit(side: bool, input)

```
update() // update before every interaction

if side { 
    lp-caller = long-cup-lp * ((long-cup + input) / long-cup - 1)
    lp-long-cup += lp-caller
} else {
    lp-caller = short-cup-lp * ((short-cup + input) / short-cup - 1)
    lp-short-cup += lp-caller
}

transfer input from caller to (if side { long-cup } else { short-cup })
mint lp-caller to caller 
```

### withdraw(side: bool, input)

````
update() // update before every interaction

if side { 
    payout = input / lp-long-cup * long-cup 
    lp-long-cup -= input
} else {
    payout = input / lp-short-cup * short-cup 
    lp-short-cup -= input
}

transfer payout from (if side { long-cup } else { short-cup }) to caller
burn lp-caller from input 
```