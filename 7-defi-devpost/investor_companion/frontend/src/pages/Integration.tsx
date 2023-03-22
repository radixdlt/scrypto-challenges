import React from "react";
import { FaArrowAltCircleUp, FaICursor, FaMercury } from "react-icons/fa";
import Layout from "../components/Layout";

interface Props {}

function ProtocolsCard() {
  return (
    <div className="p-6 shadow-md bg-gray-800 w-60 h-60 flex flex-col items-center justify-center gap-2">
      <img
        className="w-20 h-20 rounded-full"
        alt="maker-dao"
        src="data:image/jpeg;base64,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"
      />
      <h1>MakerDAO</h1>
      <small className="text-gray-500 text-center">
        MakerDAO is a decentralized autonomous organization that manages the Dai
      </small>
      <a className="text-sm flex items-center gap-1 text-primary-1" href="">Learn more <FaArrowAltCircleUp/></a>
    </div>
  );
}
function Integration(props: Props) {
  const {} = props;
  const list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

  return (
    <Layout>
      <main>
        <h1 className="text-gray-400 my-2">
          <strong>Popular Protocols</strong>
        </h1>
        <div className="flex flex-wrap gap-2">
          {list.map((item, index) => (
            <div className="" key={index}>
              <ProtocolsCard />
            </div>
          ))}
        </div>
      </main>
    </Layout>
  );
}

export default Integration;
