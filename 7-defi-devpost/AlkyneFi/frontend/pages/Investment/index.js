import Header from '../components/header'
import Head from 'next/head'
import SquigglyLines from '../components/SquigglyLines'
import { useState } from 'react'
export default function Home() {
  return (
    <div className="flex max-w-6xl mx-auto flex-col items-center justify-center py-2 min-h-screen">
      <Head>
        <title>AlkyneFi</title>
      </Head>
      <Header />

      <main className=" flex flex-1 w-full flex-col p-12 sm:mt-20 mt-20 background-gradient gap-16">
        <div className="flex flex-col gap-10">
          <h1 className="font-display text-5xl font-bold tracking-normal text-gray-300 ">
            Investments{' '}
            <span className="relative whitespace-nowrap text-blue-600">
              <SquigglyLines />
              <span className="relative">using AlkyneFi</span>
            </span>{' '}
          </h1>
          <div>
            <h2 className="text-2xl font-semibold text-gray-300 ">
              Ongoing Investments
            </h2>
            <div className="grid row-auto p-5">
              <Rows />
              <Rows />
            </div>
          </div>

          <div>
            <h2 className="text-2xl font-semibold text-gray-300 ">
              Past Investments
            </h2>
            <div className="grid row-auto p-5">
              <Rows />
              <Rows />
            </div>
          </div>
        </div>
      </main>
    </div>
  )
}

function Rows() {
  return (
    <div className="grid grid-cols-3 text-center px-5">
      <p className="border"> Investment 1</p>
      <p className="border"> Investment 2</p>
      <p className="border"> Investment 3</p>
    </div>
  )
}
