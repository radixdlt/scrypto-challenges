import Image from 'next/image';
import Link from 'next/link';
import { useEffect, useState } from 'react';
import { RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';

export default function Header({ photo }) {
  const [rdtState, setRdtState] = useState();

  // useEffect(() => {
  //   const rdt = RadixDappToolkit(
  //     {
  //       dAppDefinitionAddress: 'account_tdx_b_1ppglnkmukh36l2dfw3uygvgjf2jsfypl885u9840md7swrvpmj',
  //       dAppName: 'AlkyneFi',
  //     },
  //     (requestData) => {
  //       requestData({
  //         accounts: { quantifier: 'atLeast', quantity: 1 },
  //       }).map(({ data: { accounts } }) => {
  //         // set your application state
  //       });
  //     },
  //     {
  //       networkId: 11,
  //       onDisconnect: () => {
  //         // clear your application state
  //       },
  //       onInit: ({ accounts }) => {
  //         // set your initial application state
  //       },
  //     }
  //   );
  //   console.log("Rdt: ", rdt.state$.subscribe());
  //   // const subscription = rdt?.state$.subscribe((state) => {
  //   //   setState(state);
  //   // });
  // }, []);

  return (
    <header className="flex flex-row xs:flex-row justify-between items-center w-full mt-3 border-b pb-7 sm:px-4 px-2 border-gray-500 gap-2">
      <Link href="/Home" className="flex space-x-2">
        <Image alt="header text" src="/bed.svg" className="sm:w-10 sm:h-10 w-9 h-9" width={24} height={24} />
        <h1 className="sm:text-3xl text-xl font-bold ml-2 tracking-tight">AlkyneFi</h1>
      </Link>
      {/* <radix-connect-button /> */}
      <div className="flex max-w-fit items-center justify-center space-x-2 text-white px-5 py-2 text-sm shadow-md font-medium transition">
      </div>
    </header>
  );
}
