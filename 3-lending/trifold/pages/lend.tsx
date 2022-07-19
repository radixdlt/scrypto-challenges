import { NextPage } from "next";
import { getAccountAddress, signTransaction } from "pte-browser-extension-sdk";
import { DefaultApi, ManifestBuilder } from "pte-sdk";
import { useEffect, useState } from "react";
import Navbar from "../components/Navbar";
import useLocalStorage from "../src/localStorage";
import sleep from "../src/sleep";

const Lend: NextPage = () => {
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
  const [lockdownAddress, setLockdownAddress] = useLocalStorage(
    "lockdownAddress",
    ""
  );

  const XRD = "030000000000000000000000000000000000000000000000000004";
  const [depositAmount, setDepositAmount] = useState("");
  const [withdrawAmount, setWithdrawAmount] = useState("");

  const [personalVirtualBalance, setPersonalVirtualBalance] = useState("");
  const [personalActualBalance, setPersonalActualBalance] = useState("");
  const [poolVirtualBalance, setPoolVirtualBalance] = useState("");
  const [poolActualBalance, setPoolActualBalance] = useState("");

  // action, amount
  const recentActivityTypeGuide: [string, string, Date][] = [];
  const [recentActivity, setRecentActivity] = useLocalStorage(
    "",
    recentActivityTypeGuide
  );

  const updateBalance = async () => {
    await sleep(1000);
    const api = new DefaultApi();
    const personalAccountInfo = await api.getComponent({
      address: await getAccountAddress(),
    });
    const poolAccountInfo = await api.getComponent({
      address: componentAddress,
    });

    console.log(personalAccountInfo.ownedResources);
    console.log(resourceAddress);

    const personalVirtualInfo = personalAccountInfo.ownedResources.filter(
      (e) => e.resourceAddress === resourceAddress
    )[0];

    const personalActualInfo = personalAccountInfo.ownedResources.filter(
      (e) => e.resourceAddress === XRD
    )[0];

    const poolVirtualInfo = poolAccountInfo.ownedResources.filter(
      (e) => e.resourceAddress === resourceAddress
    )[0];

    const poolActualInfo = poolAccountInfo.ownedResources.filter(
      (e) => e.resourceAddress === XRD
    )[0];

    if (personalVirtualInfo) {
      setPersonalVirtualBalance(personalVirtualInfo.amount);
    } else {
      setPersonalVirtualBalance("0");
    }

    if (personalActualInfo) {
      setPersonalActualBalance(personalActualInfo.amount);
    } else {
      setPersonalActualBalance("0");
    }

    if (poolVirtualInfo) {
      setPoolVirtualBalance(poolVirtualInfo.amount);
    } else {
      setPoolVirtualBalance("0");
    }

    if (poolActualInfo) {
      setPoolActualBalance(poolActualInfo.amount);
    } else {
      setPoolActualBalance("0");
    }
  };

  useEffect(() => {
    if (componentAddress && resourceAddress) {
      updateBalance();
    }
  });

  const deposit = async () => {
    const manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(accountAddress, parseInt(depositAmount), XRD)
      .takeFromWorktop(XRD, "xrd")
      .callMethod(componentAddress, "deposit", ['Bucket("xrd")'])
      .callMethodWithAllResources(accountAddress, "deposit_batch")
      .build()
      .toString();
    console.log(manifest);
    const receipt = await signTransaction(manifest);
    updateBalance();

    setRecentActivity([
      ["Deposit", `You deposited ${depositAmount} XRD`, new Date()],
      ...recentActivity,
    ]);
  };

  const withdraw = async () => {
    const manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(
        accountAddress,
        parseInt(withdrawAmount),
        resourceAddress
      )
      .takeFromWorktop(resourceAddress, "lnxrd")
      .callMethod(componentAddress, "withdraw", ['Bucket("lnxrd")'])
      .callMethodWithAllResources(accountAddress, "deposit_batch")
      .build()
      .toString();
    console.log(manifest);
    const receipt = await signTransaction(manifest);
    console.log(receipt);
    updateBalance();

    setRecentActivity([
      ["Withdraw", `You withdrew ${withdrawAmount} XRD`, new Date()],
      ...recentActivity,
    ]);
  };

  const lockdown = async () => {
    const manifest = new ManifestBuilder()
      .withdrawFromAccount(accountAddress, resourceAddress)
      .takeFromWorktop(resourceAddress, "lnxrd")
      .callMethod(componentAddress, "lockdown_vote", ['Bucket("lnxrd")'])
      .callMethodWithAllResources(accountAddress, "deposit_batch")
      .build()
      .toString();
    console.log(manifest);
    const receipt = await signTransaction(manifest);
    console.log(receipt);
    updateBalance();

    setRecentActivity([
      ["Lockdown", `You locked ${withdrawAmount} XRD`, new Date()],
      ...recentActivity,
    ]);
  };

  const unlock = async () => {
    const manifest = new ManifestBuilder()
      .withdrawFromAccount(accountAddress, lockdownAddress)
      .takeFromWorktop(lockdownAddress, "locked")
      .callMethod(componentAddress, "withdraw_lockdown", ['Bucket("locked")'])
      .callMethodWithAllResources(accountAddress, "deposit_batch")
      .build()
      .toString();
    console.log(manifest);
    const receipt = await signTransaction(manifest);
    console.log(receipt);
    updateBalance();

    setRecentActivity([
      ["Unlock", `You unlocked ${withdrawAmount} XRD`, new Date()],
      ...recentActivity,
    ]);
  };

  return (
    <div>
      <div
        className="grid grid-cols-2 grid-rows-3 gap-10 p-20 min-h-screen place-items-stretch justify-center bg-nordbg2 text-nordtext"
        id="section2"
      >
        <div className="flex flex-col bg-nordbg rounded-3xl p-10">
          <h1 className="font-extrabold text-4xl mb-5">Current Balances</h1>
          <p className="text-2xl mb-5 font-semibold">
            Personal:
            <br />
            {personalVirtualBalance} lnXRD
            <br />
            {personalActualBalance} XRD
            <br />
            <br />
            <br />
            Pool:
            <br />
            {poolVirtualBalance} lnXRD
            <br />
            {poolActualBalance} XRD
          </p>
        </div>
        <div className="flex flex-col bg-nordbg rounded-3xl p-10 row-span-3">
          <h1 className="font-extrabold text-4xl mb-5">Recent Activity</h1>
          {recentActivity.map(([action, amount, date]) => (
            <div className="bg-nordbg2 rounded-lg p-5 my-5">
              <h3 className="text-2xl mb-5 font-semibold">
                {action} at {date.toString()}
              </h3>
              <p className="text-xl font-light">{amount}</p>
            </div>
          ))}
        </div>
        <div className="flex flex-col bg-nordbg rounded-3xl p-10">
          <h1 className="font-extrabold text-4xl mb-5">Deposit</h1>
          <input
            type="number"
            name="deposit"
            id="deposit"
            className="bg-gray-600"
            onChange={(e) => setDepositAmount(e.target.value)}
          />
          <button
            className="py-2 px-10 mt-10 font-semibold text-4xl bg-nordhighlight rounded-3xl"
            onClick={deposit}
          >
            Deposit
          </button>
          <h1 className="font-extrabold text-4xl my-5">Withdraw</h1>
          <input
            type="number"
            name="withdraw"
            id="withdraw"
            className="bg-gray-600"
            onChange={(e) => setWithdrawAmount(e.target.value)}
          />
          <button
            className="py-2 px-10 mt-10 font-semibold text-4xl bg-nordhighlight rounded-3xl"
            onClick={withdraw}
          >
            Withdraw
          </button>
        </div>
        <div className="flex flex-col bg-nordbg rounded-3xl p-10">
          <h1 className="font-extrabold text-4xl mb-5">Lockdown</h1>
          <p>For emergency use only</p>
          <button
            className="py-2 px-10 mt-10 font-semibold text-4xl bg-nordhighlight rounded-3xl"
            onClick={lockdown}
          >
            Lock
          </button>
          <button
            className="py-2 px-10 mt-10 font-semibold text-4xl bg-nordhighlight rounded-3xl"
            onClick={unlock}
          >
            Unlock
          </button>
        </div>
      </div>
      <Navbar />
    </div>
  );
};

export default Lend;
