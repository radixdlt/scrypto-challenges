/**
 *                              Streamdao is a DAO for content creators.
 *
 * The idea is quite simple, think of a voting system that you can use
 * on your Youtube or Twitch channel, where subscribing members can propose ideas.
 *
 *Example:
 * Imagine that Dan Hughes (Founder Radix) decided to create a new cryptocurrency, so
 * he decides that the community that follows him on his Twitch channel will decide
 * what will be the name of this new currency. For that, he can use the component "Streamdao".
 * Your subscribers can vote with the rewards they have claimed for being subscribed.
 * subscribers cannot buy rewards for voting, the only way to obtain them is to sign the
 * channel and claim rewards while watching content.
 *
 * The "stream" component implements a Dapp idea, where it is possible to create an account (membership),
 * Create a channel, and propose, in addition to claiming rewards.
 *
 * The "DAO" Component implements a simple voting system, where each channel "has its DAO".
 *
 */
mod dao;
mod stream;
mod structs;
