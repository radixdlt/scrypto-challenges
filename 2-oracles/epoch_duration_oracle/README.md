# epoch_duration_oracle

The goal of this oracle is to provide an on-ledger duration of all passed epoch since creation of the component as well as a way to compute time elapsed between an epoch and the current epoch.

All durations are currently in milliseconds.

## Running

### Feed the oracle

To run and test the epoch oracle, you can run one of the following:

> - update the oracle only once per epoch:
> ```bash
> (cd scrypto/epoch_duration_oracle && ./tick_on_epoch.sh)
> ```

> - update the oracle each second (to avoid locking on scrypto local resource, a 1 second sleep is necessary if we want to request the component while script runs):
> ```bash
> (cd scrypto/epoch_duration_oracle && ./tick_asap.sh)
> ```

### Query time

To query from the start of the oracle state, you can use:

```bash
resim run scrypto/epoch_duration_oracle/manifests/since_epoch_0.manifest
```

To see the difference, there is also a `since_epoch_2.manifest` and a `since_epoch_633.manifest` to compute time elapsed since different epoch.

## ABI

The available functions are:

- `EpochDurationOracle::new`: creates a new oracle starting from now and disregarding already elapsed time (does not consider elapsed time of the passed epochs).
- `EpochDurationOracle::new_with_bootstrap(last_epoch: u64, millis_in_last_epoch: u64)`: creates a new oracle starting from `last_epoch` which lasted `millis_in_last_epoch`. This is useful if you want to start the oracle at a given point in time (but after January 1st 1970).

The available method for oracle creator is:

- `EpochDurationOracle::tick(millis_since_last_tick: u64)`: ticks the internal clock by the provided milliseconds. This can:
  - end the current epoch, categorize it with its duration and start counting down for a new epoch if previous epoch just ended
  - add the tick amount to the epoch being currently counted down
  - this always return the current ledger epoch

The available open method is:

- `EpochDurationOracle::millis_since_epoch(epoch: u64)`: measure time passed between provided `epoch` and current epoch. This can give birth to few cases:
  - the provided `epoch` is higher than on-ledger epoch: we will return the time spent on the current epoch
  - the provided `epoch` is equal to the on-ledger epoch: we will return the time spent on the current epoch
  - the provided `epoch` is lower than the on-ledger epoch: we will return the time spent between provided epoch and current epoch
  - the provided `epoch` does not appear for oracle was not created yet: we will return the time spent since the creation of the oracle and the current epoch

> Note: we will be adding a method to get duration of a provided epoch and duration between two epochs.