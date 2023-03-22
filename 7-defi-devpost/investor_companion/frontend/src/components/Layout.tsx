import React from "react";
import { DashboardIcon, IntegrationIcon, PreferencesIcon } from "./Icons";
import { FaUserCircle, FaSearch } from "react-icons/fa";
import Input from "./Input";

interface Props {
  children: React.ReactNode;
}

function Layout(props: Props) {
  const {} = props;

  return (
    <div className="flex text-inherit">
      <div className="h-screen w-1/5 flex flex-col gap-2 p-2  items-center bg-gray-800 shadow-md shadow-gray-700">
        <p className="font-bold text-md">🦄 DeCrypt </p>
        <hr className="bg-primary-1 " />
        <a href="/dashboard">
          <DashboardIcon />
        </a>
        <a href="/savings">
          <PreferencesIcon />
        </a>
        <a href="/integration">
          <IntegrationIcon />
        </a>
      </div>
      <main className="ml-2 w-4/5">
        <div className=" bg-gray-800 p-3 w-full">
          <ul className="flex justify-between gap-1 items-center ">
            <li>
              <Input
                onChange={(e) => console.log("Clicked", e.target.value)}
                type="text"
                placeholder="Search"
              />
            </li>
            <li className="flex items-center gap-1 self-end">
              {" "}
              <FaUserCircle /> Profile
            </li>
          </ul>
        </div>
        {props.children}
      </main>
    </div>
  );
}

export default Layout;
