import { useState } from "react";

export default function UserForm({addGiftCode}) {

  const [message, setMessage] = useState("");

  const [id, setID] = useState("");
  const [to_address, setAddress] = useState("");
  const [amount, setAmount] = useState("");

  let handleSubmit = async (e) => {
    e.preventDefault();
    try {
      let res = await fetch("http://127.0.0.1:3002/giftcode", {
        method: "POST",
        body: JSON.stringify({ giftcode: {
          id: id,
          to_address: to_address,
          amount: amount,
        }}),
      });
      let resJson = await res.json();
      console.log(resJson);
      if (resJson.status === 200) {
        setID("");
        setAddress("");
        setAmount("");
        setMessage(`${resJson.giftcode}`);
      } else {
        setMessage("Some error occured");
      }
    } catch (err) {
      console.log(err);
    }
  };
  return (
    <div className="form-container">
      <form onSubmit={handleSubmit}> 
        <div className="form-input">
          <h3>Generate your gift card code</h3>
        </div>
        <div>
          <label>Enter your wallet id</label>
          <input
            type="text"
            name="id"
            placeholder="Your wallet id"
            value={id}
            onChange={(e) => setID(e.target.value)}
          />
        </div>
        <div>
        <label>Enter sender's wallet id</label>
          <input
            type="text"
            name="to_address"
            placeholder="Receiver Address"
            value={to_address}
            onChange={(e) => setAddress(e.target.value)}
          />
        </div>
        <div>
        <label>Enter the amount</label>
          <input
            type="text"
            name="amount"
            placeholder="Amount"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
          />
        </div>
        <div>
          <button>Generate GiftCode</button>
        </div>
        <br></br>
        <label>Copy over the git card code below</label>
        <div className="message">{message ? <p>{message}</p> : null}</div>
      </form>
    </div>
  );
}