use p256::elliptic_curve::group::GroupEncoding;
use p256::elliptic_curve::hash2curve::{ExpandMsgXmd, GroupDigest};
use p256::elliptic_curve::ScalarCore;
use p256::{AffinePoint, EncodedPoint, NistP256, ProjectivePoint, PublicKey, Scalar};
use scrypto::prelude::*;
use sha2::{Digest, Sha256};

use thiserror::Error;

#[derive(NonFungibleData)]
struct MemberData {}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum VrfError {
    #[error("Decode proof error")]
    DecodeProofError,
    #[error("Public Key")]
    PublicKeyError,
    #[error("Ecvrf verify error")]
    VerifyError,
    #[error("Elliptic curve error")]
    EncodeToCurveError,
}

pub struct EcvrfCiphersuite {
    suite_string: u8,
    h2c_suite_id_string: Vec<u8>,
}

impl EcvrfCiphersuite {
    pub fn new(suite_string: u8, h2c_suite_id_string: &[u8]) -> Self {
        Self {
            suite_string,
            h2c_suite_id_string: h2c_suite_id_string.to_owned(),
        }
    }

    // 5.2.  ECVRF Proof to Hash
    // https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.2
    fn ecvrf_proof_to_hash(&self, gamma: ProjectivePoint) -> [u8; 32] {
        // 1.  D = ECVRF_decode_proof(pi_string) (see Section 5.4.4)
        // 2.  If D is "INVALID", output "INVALID" and stop
        // 3.  (Gamma, c, s) = D
        // Skip above steps, only gamma needed for generating hash from proof

        // 4.  proof_to_hash_domain_separator_front = 0x03
        // 5.  proof_to_hash_domain_separator_back = 0x00
        let proof_to_hash_domain_separator_front = 0x03;
        let proof_to_hash_domain_separator_back = 0x00;

        // 6.  beta_string = Hash(suite_string ||
        // proof_to_hash_domain_separator_front || point_to_string(cofactor
        // * Gamma) || proof_to_hash_domain_separator_back)
        //
        // cofactor for NistP256 prime curve is 1, so cofactor*gamma = gamma
        let mut hasher = Sha256::new();
        hasher.update([self.suite_string, proof_to_hash_domain_separator_front]);
        hasher.update(gamma.to_bytes());
        hasher.update([proof_to_hash_domain_separator_back]);
        let beta_string: [u8; 32] = hasher.finalize().into();

        // 7.  Output beta_string
        beta_string
    }

    // 5.3.  ECVRF Verifying
    // https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.3
    #[allow(non_snake_case)]
    pub fn ecvrf_verify(
        &self,
        pk_bytes: &[u8],
        alpha_bytes: &[u8],
        pi_bytes: &[u8],
    ) -> Result<[u8; 32], VrfError> {
        // 1.   Y = string_to_point(PK_string)
        // 2.   If Y is "INVALID", output "INVALID" and stop
        let pk = match PublicKey::from_sec1_bytes(pk_bytes) {
            Ok(pk) => pk,
            Err(_) => return Err(VrfError::PublicKeyError),
        };
        // 3.   NA
        // 4.   D = ECVRF_decode_proof(pi_string) (see Section 5.4.4)
        // 5.   If D is "INVALID", output "INVALID" and stop
        // 6.   (Gamma, c, s) = D
        let (gamma, c_bytes, s_bytes) = self.ecvrf_decode_proof(pi_bytes)?;

        // 7.   H = ECVRF_encode_to_curve(encode_to_curve_salt, alpha_string) (see Section 5.4.1)
        let H = self.ecvrf_encode_to_curve(pk_bytes, alpha_bytes)?;

        let s: Scalar = match ScalarCore::from_be_slice(&s_bytes) {
            Ok(sc) => sc.into(),
            Err(_) => return Err(VrfError::VerifyError),
        };
        // need to extend C to U256 worth of bytes to convert into Scalar
        let zero_padded_c_bytes: [u8; 32] =
            match [[0u8; 16], c_bytes].concat().as_slice().try_into() {
                Ok(bytes) => bytes,
                Err(_) => return Err(VrfError::VerifyError),
            };
        let c: Scalar = match ScalarCore::from_be_slice(&zero_padded_c_bytes) {
            Ok(scalarCore) => scalarCore.into(),
            Err(_) => return Err(VrfError::VerifyError),
        };
        let B: ProjectivePoint = ProjectivePoint::GENERATOR;
        let Y: ProjectivePoint = pk.into();

        // 8.   U = s*B - c*Y
        // 9.   V = s*H - c*Gamma
        // Mul is defined for ProjectivePoint with Scalar on the rhs, so ProjectivePoint.mul(rhs=Scalar)
        let U = &B * &s - &Y * &c;
        let V = &H * &s - &gamma * &c;

        // 10.  c' = ECVRF_challenge_generation(Y, H, Gamma, U, V) (see Section 5.4.3)
        let c_prime = self.ecvrf_challenge_generation(&Y, &H, &gamma, &U, &V);
        if c_bytes != c_prime {
            return Err(VrfError::VerifyError);
        }
        let beta_bytes = self.ecvrf_proof_to_hash(gamma);
        Ok(beta_bytes)
    }

