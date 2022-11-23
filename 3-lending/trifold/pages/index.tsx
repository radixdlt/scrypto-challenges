import type { NextPage } from "next";
import { DefaultApi, ManifestBuilder } from "pte-sdk";
import Head from "next/head";
import Image from "next/image";
import { Component, createRef, ReactNode, useEffect, useState } from "react";
import { getAccountAddress, signTransaction } from "pte-browser-extension-sdk";
import useLocalStorage from "../src/localStorage";
import Modal from "../components/Modal";
import sleep from "../src/sleep";
import Navbar from "../components/Navbar";

const Home: NextPage = () => {
  // convert the above to use react state
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
  const [lockdownAddress, setLockdownAddress] = useLocalStorage(
    "lockdownAddress",
    ""
  );

  const [packageModalVisibility, setPackageModalVisibility] = useState(false);
  const [instantiateModalVisibility, setInstantiateModalVisibility] =
    useState(false);
  const [existingPoolModalVisibility, setExistingPoolModalVisibility] =
    useState(false);
  const [syncModalVisibility, setSyncModalVisibility] = useState(false);
  const XRD = "030000000000000000000000000000000000000000000000000004";

  return (
    <div>
      <div className="flex min-h-screen flex-col items-center justify-center py-2 bg-nordbg text-center text-nordtext">
        <div className="flex flex-row m-3">
          <img src="/trifold.svg" className="px-10"></img>
          <h1 className="font-extrabold text-9xl">Trifold</h1>
        </div>
        <p className="text-4xl m-3 w-1/2 ">
          A three factor DeFI lending solution based on the RADIX token and the
          scrypto library.
        </p>
        <div className="flex flex-row mt-10">
          <button
            className="py-4 px-10 font-semibold text-4xl bg-nordhighlight rounded-3xl mx-3"
            onClick={() => setPackageModalVisibility(true)}
          >
            Create new pool
          </button>
          <button
            className="py-4 px-10 font-semibold text-4xl bg-nordhighlight rounded-3xl mx-3"
            onClick={() => setExistingPoolModalVisibility(true)}
          >
            Join existing pool
          </button>
        </div>
        {packageModalVisibility && (
          <Modal
            title="Sign package"
            run={async () => {
              setAccountAddress(await getAccountAddress());

              const response = await fetch("trifold.wasm");
              const wasm = new Uint8Array(await response.arrayBuffer());

              let manifest = new ManifestBuilder()
                .publishPackage(wasm)
                .build()
                .toString();

              let receipt = await signTransaction(manifest);

              setPackageAddress(receipt.newPackages[0]);

              setPackageModalVisibility(false);
              await sleep(500);
              setInstantiateModalVisibility(true);
            }}
            cancel={() => setPackageModalVisibility(false)}
          >
            Sign and approve the manifest in the PTE browser extension
          </Modal>
        )}
        {instantiateModalVisibility && (
          <Modal
            title="Sign instantiation"
            run={async () => {
              const manifest = new ManifestBuilder()
                .callFunction(packageAddress, "Trifold", "instantiate", [])
                .callMethodWithAllResources(accountAddress, "deposit_batch")
                .build()
                .toString();

              const receipt = await signTransaction(manifest);

              setComponentAddress(receipt.newComponents[0]);

              console.log("instantiated!");
              console.log(componentAddress, resourceAddress, adminAddress);
              console.log(receipt);
              setInstantiateModalVisibility(false);
              await sleep(500);
              setSyncModalVisibility(true);
            }}
            cancel={() => setInstantiateModalVisibility(false)}
          >
            Sign and approve the manifest in the PTE browser extension
          </Modal>
        )}
        {existingPoolModalVisibility && (
          <Modal
            title="Join existing pool"
            run={() => setExistingPoolModalVisibility(false)}
            cancel={() => setExistingPoolModalVisibility(false)}
          >
            <input
              type="text"
              name="componentAddress"
              id="componentAddress"
              placeholder="Component address"
              onChange={(e) => setComponentAddress(e.target.value)}
            />
          </Modal>
        )}
        {syncModalVisibility && (
          <Modal
            title="Sync"
            run={async () => {
              const manifest = new ManifestBuilder()
                .callMethod(componentAddress, "get_info", [])
                .build()
                .toString();

              const receipt = await signTransaction(manifest);

              console.log(receipt);

              interface SyncData {
                approved_borrower_badge: string;
                karma_token: string;
                virtual_token: string;
                admin_badge: string;
              }

              const data: SyncData = JSON.parse(
                JSON.parse(receipt.outputs[0]).value
              );
              console.log(data);
              setBorrowerBadgeAddress(data.approved_borrower_badge);
              setKarmaAddress(data.karma_token);
              setResourceAddress(data.virtual_token);
              setAdminAddress(data.admin_badge);
              setSyncModalVisibility(false);
            }}
            cancel={() => setSyncModalVisibility(false)}
          >
            Sync with the smart contract
          </Modal>
        )}
      </div>
      <Navbar />
    </div>
  );
};

export default Home;
