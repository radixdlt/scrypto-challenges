
//import Layout from "../components/Layout";
import Header from "./Header";
import Hero from "../components/Hero";
//import IS from "../components/IS";
//import B from "../components/B";
import BasicTable from "../components/BasicTable";
import classes from '../styles/Home.module.scss';

import ReactDOM from "react-dom";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "../pages/Layout";
import Issue from "./Issue";
import Sell from "./Sell";
import Buy from "./Buy";



import Link from "next/link"
import styles from '../styles/Home.module.css'

export default function Home() {
  return (
   <div className="relative z-1">
      <div className="dark">
    
    <div className={styles.container}>
    <Header />

      <main className={styles.main}>
        

        <div className={styles.grid}>
        <div className="mt-5 mt-lg-0">
            </div>
        
          <Link href="/Issue" >
          <a className={styles.card}>
            <h2>Issue &rarr;</h2>
            </a>
          </Link>

          <Link href="/Sell" >
          <a className={styles.card}>
            <h2>Sell &rarr;</h2>
            </a>
          </Link>

          <Link href="/Buy" >
            <a className={styles.card}>
            <h2>Buy &rarr;</h2>
            </a>
          </Link>

        </div>
      </main>
    </div>
  </div>
</div>
  
  )
}