    // 5.4.1.2.  ECVRF_encode_to_curve_h2c_suite
    // https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.4.1.2
    #[allow(non_snake_case)]
    fn ecvrf_encode_to_curve(
        &self,
        encode_to_curve_salt: &[u8],
        alpha_bytes: &[u8],
    ) -> Result<ProjectivePoint, VrfError> {
        // Parameters specified in: 5.5.  ECVRF Ciphersuites
        // https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.5

        // The domain separation tag DST, a parameter to the hash-to-curve suite, SHALL be set to "ECVRF_" || h2c_suite_ID_string || suite_string
        let mut dst: Vec<u8> = vec![];
        dst.extend(b"ECVRF_");
        dst.extend(self.h2c_suite_id_string.clone());
        dst.extend([self.suite_string]);
        //    1.  string_to_be_hashed = encode_to_curve_salt || alpha_string
        //    2.  H = encode(string_to_be_hashed)
        let mut string_to_be_hashed: Vec<u8> = vec![];
        string_to_be_hashed.extend(encode_to_curve_salt);
        string_to_be_hashed.extend(alpha_bytes);
        let H = match NistP256::encode_from_bytes::<ExpandMsgXmd<Sha256>>(
            &[&string_to_be_hashed],
            &dst,
        ) {
            Ok(point) => point,
            Err(_) => return Err(VrfError::EncodeToCurveError),
        };
        //    3.  Output H
        Ok(H)
    }

    // 5.4.3.  ECVRF Challenge Generation
    // https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.4.3
    fn ecvrf_challenge_generation(
        &self,
        p1: &ProjectivePoint,
        p2: &ProjectivePoint,
        p3: &ProjectivePoint,
        p4: &ProjectivePoint,
        p5: &ProjectivePoint,
    ) -> [u8; 16] {
        //    1.  challenge_generation_domain_separator_front = 0x02
        let challenge_generation_domain_separator_front = 0x02;
        //    2.  Initialize str = suite_string ||
        //        challenge_generation_domain_separator_front
        let mut hash_str: Vec<u8> = vec![
            self.suite_string,
            challenge_generation_domain_separator_front,
        ];
        //    3.  for PJ in [P1, P2, P3, P4, P5]:
        //        str = str || point_to_string(PJ)
        for p in [p1, p2, p3, p4, p5] {
            hash_str.extend(p.to_bytes());
        }
        //    4.  challenge_generation_domain_separator_back = 0x00
        let challenge_generation_domain_separator_back = 0x00;
        //    5.  str = str || challenge_generation_domain_separator_back
        hash_str.extend([challenge_generation_domain_separator_back]);
        //    6.  c_string = Hash(str)
        let mut hasher = Sha256::new();
        hasher.update(hash_str);
        let c_string: [u8; 32] = hasher.finalize().into();
        //    7.  truncated_c_string = c_string[0]...c_string[cLen-1]
        // c_len = 16 defined by https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.5 for NISTP256
        const C_LEN: usize = 16;
        let mut c: [u8; C_LEN] = Default::default();
        c.copy_from_slice(&c_string[0..C_LEN]);
        // let c: Vec<u8> = c_string.into_iter().take(16).collect();
        //    8.  c = string_to_int(truncated_c_string)
        //    9.  Output c
        // don't do 8 conversion to int right now, return raw bytes and let caller decide what to do with bytes
        c
    }

