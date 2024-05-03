# Kaupa - A generic market for trading bags of tokens

The Kaupa component allows you to create marketplaces where
participants trade bags of tokens in exchange for other bags of
tokens, with fungibles and non-fungibles both fully supported
throughout Kaupa.

If the two bags have only one resource in them each, Kaupa has special
support for treating them as a trading pair with order books, limit
trade and market trade.

And if you would rather *lend* your bag of tokens instead of sell it,
Kaupa supports flash lending of a bag of tokens for another bag of
tokens as well.

While all that is going, the owner of the Kaupa instance itself can be
gathering fees from the trade activity, with a flexible fee system
that can be configured to their liking.

(Note that flash loan support is most likely hampered by a bug in
Scrypto 0.8.0 that prevents transient tokens from working as they
should at the conclusion of a transaction manifest. While the flash
loans *have* been implemented, getting them to actually run the way we
want to will likely have to wait for a Scrypto update.)

## How to build the blueprint
Make sure you have the necessary toolchain installed, see
[here](https://docs-babylon.radixdlt.com/main/getting-started-developers/getting-started-developers.html)
for details. You will need Scrypto 0.8.0.
- From the command line, in the `kaupa` directory, run `scrypto build`

### How to run the test suite
- From the command line, in the `kaupa` directory, run `scrypto test`

The test suite includes a compelling scenario for the use of a flash
loan, so make sure to check it out! (Look for Story time!)

### How to generate the documentation
- From the command line, in the `kaupa` directory, run `cargo doc`

The generated web pages contain detailed documentation on how the
blueprint works.


## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.

