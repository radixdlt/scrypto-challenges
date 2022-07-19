import { NextPage } from "next";
import { signTransaction } from "pte-browser-extension-sdk";
import { ManifestBuilder } from "pte-sdk";
import { useState } from "react";
import Navbar from "../components/Navbar";
import useLocalStorage from "../src/localStorage";

const Borrow: NextPage = () => {
  const [componentAddress, setComponentAddress] = useLocalStorage(
    "componentAddress",
    ""
  );
  const [accountAddress, setAccountAddress] = useLocalStorage(
    "accountAddress",
    ""
  );
  const [resourceAddress, setResourceAddress] = useLocalStorage(
    "resourceAddress",
    ""
  );
  const [adminAddress, setAdminAddress] = useLocalStorage("adminAddress", "");
  const [packageAddress, setPackageAddress] = useLocalStorage(
    "packageAddress",
    ""
  );
  const [borrowerBadgeAddress, setBorrowerBadgeAddress] = useLocalStorage(
    "borrowerBadgeAddress",
    ""
  );
  const [karmaAddress, setKarmaAddress] = useLocalStorage("karmaAddress", "");

  const [amount, setAmount] = useState("");

  const borrow = async (amount: string) => {
    const manifest = new ManifestBuilder()
      .withdrawFromAccount(accountAddress, borrowerBadgeAddress)
      .takeFromWorktop(borrowerBadgeAddress, "borrower")
      .createProofFromBucket("borrower", "borrowerproof")
      .withdrawFromAccountByAmount(accountAddress, Number(amount), karmaAddress)
      .takeFromWorktop(karmaAddress, "karma")
      .callMethod(componentAddress, "borrow", [
        'Bucket("karma")',
        'Proof("borrowerproof")',
      ])
      .build()
      .toString();
    console.log(manifest);
    const receipt = await signTransaction(manifest);
    console.log(receipt);
  };
  return (
    <div>
      <div className="min-h-screen bg-nordbg2 text-nordtext flex p-5 flex-col">
        <div className="flex flex-col bg-nordbg rounded-3xl p-10">
          <h1 className="font-extrabold text-4xl mb-5">Borrow</h1>
          <input
            type="text"
            name="amount"
            id="amount"
            placeholder="Amount"
            className="bg-gray-600"
            onChange={(e) => setAmount(e.target.value)}
          />
          <button
            className="py-4 px-10 mt-10 font-semibold text-4xl bg-nordhighlight rounded-3xl"
            onClick={() => borrow(amount)}
          >
            Borrow
          </button>
        </div>
        <h1 className="font-bold text-4xl my-5 text-center">
          Note: This will only work with a borrower's badge.
        </h1>
      </div>
      <Navbar />
    </div>
  );
};

export default Borrow;
