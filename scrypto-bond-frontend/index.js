
//import Layout from "../components/Layout";
import Header from "../components/Header";
import Hero from "../components/Hero";
//import IS from "../components/IS";
//import B from "../components/B";
import BasicTable from "../components/BasicTable";
import classes from '../styles/Home.module.scss';

import ReactDOM from "react-dom";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "../pages/Layout";
import IS from "../pages/IS";
import B from "../pages/B";
//import Contact from "./pages/Contact";
//import NoPage from "./pages/NoPage";


/*
const Index = () => {
  return (
    <Layout pageTitle="Landing Page Nextjs">
      <Header />
      <Hero />
      <IS />
      <B />
      <BasicTable />
    </Layout>
  )
}
export default Index;
*/


/*
export default function Home() {
  return (
    <div className={classes.container}>
      <h1>Header</h1>
      <Hero />
      <IS />
      <B />
      <BasicTable />
    </div>
  );
}
*/


/*
import ReactDOM from "react-dom";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from "./pages/Layout";
import Home from "./pages/Home";
import Blogs from "./pages/Blogs";
import Contact from "./pages/Contact";
import NoPage from "./pages/NoPage";
*/

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="blogs" element={<Blogs />} />
          <Route path="contact" element={<Contact />} />
          <Route path="*" element={<NoPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
COPY