    // 5.4.4.  ECVRF Decode Proof
    // https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-11#section-5.4.4
    fn ecvrf_decode_proof(
        &self,
        pi_string: &[u8],
    ) -> Result<(ProjectivePoint, [u8; 16], [u8; 32]), VrfError> {
        const GAMMA_LEN: usize = 33;
        const C_LEN: usize = 16;
        const Q_LEN: usize = 32;
        const C_START: usize = GAMMA_LEN;
        const S_START: usize = GAMMA_LEN + C_LEN;
        let gamma_string: [u8; GAMMA_LEN] = match pi_string[0..C_START].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(VrfError::DecodeProofError),
        };
        let c: [u8; C_LEN] = match pi_string[C_START..S_START].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(VrfError::DecodeProofError),
        };
        let s: [u8; Q_LEN] = match pi_string[S_START..].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(VrfError::DecodeProofError),
        };
        let gamma_encoded_point = match EncodedPoint::from_bytes(gamma_string) {
            Ok(bytes) => bytes,
            Err(_) => return Err(VrfError::DecodeProofError),
        };
        let gamma = match AffinePoint::try_from(gamma_encoded_point) {
            Ok(point) => point,
            Err(_) => return Err(VrfError::DecodeProofError),
        };
        let gamma: ProjectivePoint = gamma.into();
        Ok((gamma, c, s))
    }
}

