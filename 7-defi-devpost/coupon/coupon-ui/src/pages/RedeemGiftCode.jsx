import { useState } from "react";

export default function UserForm({addGiftCode}) {

  const [message, setMessage] = useState("");

  const [giftCode, setGiftCode] = useState("");

  let handleSubmit = async (e) => {
    e.preventDefault();
    try {
      let res = await fetch("http://127.0.0.1:3002/redeem/code", {
        method: "POST",
        body: JSON.stringify({ redeemcode: {
          giftcode: giftCode
        }}),
      });
      let resJson = await res.json();
      console.log(resJson);
      if (resJson.status === 200) {
        setGiftCode("");
        setMessage(`The amount of ${resJson.paymentDetails[0].amount} XRD will be sent to address ${resJson.paymentDetails[0].to_address}`);
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
          <h3>Redeem your gift card code</h3>
        </div>
        <div>
          <label>Enter your gift card code</label>
          <input
            type="text"
            name="giftCode"
            placeholder="Your gift card code"
            value={giftCode}
            onChange={(e) => setGiftCode(e.target.value)}
          />
        </div>
        <div>
          <button>Redeem</button>
        </div>
        <br></br>
        <div className="message">{message ? <p>{message}</p> : null}</div>
      </form>
    </div>
  );
}