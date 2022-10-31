//! Defines what is a voter card, which enables users to vote and participate in the DAO.
//! Note: we made the choice to pass the `current_epoch` as an argument of the functions instead of
//! calling `Runtime::current_epoch` to be able to unit test the file without using blueprints

use scrypto::prelude::{Decimal};
use scrypto::{dec, NonFungibleData};
use crate::decimal_maths::{exp};
use crate::proposal::ProposalStatus;

/// A voter card, records the different tokens locked and the epoch when they were.
/// It also records the votes that the voters casted and the voters they approve.
/// Adding an approved voter resets the locking epoch of the tokens.
#[derive(NonFungibleData)]
pub struct VoterCard {

    /// Id of the voter
    pub voter_id: u64,

    /// Total number of tokens held by the voter
    pub total_number_of_token : Decimal,

    /// Pairs of tokens with their lock period
    pub locked_tokens: Vec<(Decimal,u64)>,

    /// Votes casted by the voter
    pub votes : Vec<(usize, ProposalStatus)>,

    /// Voters that the voter approves
    pub approved_voters: Vec<u64>
}

impl VoterCard
{
    /// Instantiates a new voter card from an id and an amount of tokens
    ///
    /// # Arguments
    /// * `voter_id` - id to associate to the VoterCard
    ///
    /// # Examples
    /// ```
    /// use styx::voter_card::VoterCard;
    /// let new_voter_card = VoterCard::new(0);
    /// ```
    pub fn new(voter_id: u64) -> VoterCard
    {
        VoterCard
        {
            voter_id: voter_id,
            total_number_of_token : dec!(0),
            locked_tokens: vec![],
            votes: vec![],
            approved_voters: vec![]
        }
    }

    /// Adds tokens with a given lock_period to the VoterCard
    ///
    /// # Arguments
    /// * `amount` - amount of tokens to add
    /// * `lock_epoch`  - epoch when the tokens were locked
    ///
    /// # Examples
    /// ```
    /// use radix_engine::ledger::TypedInMemorySubstateStore;
    /// use scrypto::dec;
    /// use scrypto_unit::TestRunner;
    /// use styx::voter_card::VoterCard;
    ///
    /// let mut store = TypedInMemorySubstateStore::with_bootstrap();
    /// let mut test_runner = TestRunner::new(true, &mut store);
    /// let mut new_voter_card = VoterCard::new(0);
    /// new_voter_card.add_tokens(dec!(10), test_runner.get_current_epoch());
    /// assert_eq!(new_voter_card.total_number_of_token, dec!(10));
    /// ```
    pub fn add_tokens(&mut self, amount: Decimal, lock_epoch: u64)
    {
        self.total_number_of_token = self.total_number_of_token + amount;
        self.locked_tokens.push((amount, lock_epoch));
    }

    /// Returns a boolean stating whether the given voter can delegate its tokens to another given voter
    ///
    /// # Arguments
    /// * `other_voter` - voter to check whether they can delegate to
    ///
    /// # Examples
    /// ```
    /// use styx::voter_card::VoterCard;
    /// let mut new_voter_card = VoterCard::new(0);
    /// assert!(!new_voter_card.approves(1));
    /// ```
    pub fn approves(&self, other_voter: u64) -> bool
    {
        if self.voter_id == other_voter
        {
            return true;
        }
        for nfid in self.approved_voters.iter()
        {
            if *nfid == other_voter
            {
                return true;
            }
        }
        false
    }

    /// Adds a user to list of voters the voter approves, resets the lock epoch and merges
    /// all the locked tokens
    ///
    /// # Arguments
    /// * `other_voter` - VoterCard id of the user to add to possible delagatee
    /// * `current_epoch`  - current epoch
    ///
    /// # Examples
    /// ```
    /// use radix_engine::ledger::TypedInMemorySubstateStore;
    /// use scrypto_unit::TestRunner;
    /// use styx::voter_card::VoterCard;
    /// let mut store = TypedInMemorySubstateStore::with_bootstrap();
    /// let mut test_runner = TestRunner::new(true, &mut store);
    ///
    /// let mut new_voter_card = VoterCard::new(0);
    /// new_voter_card.approve(1, test_runner.get_current_epoch());
    /// assert!(new_voter_card.approves(1));
    /// ```
    pub fn approve(&mut self, other_voter: u64, current_epoch: u64)
    {
        if !self.approves(other_voter)
        {
            self.approved_voters.push(other_voter);
            self.merge(current_epoch);
        }
    }

