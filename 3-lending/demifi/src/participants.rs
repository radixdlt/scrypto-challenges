//! Provides for managing catalogs of participants with trust
//! relationships between those participants.
//!
//! This module provides a generic framework for managing identities in
//! any Scrypto-based system that needs them. It allows for users to
//! obtain Participant NFTs that they use to identify themselves, and
//! for those NFTs to attract endorsements from others. This provides
//! a tool that users of this system can employ to gauge how
//! trustworthy someone else is.
//!
//! # What is a catalog?
//!
//! When a Participants component is instantiated from this blueprint
//! it sets up a new NFT resource to use for its Participant NFTs. The
//! intent of the design is for different Participants instances to be
//! independent and not communicate with eachother. We sometimes refer
//! to the set of Participants tied to one such instance / NFT
//! resource address as a Participant catalog.
//!
//! # Sponsorships
//!
//! A Participant can have at most one other Participant sponsoring
//! him. The sponsor is typically someone who vouches for your
//! legitimacy: they have done their due dilligence on determining who
//! you are and have transferred that knowledge into the system
//! through the sponsorship mechanism.
//!
//! In some use cases perhaps all Participants who want to take part
//! in the online activity that makes use of this system will be
//! required to have one particular sponsor. For example if you have a
//! user account in an existing service then that service may sponsor
//! one Participant for each of their users and this sponsorship is
//! what gives you access to their ledger-based offerings. Other
//! Participants might exist but so long as they don't have the
//! service itself sponsoring them they can't get much done.
//!
//! In other use cases you may just want to attract a sponsor who is
//! trusted or famous within the community in order to benefit from
//! their reputation.
//!
//! In yet other uses perhaps sponsorships aren't even used.
//!
//! # Endorsements
//!
//! A Participant can choose to endorse any number of other
//! Participants.  Endorsing someone signals that you trust this
//! person and you feel like they are a dependable sort. Other people
//! who respect your judgement may then decide to also trust them.
//!
//! A system may use endorsement to judge how trustworthy someone is
//! by analyzing how many endorsements they have, who is endorsing
//! them, etc.
//!
//! # Identity
//!
//! Participant NFTs are suitable as use for identity purposes so long
//! as you have a strong system in place for vetting applicants. If
//! you need strong control over this then the suggestion is to use
//! centralized sponsorship as your mechanism of approving or not any
//! given Participant for the use of your system.
//!
//! Typically a user will provide his Participant NFT as Proof into a
//! protected Scrypto method.
//!
//! Note that Participants NFTs have an "id_ref" field. When this
//! field is used it is intended to point at some other identity
//! resource that belongs to the user. As of yet this is not a fully
//! developed concept and so it is simply a text field the user can
//! change at a whim. There is no real security or authentication
//! backing it at the moment and so it should be considered purely
//! advisory information.
//!
use scrypto::prelude::*;

/// This is the Participant NFT data.
///
/// Most of its contents can be changed at the whim of the user. The
/// key identifying constant to use in relation to this when doing
/// authentication checks etc. is its on-ledger
/// NonFungibleAddress. (Or if in a context where the NFT
/// ResourceAddress is already given, the NonFungibleId can be used.)
///
/// We can only endorse or sponsor users of the same Participants
/// catalog as ourselves and so we use their NonFungibleIds to
/// identify them in these situations.
#[derive(NonFungibleData)]
pub struct Participant {
    /// The name of the Participant. The user can change this at any
    /// time.
    #[scrypto(mutable)]
    name: String,

    /// A link to a resource that describes the user. The user can
    /// change this at any time.
    #[scrypto(mutable)]
    url: String,

    /// A link to an identifying document or resource. The user can
    /// change this at any time.
    #[scrypto(mutable)]
    id_ref: String,

    /// Our sponsor, if any. There is a process for acquiring a
    /// sponsor, see [Participants::expect_sponsor] and
    /// [Participants::sponsor] for details.
    #[scrypto(mutable)]
    sponsor: Option<NonFungibleId>,

    /// The sponsor we expect, if any. The user can change this at any
    /// time.
    #[scrypto(mutable)]
    expect_sponsor: Option<NonFungibleId>,

    /// The other users that we endorse. The user can change this at
    /// any time.
    #[scrypto(mutable)]
    endorsing: HashSet<NonFungibleId>,
}

