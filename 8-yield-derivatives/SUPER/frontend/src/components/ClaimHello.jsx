import { useSendTransaction } from "../hooks/useSendTransaction";
import PropTypes from "prop-types";

ClaimHello.propTypes = {
  selectedAccount: PropTypes.string,
  enableButtons: PropTypes.bool,
};

function ClaimHello(props) {
  const { selectedAccount, enableButtons } = props;

  const sendTransaction = useSendTransaction();

  const handleClaimToken = async () => {
    if (!selectedAccount) {
      alert("Please select an account first.");
      return;
    }

    const componentAddress =
      "component_tdx_2_1crmw9yqwfaz9634qf3tw9s89zxnk8fxva958vg8mxxeuv9j6eqer2s";
    const accountAddress = selectedAccount;

    let manifest = `
      CALL_METHOD
        Address("${componentAddress}")
        "free_token"
        ;
      CALL_METHOD
        Address("${accountAddress}")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP")
        ;
    `;

    const { receipt } = await sendTransaction(manifest);
    console.log("transaction receipt:", receipt);
  };

  return (
    <button
      id="get-hello-token"
      onClick={handleClaimToken}
      disabled={!selectedAccount || !enableButtons}>
      Claim Hello Token
    </button>
  );
}

export default ClaimHello;
