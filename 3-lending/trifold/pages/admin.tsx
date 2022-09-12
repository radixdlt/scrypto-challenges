import { NextPage } from "next";
import { signTransaction } from "pte-browser-extension-sdk";
import { ManifestBuilder } from "pte-sdk";
import { useState } from "react";
import Navbar from "../components/Navbar";
import useLocalStorage from "../src/localStorage";

const Admin: NextPage = () => {
  const [componentAddress, setComponentAddress] = useLocalStorage(
    "componentAddress",
    ""
  );
  const [accountAddress, setAccountAddress] = useLocalStorage(
    "accountAddress",
    ""
  );
  const [adminAddress, setAdminAddress] = useLocalStorage("adminAddress", "");
  const [packageAddress, setPackageAddress] = useLocalStorage(
    "packageAddress",
    ""
  );

  const [name, setName] = useState("");
  const [website, setWebsite] = useState("");
  const [address, setAddress] = useState("");

  console.log(componentAddress, accountAddress, adminAddress);

  const createBorrowerBadge = async (
    name: string,
    website: string,
    address: string
  ) => {
    // create a borrower badge
    const manifest = new ManifestBuilder()
      .withdrawFromAccount(accountAddress, adminAddress)
      .takeFromWorktop(adminAddress, "admin")
      .createProofFromBucket("admin", "adminproof")
      .pushToAuthZone("adminproof")
      .callMethod(componentAddress, "approve_borrower", [
        `ComponentAddress("${address}")`,
        `"${name}"`,
        `"${website}"`,
      ])
      .callMethodWithAllResources(address, "deposit_batch")
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
          <h1 className="font-extrabold text-4xl mb-5">
            Create Borrower Badge
          </h1>
          <input
            type="text"
            name="name"
            id="name"
            placeholder="Name"
            className="bg-gray-600"
            onChange={(e) => setName(e.target.value)}
          />
          <input
            type="text"
            name="website"
            id="website"
            placeholder="Website"
            className="bg-gray-600"
            onChange={(e) => setWebsite(e.target.value)}
          />
          <input
            type="text"
            name="address"
            id="address"
            placeholder="Address"
            className="bg-gray-600"
            onChange={(e) => setAddress(e.target.value)}
          />
          <button
            className="py-4 px-10 mt-10 font-semibold text-4xl bg-nordhighlight rounded-3xl"
            onClick={() => createBorrowerBadge(name, website, address)}
          >
            Create
          </button>
        </div>
        <h1 className="font-bold text-4xl my-5 text-center">
          Note: This will only work with a admin's badge.
        </h1>
      </div>
      <Navbar />
    </div>
  );
};

export default Admin;