blueprint! {

    struct Participants {
        /// The NFT resource address of this Participant catalog.
        nft_address: ResourceAddress,

        /// Our admin badge, used to manage the Participent NFTs.
        admin_badge: Vault,

        /// The id of the Participant that created the catalog. This
        /// is a useful "root user" if you want strong centralized
        /// control over the catalog sponsorship tree etc. It is also
        /// the proof you can use that you created / own this catalog.
        catalog_creator: NonFungibleId,
    }

    impl Participants {

        /// Creates a new Participants catalog.
        ///
        /// You can optionally provide names that will be used in the
        /// metadata for the admin badge, the NFT resource and the
        /// root participant.
        ///
        /// A bucket will be returned from this method containing the
        /// catalog owner Participant NFT. It also returns, in order:
        ///
        /// 0. The component address of the new Participants instance.
        /// 1. The resource address of the Participant NFT resource.
        /// 2. The bucket with the NFT
        /// 3. The id of the NFT
        ///
        /// ---
        ///
        /// **Access control:** Anyone can create a Participants
        /// catalog. The trick is getting people to use it.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/instantiate_participant_catalog.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/instantiate_participant_catalog.rtm")]
        /// ```
        pub fn instantiate_participant_catalog(
            admin_badge_name: Option<String>,
            nft_resource_name: Option<String>,
            root_participant_name: Option<String>)
            -> (ComponentAddress, ResourceAddress, Bucket, NonFungibleId)
        {
            let badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", admin_badge_name.unwrap_or(
                    "Loan Participants NFT control badge".to_string()))
                .initial_supply(1);
            let nft_address = ResourceBuilder::new_non_fungible()
                .metadata("name", nft_resource_name.unwrap_or(
                    "Loan Participant NFT".to_string()))
                .mintable(rule!(require(badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(badge.resource_address())), LOCKED)
                .no_initial_supply();
            let badge = Vault::with_bucket(badge);

            let (nft, nfid) = Participants::create_participant(
                &badge,
                nft_address,
                root_participant_name.unwrap_or(
                    "Loan market creator".to_string()),
                "".to_string(),
                "".to_string(),
                None);

            let participants =
                Self {
                    nft_address,
                    admin_badge: badge,
                    catalog_creator: nfid.clone(),
                }
            .instantiate()
                .globalize();

            // All methods that require access control in this blueprint
            // handle this themselves through the Proof instances provided
            // to them.
            
            (participants, nft_address, nft, nfid)
        }

        /// Call this method to create your very own Participant NFT.
        ///
        /// (Then try to attract sponsorships and endorsements so that
        /// people will start taking you seriously.)
        ///
        /// ---
        ///
        /// **Access control:** Anyone can create a Participant.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/new_participant.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/new_participant.rtm")]
        /// ```
        pub fn new_participant(&self,
                               name: String,
                               url: String,
                               id_ref: String,
                               expect_sponsor: Option<NonFungibleId>) -> (Bucket, NonFungibleId)
        {
            Participants::create_participant(&self.admin_badge,
                                             self.nft_address,
                                             name,
                                             url,
                                             id_ref,
                                             expect_sponsor)
        }

        /// Call this method to sponsor someone who is expecting you
        /// to sponsor them.
        ///
        /// This method will panic if the target Participant is not
        /// expecting you (see [Participants::expect_sponsor]).
        ///
        /// ---
        ///
        /// **Access control:** Allows only the participant in the
        /// proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/sponsor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/sponsor.rtm")]
        /// ```
        pub fn sponsor(&self,
                       sponsor: Proof,
                       target: NonFungibleId)
        {
            let (sponsor_nfid, _, _) = self.check_and_retrieve_participant(sponsor);
            let (target, _, mut target_data) = self.retrieve_participant_from_id(target);
            assert_eq!(target_data.expect_sponsor.unwrap(), sponsor_nfid,
                       "Target participant is not expecting this sponsor");

            target_data.sponsor = Some(sponsor_nfid);
            target_data.expect_sponsor = None;
            self.save_participant_data(&target, target_data);
        }

        /// Call this method to witdhraw your sponsorship from someone.
        ///
        /// This method will panic if you are not the sponsor of the
        /// target Participant.
        ///
        /// ---
        ///
        /// **Access control:** Allows only the participant in the
        /// proof, and this participant must be currently sponsoring
        /// the target.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/unsponsor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/unsponsor.rtm")]
        /// ```
        pub fn unsponsor(&self,
                         sponsor: Proof,
                         target: NonFungibleId)
        {
            let (sponsor_nfid, _, _) = self.check_and_retrieve_participant(sponsor);
            let (target, _, mut target_data) = self.retrieve_participant_from_id(target);
            assert_eq!(target_data.sponsor.unwrap(), sponsor_nfid,
                       "You are not sponsoring this participant");

            target_data.sponsor = None;
            self.save_participant_data(&target, target_data);
        }

        /// Call this method to register that you expect to receive a
        /// specific sponsorship.
        ///
        /// Someone can only sponsor you if you have first used this
        /// method to confirm that you expect them to do so.
        ///
        /// This method will fail if you already have a sponsor.
        ///
        /// ---
        ///
        /// **Access control:** Allows only the participant in the
        /// proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/expect_sponsor.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/expect_sponsor.rtm")]
        /// ```
        pub fn expect_sponsor(&self,
                              participant: Proof,
                              expected_sponsor: NonFungibleId)
        {
            let (participant_nfid, _, mut participant_data) =
                self.check_and_retrieve_participant(participant);
            assert!(participant_data.sponsor.is_none(),
                    "You already have a sponsor");
            participant_data.expect_sponsor = Some(expected_sponsor);
            self.save_participant_data(&participant_nfid, participant_data);
        }

        /// Call this method to change your name.
        ///
        /// ---
        ///
        /// **Access control:** Allows only the participant in the
        /// proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/change_name.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/change_name.rtm")]
        /// ```
        pub fn change_name(&self,
                           participant: Proof,
                           new_name: String)
        {
            let (participant_nfid, _, mut participant_data) =
                self.check_and_retrieve_participant(participant);
            participant_data.name = new_name;
            self.save_participant_data(&participant_nfid, participant_data);
        }
        
        /// Call this method to change your URL.
        ///
        /// ---
        ///
        /// **Access control:** Allows only the participant in the
        /// proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/change_url.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/change_url.rtm")]
        /// ```
        pub fn change_url(&self,
                          participant: Proof,
                          new_url: String)
        {
            let (participant_nfid, _, mut participant_data) =
                self.check_and_retrieve_participant(participant);
            participant_data.url = new_url;
            self.save_participant_data(&participant_nfid, participant_data);
        }

        /// Call this method to change your ID ref.
        ///
        /// ---
        ///
        /// **Access control:** Allows only the participant in the
        /// proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/change_id_ref.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/change_id_ref.rtm")]
        /// ```
        pub fn change_id_ref(&self,
                          participant: Proof,
                          new_id_ref: String)
        {
            let (participant_nfid, _, mut participant_data) =
                self.check_and_retrieve_participant(participant);
            participant_data.id_ref = new_id_ref;
            self.save_participant_data(&participant_nfid, participant_data);
        }

        /// Call this method to endorse another Participant.
        ///
        /// ---
        ///
        /// **Access control:** Endorses on behalf of the participant
        /// in the proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/endorse.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/endorse.rtm")]
        /// ```
        pub fn endorse(&self,
                       participant: Proof,
                       target: NonFungibleId)
        {
            let (participant_nfid, _, mut participant_data) =
                self.check_and_retrieve_participant(participant);
            participant_data.endorsing.insert(target);
            self.save_participant_data(&participant_nfid, participant_data);
        }

        /// Call this method to stop endorsing another Participant.
        ///
        /// ---
        ///
        /// **Access control:** Unendorses on behalf of the
        /// participant in the proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/unendorse.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/unendorse.rtm")]
        /// ```
        pub fn unendorse(&self,
                         participant: Proof,
                         target: NonFungibleId)
        {
            let (participant_nfid, _, mut participant_data) =
                self.check_and_retrieve_participant(participant);
            participant_data.endorsing.remove(&target);
            self.save_participant_data(&participant_nfid, participant_data);
        }

        /// Queries whether one Participant is currently endorsing
        /// another.
        ///
        /// Returns true if "myself" endorses "endorsed_id".
        ///
        /// ---
        ///
        /// **Access control:** Read only, allows anyone
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/do_i_endorse.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/do_i_endorse.rtm")]
        /// ```
        pub fn do_i_endorse(&self,
                            myself: NonFungibleId,
                            endorsed_id: NonFungibleId) -> bool
        {
            let (_, _, myself_data) =
                self.retrieve_participant_from_id(myself);
            myself_data.endorsing.contains(&endorsed_id)
        }

        /// Retrieves data about a Participant.
        ///
        /// Returns, in order:
        ///
        /// 0. Name
        /// 1. URL
        /// 2. ID ref
        /// 3. Sponsor address if any
        /// 4. Expected sponsor if any
        ///
        /// ---
        ///
        /// **Access control:** Read only, allows anyone
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/read_data.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/read_data.rtm")]
        /// ```
        pub fn read_data(&self,
                         participant: NonFungibleId)
                         -> (String, String, String,
                             Option<NonFungibleAddress>, Option<NonFungibleAddress>)
        {
            let (_, _, participant_data) =
                self.retrieve_participant_from_id(participant);

            (participant_data.name,
             participant_data.url,
             participant_data.id_ref,
             if let Some(nfid) = participant_data.sponsor
             { Some(NonFungibleAddress::new(self.nft_address, nfid.clone())) } else { None },
             if let Some(nfid) = participant_data.expect_sponsor
             { Some(NonFungibleAddress::new(self.nft_address, nfid.clone())) } else { None })
        }

        /// Retrieves a Participant's endorsement list. That is, a
        /// list of everyone he endorses.
        ///
        /// ---
        ///
        /// **Access control:** Read only, allows anyone
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/read_endorsements.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/read_endorsements.rtm")]
        /// ```
        pub fn read_endorsements(&self,
                                 participant: NonFungibleId)
                                 -> HashSet<NonFungibleId>
        {
            let (_, _, participant_data) =
                self.retrieve_participant_from_id(participant);
            participant_data.endorsing
        }

        /// Retrieves the resource address of our Participant NFTs.
        ///
        /// ---
        ///
        /// **Access control:** Read only, allows anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/read_participants_nft_addr.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/read_participants_nft_addr.rtm")]
        /// ```
        pub fn read_participants_nft_addr(&self) -> ResourceAddress {
            self.nft_address
        }

        /// Retrieves address of the catalog creator's Participant
        /// NFT.
        ///
        /// ---
        ///
        /// **Access control:** Read only, allows anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/participants/read_catalog_creator.rtm`
        /// ```text
        #[doc = include_str!("../rtm/participants/read_catalog_creator.rtm")]
        /// ```
        pub fn read_catalog_creator(&self) -> NonFungibleAddress {
            NonFungibleAddress::new(self.nft_address, self.catalog_creator.clone())
        }

        //
        // Internal utility methods follow
        //

        /// Helper function to create a new Participant.
        fn create_participant(admin_badge: &Vault,
                              nft_address: ResourceAddress,
                              name: String,
                              url: String,
                              id_ref: String,
                              expect_sponsor: Option<NonFungibleId>) -> (Bucket, NonFungibleId)
        {
            let nfid: NonFungibleId = NonFungibleId::random();
            let nft: Bucket = admin_badge.authorize(||
                borrow_resource_manager!(nft_address)
                    .mint_non_fungible(
                        &nfid,
                        Participant {
                            name,
                            url,
                            id_ref,
                            sponsor: None,
                            expect_sponsor,
                            endorsing: HashSet::new(),
                        }
                    )
            );
            (nft, nfid)
        }

        /// Produces a resource manager and participant from a
        /// Participant id; also returns the id itself.
        fn retrieve_participant_from_id(&self, non_fungible_id: NonFungibleId) 
                                        -> (NonFungibleId, &ResourceManager, Participant)
        {
            let nft_manager = borrow_resource_manager!(self.nft_address);
            let data = nft_manager.get_non_fungible_data(&non_fungible_id);
            (non_fungible_id, nft_manager, data)
        }

        /// Asserts that the Proof is for a Participant NFT of the
        /// catalog we're connected to, and returns useful objects for
        /// working with it.
        fn check_and_retrieve_participant(&self, nft: Proof)
                                          -> (NonFungibleId, &ResourceManager, Participant)
        { 
           assert_eq!(
                nft.resource_address(),
                self.nft_address,
                "Unsupported participant NFT"
            );
            assert_eq!(nft.amount(), dec!("1"),
                       "Use only one participant NFT at a time");
            let nfid = nft
                .non_fungible_ids()
                .into_iter()
                .collect::<Vec<NonFungibleId>>()[0]
                .clone();
            self.retrieve_participant_from_id(nfid)
        }

        /// Writes the Participant NFT data to the ledger.
        fn save_participant_data(&self, non_fungible_id: &NonFungibleId, data: Participant)
        {
            self.admin_badge.authorize(||  {
                borrow_resource_manager!(self.nft_address)
                    .update_non_fungible_data(&non_fungible_id, data);
            });
        }


    }
}