    /// Returns a boolean stating if the given voter can vote for a given proposal.
    /// If they can vote for the proposal, the list of votes is updated.
    ///
    /// # Arguments
    /// * `proposal_id` - id of the Proposal
    /// * `current_status`  - status of the proposal
    ///
    /// # Examples
    /// ```
    /// use styx::proposal::ProposalStatus;
    /// use styx::voter_card::VoterCard;
    /// let mut new_voter_card = VoterCard::new(0);
    /// // Can vote during voting phase
    /// assert!(new_voter_card.try_vote_for(0, &ProposalStatus::VotingPhase));
    /// // Cannot vote twice
    /// assert!(!new_voter_card.try_vote_for(0, &ProposalStatus::VotingPhase));
    ///
    /// ```
    pub fn try_vote_for(&mut self, proposal_id: usize, current_status: &ProposalStatus) -> bool
    {
        if !current_status.is_voting_phase() && !current_status.is_suggestion_phase()
        {
            false
        }
        else
        {

            for (id,status) in self.votes.iter()
            {
                if *id == proposal_id
                {
                    // If the proposal id was found, then the voter can only vote if the status is Voting Phase
                    // And the previous status was suggestion phase
                    return match current_status
                    {
                        ProposalStatus::VotingPhase =>
                            {
                                if status.is_suggestion_phase()
                                {
                                    true
                                } else {
                                    false
                                }
                            }
                        _ => { false }
                    }
                }
            }

            // If nothing was found, add the vote to the votes and return true
            self.votes.push((proposal_id, current_status.clone()));
            true
        }
    }

    /// Retrieves a certain amount of locked tokens from the voter card
    /// Last elements of the list (ie. tokens added last) are retrived first
    ///
    /// # Arguments
    /// * `amount` - amount of tokens to retrieve
    ///
    /// # Examples
    /// ```
    /// use radix_engine::ledger::TypedInMemorySubstateStore;
    /// use scrypto::dec;
    /// use scrypto_unit::TestRunner;
    /// use styx::proposal::ProposalStatus;
    /// use styx::voter_card::VoterCard;
    ///
    /// let mut new_voter_card = VoterCard::new(0);
    /// let mut store = TypedInMemorySubstateStore::with_bootstrap();
    /// let mut test_runner = TestRunner::new(true, &mut store);
    ///
    /// new_voter_card.add_tokens(dec!(10), test_runner.get_current_epoch());
    /// println!("{}", new_voter_card.total_number_of_token);
    /// new_voter_card.retrieve_tokens(dec!(8));
    /// assert_eq!(new_voter_card.total_number_of_token, dec!(2));
    ///
    /// ```
    pub fn retrieve_tokens(&mut self, amount: Decimal)
    {
        assert!(amount <= self.total_number_of_token, "Cannot retrieve more tokens than owned");

        if amount == self.total_number_of_token
        {
            self.retrieve_all_tokens();
        }
        else
        {
            self.total_number_of_token -= amount;
            let mut amount_loop = amount;
            while amount_loop > dec!(0)
            {
                let (tokens,time) = self.locked_tokens.pop().unwrap();
                if tokens > amount
                {
                    self.locked_tokens.push( (tokens- amount_loop,time));
                }

                amount_loop = amount_loop - tokens;
            }
        }
    }

    /// Retrieves amm locked tokens from the voter card
    ///
    /// # Examples
    /// ```
    /// use radix_engine::ledger::TypedInMemorySubstateStore;
    /// use scrypto::dec;
    /// use scrypto_unit::TestRunner;
    /// use styx::proposal::ProposalStatus;
    /// use styx::voter_card::VoterCard;
    ///
    /// let mut new_voter_card = VoterCard::new(0);
    /// let mut store = TypedInMemorySubstateStore::with_bootstrap();
    /// let mut test_runner = TestRunner::new(true, &mut store);
    ///
    /// new_voter_card.add_tokens(dec!(10), test_runner.get_current_epoch());
    /// new_voter_card.retrieve_all_tokens();
    /// assert_eq!(new_voter_card.total_number_of_token, dec!(0));
    ///
    /// ```
    pub fn retrieve_all_tokens(&mut self) -> Decimal
    {
        let total_number_of_token = self.total_number_of_token;
        self.total_number_of_token = dec!(0);
        self.locked_tokens = vec![];
        total_number_of_token
    }