blueprint! {
    struct VrfOracleContract {
        fee_vault: Vault,
        jobs: HashMap<NonFungibleId, Vec<u8>>,
        owner_badge: ResourceAddress,
        receipt_minter: Vault,
        receipt_nft_address: ResourceAddress,
        pk_bytes: Vec<u8>,
        counter: u64,
    }

    impl VrfOracleContract {
        // When creating new oracle contract, provide the public key of the off-chain oracle node to store for VRF verification.
        pub fn new(pk_hex_string: String) -> (ComponentAddress, Bucket) {
            let pk_bytes = hex::decode(pk_hex_string);
            assert!(pk_bytes.is_ok(), "Public key hex string decode error");
            let pk_bytes = pk_bytes.unwrap();

            let owner_badges = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Owner Badge")
                .initial_supply(1);

            let receipt_minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Receipt Minter")
                .initial_supply(1);

            let receipt_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .mintable(
                    rule!(require(receipt_minter_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(receipt_minter_badge.resource_address())),
                    LOCKED,
                )
                .no_initial_supply();

            let component = Self {
                fee_vault: Vault::new(RADIX_TOKEN),
                jobs: HashMap::new(),
                owner_badge: owner_badges.resource_address(),
                receipt_minter: Vault::with_bucket(receipt_minter_badge),
                receipt_nft_address: receipt_nft_address,
                pk_bytes,
                counter: 0,
            }
            .instantiate();

            let access_rules = AccessRules::new()
                .method(
                    "withdraw_all",
                    rule!(require(owner_badges.resource_address())),
                )
                .default(rule!(allow_all));

            (
                component.add_access_check(access_rules).globalize(),
                owner_badges,
            )
        }

        pub fn withdraw_all(&mut self) -> Bucket {
            self.fee_vault.take_all()
        }

        // This function can only be tested manually be copy and pasting the input seed to the off-chain vrf prover.
        // See request_randomness_by_counter for deterministic seed generation for automated tests.
        pub fn request_randomness(&mut self, payment: Bucket) -> Bucket {
            // take payment and add to fee_vault
            self.fee_vault.put(payment);

            // mint new non-fungible receipt as job id to return to requestor
            let receipt = self.receipt_minter.authorize(|| {
                let receipt_nft_manager: &ResourceManager =
                    borrow_resource_manager!(self.receipt_nft_address);

                // Would be nice to have a description of the NFT receipt
                receipt_nft_manager.mint_non_fungible(&NonFungibleId::random(), MemberData {})
            });

            let receipt_id = receipt.non_fungible::<MemberData>().id();

            // use random receipt id as alpha input seed
            let alpha = receipt_id.clone();
            // make off-chain oracle node request for random number
            info!(
                "Requesting VRF with input {} for receipt {}",
                alpha, receipt_id
            );

            receipt
        }

        // Deterministic seed generation. Provided to make unit testing easier for scrypto oracles challenge.
        // Should avoid using in production.
        pub fn request_randomness_by_counter(&mut self, payment: Bucket) -> Bucket {
            // take payment and add to fee_vault
            self.fee_vault.put(payment);
            // generate UUID as random input seed and use as jobId
            // let alpha = Runtime::generate_uuid();
            // self.jobs.insert(alpha, caller);
            // make off-chain oracle node request for random number
            // info!("Requesting VRF with input {} for caller component {}", alpha, caller);

            // mint new non-fungible receipt
            let receipt = self.receipt_minter.authorize(|| {
                let receipt_nft_manager: &ResourceManager =
                    borrow_resource_manager!(self.receipt_nft_address);

                // Would be nice to have a description of the NFT receipt
                receipt_nft_manager
                    .mint_non_fungible(&NonFungibleId::from_u64(self.counter), MemberData {})
            });
            self.counter += 1;

            let receipt_id = receipt.non_fungible::<MemberData>().id();

            // use random receipt id as alpha input seed
            let alpha = receipt_id.clone();
            // make off-chain oracle node request for random number
            info!(
                "Requesting VRF with input {} for receipt {}",
                alpha, receipt_id
            );

            receipt
        }

        // Allow user to set seed. Provided to verify against VRF Specification examples in scrypto oracles challenge.
        // DO NOT USE IN PRODUCTION.
        pub fn request_randomness_with_seed(&mut self, payment: Bucket, input_bytes: String) -> Bucket {
            // take payment and add to fee_vault
            self.fee_vault.put(payment);
            // generate UUID as random input seed and use as jobId
            // let alpha = Runtime::generate_uuid();
            // self.jobs.insert(alpha, caller);
            // make off-chain oracle node request for random number
            // info!("Requesting VRF with input {} for caller component {}", alpha, caller);

            // mint new non-fungible receipt
            let receipt = self.receipt_minter.authorize(|| {
                let receipt_nft_manager: &ResourceManager =
                    borrow_resource_manager!(self.receipt_nft_address);

                // Would be nice to have a description of the NFT receipt
                receipt_nft_manager
                    // .mint_non_fungible(&NonFungibleId::from_u64(self.counter), MemberData {})
                    .mint_non_fungible(&NonFungibleId::from_bytes(hex::decode(input_bytes).unwrap()), MemberData {})
            });
            self.counter += 1;

            let receipt_id = receipt.non_fungible::<MemberData>().id();

            // use random receipt id as alpha input seed
            let alpha = receipt_id.clone();
            // make off-chain oracle node request for random number
            info!(
                "Requesting VRF with input {} for receipt {}",
                alpha, receipt_id
            );

            receipt
        }


        // Function for the off-chain oracle to call to provide vrf proof for given input seed alpha.
        pub fn fullfill_randomness_request(
            &mut self,
            alpha_hex_string: String,
            proof_hex_string: String,
        ) {
            let alpha_bytes = hex::decode(alpha_hex_string);
            assert!(alpha_bytes.is_ok(), "Job id hex string decode error");
            let alpha_bytes = alpha_bytes.unwrap();

            let proof_bytes = hex::decode(proof_hex_string);
            assert!(proof_bytes.is_ok(), "proof hex string decode error");
            let proof_bytes = proof_bytes.unwrap();

            let p256_vrf = EcvrfCiphersuite::new(0x02, b"P256_XMD:SHA-256_SSWU_NU_");
            let randomness = p256_vrf.ecvrf_verify(&self.pk_bytes, &alpha_bytes, &proof_bytes);
            assert!(randomness.is_ok(), "Verify proof failed");
            let random_bytes = randomness.unwrap();
            let job_id = NonFungibleId::from_bytes(alpha_bytes);
            info!(
                "VRF proof accepted, storing random bytes {:x?} for job_id {:?}",
                random_bytes, job_id
            );
            self.jobs.insert(job_id, random_bytes.to_vec());
        }

        // After the consumer provides the NFT receipt badge, provide stored jobs results and also burn receipt so the VRF Proof for the given seed is one time use only. This also reduces the amount of stored jobs in the VrfOracleContract jobs hashmap.
        pub fn fetch_randomness(&mut self, receipt_badge: Bucket) -> Vec<u8> {
            assert!(
                receipt_badge.amount() == Decimal::ONE,
                "Only one receipt redemption allowed at a time"
            );
            let receipt_id = receipt_badge.non_fungible::<MemberData>().id();

            assert!(self.jobs.contains_key(&receipt_id), "Job not yet fulfilled");

            // Get stored random bytes, clear jobs storage of the input -> VRF Proof job, and burn receipt
            let random_bytes = self.jobs.remove(&receipt_id).unwrap();
            self.receipt_minter.authorize(|| {
                let receipt_nft_manager: &ResourceManager =
                    borrow_resource_manager!(self.receipt_nft_address);

                receipt_nft_manager.burn(receipt_badge);
            });

            info!(
                "Receipt accepted, returning random bytes {:?}",
                random_bytes
            );
            random_bytes
        }
    }
}
