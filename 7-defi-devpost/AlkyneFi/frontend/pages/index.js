import Header from './components/header';
import Head from 'next/head';
import SquigglyLines from './components/SquigglyLines';
import Router from 'next/router';
export default function Home() {
  return (
    <div className="flex max-w-6xl mx-auto flex-col items-center justify-center py-2 min-h-screen">
      <Head>
        <title>AlkyneFi</title>
      </Head>

      <Header />
      <main className=" flex flex-1 w-full flex-col items-center justify-center text-center px-4 sm:mt-20 mt-20 background-gradient">
        <h1 className="mx-auto max-w-4xl font-display text-5xl font-bold tracking-normal text-gray-300 sm:text-7xl">
          AlkyneFi is trader's go-to platform{' '}
          <span className="relative whitespace-nowrap text-blue-600">
            {/* <SquigglyLines /> */}
            <span className="relative">to skyrocket their investment</span>
          </span>{' '}
          by increasing the Principal involved
        </h1>
        <h2 className="mx-auto mt-12 max-w-xl text-lg sm:text-gray-400  text-gray-500 leading-7">
          Traders risk multiple assets and opportunities to cater themselves with the profits that they try to earn and
          it's only fundamentally apt to ensure that they get the best out of their investment by allowing them to
          maximise the variables that support it. We took compound interest as our bible and understood it piece by
          piece where in we first maximise profits by doubling the Principal, and suggests trader to stick longer for
          even more returns.
        </h2>
      </main>
    </div>
  );
}