    /// Computes the voting power associated to a voter card
    /// For more details on the function choice, please read the whitepaper.
    ///
    /// # Arguments
    /// * `current_epoch` - current epoch
    ///
    /// # Examples
    /// ```
    /// use radix_engine::ledger::TypedInMemorySubstateStore;
    /// use scrypto::dec;
    /// use scrypto_unit::TestRunner;
    /// use styx::voter_card::VoterCard;
    ///
    /// let mut new_voter_card = VoterCard::new(0);
    /// let mut store = TypedInMemorySubstateStore::with_bootstrap();
    /// let mut test_runner = TestRunner::new(true, &mut store);
    ///
    /// new_voter_card.add_tokens(dec!(56), test_runner.get_current_epoch());
    /// let current_epoch = test_runner.get_current_epoch();
    /// test_runner.set_current_epoch(current_epoch + 1000);
    ///
    /// let votes = new_voter_card.voting_power(test_runner.get_current_epoch());
    /// ```
    pub fn voting_power(&self, current_epoch: u64) -> Decimal
    {
        let mut total = Decimal::zero();
        for (tokens,time_tmp) in &self.locked_tokens
        {
            // In our tests, time can get negative so we transform in Decimal before subtracting
            let time = current_epoch - *time_tmp;
            total = total + *tokens * Self::sub_voting_function(time);
        }

        total
    }

    /// Internal function used to compute voting power
    ///
    /// # Arguments
    /// * `time` - time variable
    ///
    fn sub_voting_function(time: u64) -> Decimal
    {

        if time==0
        {
            return Decimal::zero();
        }


        let exp = exp(- dec!(2016) / time );
        let time_multiplicator =  ( exp - 1 )/ (exp + 1)  + 1;
        time_multiplicator
    }

    /// Internal function used to merge the list of locked tokens
    ///
    /// # Arguments
    /// * `current_epoch` - current epoch
    ///
    fn merge(&mut self, current_epoch: u64)
    {
        if !self.locked_tokens.is_empty()
        {
            self.locked_tokens = vec![(self.total_number_of_token, current_epoch)];
        }
    }
}

#[cfg(test)]
mod tests
{
    use radix_engine::ledger::TypedInMemorySubstateStore;
    use scrypto::dec;
    use scrypto_unit::TestRunner;
    use crate::proposal::ProposalStatus;
    use crate::voter_card::VoterCard;


    #[test]
    fn test_correct_initialization()
    {
        let mut store = TypedInMemorySubstateStore::with_bootstrap();
        let mut test_runner = TestRunner::new(true, &mut store);

        let mut voter_card = VoterCard::new(0);
        voter_card.add_tokens(dec!(45), test_runner.get_current_epoch());
        assert_eq!(voter_card.locked_tokens, vec![(dec!(45), test_runner.get_current_epoch())]);
        assert!(voter_card.approves(voter_card.voter_id));
    }

    #[test]
    fn test_delegate()
    {
        let mut store = TypedInMemorySubstateStore::with_bootstrap();
        let mut test_runner = TestRunner::new(true, &mut store);
        let mut voter_card = VoterCard::new(0);
        voter_card.approve(1, test_runner.get_current_epoch());

        assert!(voter_card.approves(1));
    }

    #[test]
    fn test_vote_for_suggestion_phase()
    {
        let mut voter_card = VoterCard::new(0);
        let vote = voter_card.try_vote_for(0, &ProposalStatus::SuggestionPhase);

        assert!(vote);
        assert_eq!(voter_card.votes.get(0).unwrap().0, 0);
    }

    #[test]
    fn test_vote_for_voting_phase()
    {
        let mut voter_card = VoterCard::new(0);
        let vote = voter_card.try_vote_for(0, &ProposalStatus::VotingPhase);

        assert!(vote);
        assert_eq!(voter_card.votes.get(0).unwrap().0, 0)
    }

    #[test]
    fn test_already_vote_suggestion_phase()
    {
        let mut voter_card = VoterCard::new(0);
        voter_card.try_vote_for(0, &ProposalStatus::SuggestionPhase);
        let vote = voter_card.try_vote_for(0, &ProposalStatus::SuggestionPhase);

        assert!(!vote);
    }

    #[test]
    fn test_already_vote_suggestion_phase_2()
    {
        let mut voter_card = VoterCard::new(0);
        voter_card.try_vote_for(0, &ProposalStatus::VotingPhase);
        let vote = voter_card.try_vote_for(0, &ProposalStatus::SuggestionPhase);

        assert!(!vote);
    }

    #[test]
    fn test_already_vote_voting_phase()
    {
        let mut voter_card = VoterCard::new(0);
        voter_card.try_vote_for(0, &ProposalStatus::VotingPhase);
        let vote = voter_card.try_vote_for(0, &ProposalStatus::VotingPhase);

        assert!(!vote);
    }

    #[test]
    fn test_multiple_votes()
    {
        let mut voter_card = VoterCard::new(0);
        for i in 0..10
        {
            let vote = voter_card.try_vote_for(i, &ProposalStatus::VotingPhase);
            assert!(vote);
            assert_eq!(voter_card.votes.get(i).unwrap().0, i);
        }
    }
}
