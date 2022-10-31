# Multifactor Authentication Oracle (aka mfa_oracle)

This prototype (it's very crude) implements parts of the WebAuthn protocol
on-ledger and interacts with the browser to allow users require transactions be
authorized by an MFA token.  This prototypes assumes the token is attached to a
modern web-browser, though the ledger doesn't know what it's talking to so
anything that supports WebAuthn (ECDSA only for this prototype) should work.

# Why do this?

It opens up tons of options for users to secure transactions in fancy ways.  The basic
primitive is that a user can "protect" a transaction they create by injecting into the manifest
a call to "check" on their oracle component.  In this way they can protect any transaction they want.

# How do I try it?

Be warned this is not for the faint of heart...  Sorry.

I'm publishing the built wasm file because building it requires lots of patched dependencies and I don't have time before
the challenge deadline to publish the (I think it's up to 9) git repos.  I'll get to it, but for the challenge deadline you can load the wasm file
that's baked.

So, just do `cd mfa_oracle && npm start`

If you want to try to build the scrypto-side: `cd mfa_oracle/scrypto/mfa_oracle && ./builder.sh`

(but until i make all the dependencies available that second builder.sh step won't work)

# Why is this code such a mess?

I ran out of time...

# Does it even work?

Yes, on my computer ;) ...mostly -- There's a lot of handwaving.... The prototype design depends on noticing a failed transaction
that requires MFA (the browser as an oracle noticing a log output), then running a new transaction starting in the browser
doing the "authentication ceremony" for WebAuthn and submitting a new transaction with the result.  On ledger after validation
the transaction being "vouched for" is saved in an "authorized transactions" list.

The browser can read that list too and see when it should try to resubmit the original transaction.  In the demo there's
lots of button clicking in the right order to see things work, and the UI doesn't always update right.  And it can be super slow
after a while.  I think it works, or is very close to working.

Also, to test it, you'll need to enable the virtual WebAuthn credential in Chrome.

If you want to use my "localpte" the setup is a bit tricky.  You need to use a patched version of the extension swapping out
the "pte02.radixdlt.com" string for "localhost:3500" (it only shows up 3 times).  Then, you also have extract the private
key from the extension using Chrome dev tools and paste it into the localpte app/main.py

I tried to do a "simple" design but it's based on some things that aren't flushed out, like how nonces work, and
resending transactions.  It expects to to be able to resubmit a the failed transaction, and critically expecting to get the same transaction hash.
With more time it could do something different.  With the WebAuthn code all working and interacting between on/off ledger
the way it actually gets checked on-ledger is almost secondary.

So things get tricky and some of this prototype mmight be an oversimplification
that is not really possible in practice, but since I was working locally with
my hacked up "PTE" environment controlling all the nonces I could prove
everything worked. 

# You implemented the WebAuthn Spec?

No, but I did manage to patch a whole bunch of code to run on-ledger.  This is a neat adventure on its own since
it shows practicalities of building real *integrations* on ledger that are not just self-contained DeFi programs.

I built patches for a number of dependencies to remove floating point or remove
external depencies on system things like random or time, and swapped out some
of the crypto calls in `ring` for a pure Rust implementation that compiles to
wasm.

Patched dependencies were mostly related to serialization the patched packages in the graph look like this:

```
mfa_oracle v0.1.0 (/home/user/radix_workspace/challenges/oracle/scrypto_mfa/mfa_oracle/scrypto/mfa_oracle)
├── serde v1.0.137 (/home/user/radix_workspace/challenges/oracle/serde/serde)
│   └── serde_derive v1.0.137 (proc-macro) (/home/user/radix_workspace/challenges/oracle/serde/serde_derive)
├── serde_json v1.0.81 (/home/user/radix_workspace/challenges/oracle/serde-rs/json)
│   └── serde v1.0.137 (/home/user/radix_workspace/challenges/oracle/serde/serde) (*)
└── slauth v0.6.5 (/home/user/radix_workspace/challenges/oracle/slauth)
    ├── ring v0.16.19 (/home/user/radix_workspace/challenges/oracle/ring)
    ├── serde v1.0.137 (/home/user/radix_workspace/challenges/oracle/serde/serde) (*)
    ├── serde_bytes v0.11.6 (/home/user/radix_workspace/challenges/oracle/serde-rs/bytes)
    │   └── serde v1.0.137 (/home/user/radix_workspace/challenges/oracle/serde/serde) (*)
    ├── serde_cbor v0.11.2 (/home/user/radix_workspace/challenges/oracle/cbor)
    │   └── serde v1.0.137 (/home/user/radix_workspace/challenges/oracle/serde/serde) (*)
    ├── serde_json v1.0.81 (/home/user/radix_workspace/challenges/oracle/serde-rs/json) (*)
    └── webpki v0.22.0 (/home/user/radix_workspace/challenges/oracle/webpki)
        ├── ring v0.16.19 (/home/user/radix_workspace/challenges/oracle/ring) (*)

```

# What else did you do?

Oh, I also built a "LocalPTE" to run against resim just enough to testing things.

And, a I patched up wasm-snip remove all floating point operations and non "radix_engine" imports from a wasm module.

