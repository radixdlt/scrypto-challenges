import { useState } from "react";
import GiftCodeForm from "../components/GiftCodeGenerationForm.jsx";

function GenerateGiftCode() {
  // here we create an array state to store the contact form data
  const [giftCode, updateGiftCode] = useState([]);

  const addGiftCode = (code) => {
    updateGiftCode([...giftCode, code]);
  };

  return (
    <div className="GenerateGiftCode">
      <GiftCodeForm addGiftCode={addGiftCode} />
    </div>
  );
}

export default GenerateGiftCode;