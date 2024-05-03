# VRF Oracle

- This implementation of VRF follows the [draft-irtf-cfrg-vrf-11](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11) publication. This publication is an updated version of the VRF implementation that the [Chainlink VRF v1](https://github.com/smartcontractkit/chainlink/blob/ff56c1657a48bb7f9782e407412b4e5aa10ff2fc/contracts/src/v0.6/VRF.sol#L6) uses. Version 12 was released on May 26th, 2022, right before the deadline of the Scrypto Oracle submissions challenge, and the implementation in this repo has not yet been updated to use the most current draft publication.
- This implementation uses the NistP256 curve as this is the curve that the publication has the most details regarding the VRF implementation, specifically encoding random input seeds to points on the P256 curve, a required step in generating proofs. This curve is also the the curve with the most support for the required VRF ECC math in the Rust elliptic-curves [p256](https://docs.rs/p256/latest/p256/#) crate.
- This model follows the the architecture where a random number requestor will request a random number from a VrfOracleContract component, which returns a receipt badge. The requestor will then need to fetch the result of the random number generation from the VrfOracleContract component at a later time. The VrfOracle internally verifies the proof that it receives from the off-chain oracle VRF prover, so the random number requestor may directly use the returned random bytes.

# Usage

For a VRF oracle operator, the process is as follows:

1. Generate Nist P256 elliptical curve public/private key pair that you will be using in the off-chain oracle VRF prover.
2. Instantiate your on-chain component using the VrfOracleContract::new() method, providing it the public key from above as a string of hex characters.
3. Have the off-chain oracle node subscribe random number requests made by consumers to the VrfOracleContract::request_randomness() method.
4. In the off-chain oracle, generate VRF proofs for the requested input seeds and submit the VRF proof on-chain using the VrfOracleContract::fullfill_randomness_request() method.

For a random number requestor/consumer, the process is as follows:

1. Identify a VRF Oracle Contract component that an oracle operator is running.
2. Call the request_randomness method with a payment of (TBD) XRD. (For the sake of the challenge submission, the amount is anything you want, there are no checks.) The method will return a receipt badge of your random number request.
3. At a later point in time, call the fetch_randomness method with your receipt badge. If the off-chain oracle correctly serviced your request with a valid proof, then the method will return a vector of random bytes. The badge will be burnt, as all random numbers are one time use.

Example usage can be seen in the vrf_verify.rev revup script. 

# Verification of implementation

The implementation is verified with unit tests using the [provided test vectors and examples](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#appendix-A.2) in the publication.

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