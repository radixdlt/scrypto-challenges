import { NextPage } from "next";
import Navbar from "../components/Navbar";

const About: NextPage = () => {
  return (
    <div>
      <div className="min-h-screen bg-nordbg2 text-nordtext flex p-10 flex-col">
        <h1 className="font-extrabold text-4xl mb-5">About Trifold</h1>
        <div className="text-lg">
          Trifold is built on scrypto and Next.js (React) with Tailwind CSS.
          Trifold has many common DeFi lending features:
          <ul className="list-disc pl-10 py-5">
            <li>
              Lenders are able to deposit XRD and will be compensated one to one
              in lnXRD for the amount of XRD they deposit.
            </li>
            <li>
              Borrowers are able to borrow XRD from the pool with no collateral,
              and will be charged interest per epoch.
            </li>
            <li>
              Lenders are able to withdraw XRD, along with any profit they have
              earned through interest, by returning the lnXRD they have
              recieved, provided the pool has enough liquidity to cover the
              withdrawal.
            </li>
          </ul>
          As stated earlier, Trifold's namesake is due to its three factors of
          security.
          <h2 className="font-extrabold text-3xl my-5">
            Offchain Verification
          </h2>
          The instantiator of the contract is responsible for the offchain
          governance of the contract. They are given an admin badge which is
          used to approve borrowers. Off the chain, a borrower approaches the
          instantiator of the contract to request a borrower approval. The
          instantiator of the contract is then given the opportunity to approve
          or deny the borrower in real life. If the borrower is approved, the
          instantiator can then place an approval request on the `/admin`
          endpoint. The instantiator should provide the name of the borrower (or
          the company's name), the RADIX address of the holder, and a website
          which claims responsibility for the address.
          <h2 className="font-extrabold text-3xl my-5">Karma</h2>
          Karma is a system within the Trifold contract which allows for
          borrowers to increase their lending limits depending on their past
          reputation. Borrowers are initially given a set amount of karma, which
          can be traded 1 to 1 for XRD. If they take out a loan, a corresponding
          amount of karma will be burned. If the borrower defaults, the borrower
          will lose the amount of karma they had initially. However, if the
          borrower pays back the loan, now that interest has been accrued, the
          borrower will gain more karma than they had initially. This allows for
          borrowers to continue to increase their lending limits, and prevents
          malicious borrowers from abusing the system.
          <h2 className="font-extrabold text-3xl my-5">Emergency Lockdown</h2>
          Holders of the contract can request an emergency shutdown of the
          contract by sending all of their lnXRD to a specified address. This
          will compensate the holder with a lockdown special token, and if the
          amount of lnXRD is over 50% of the total public supply of lnXRD, it
          shows that a majority of the lnXRD holders believe that the contract
          is in a bad state. This will cause prevent the contract from accepting
          any new loans, approving borrowers, withdrawing lends, and all major
          functions, until the lockdown is ended by withdrawing the lnXRD from
          the lockdown vault by returning the lockdown token.
        </div>
      </div>
      <Navbar />
    </div>
  );
};

export default About;
