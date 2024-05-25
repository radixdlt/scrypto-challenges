import PropTypes from "prop-types";
import { RdtContext } from "../contexts.jsx";

export const RdtProvider = ({ value, children }) => (
  <RdtContext.Provider value={value}>{children}</RdtContext.Provider>
)

RdtProvider.propTypes = {
  value: PropTypes.any,
  children: PropTypes.node.isRequired,
};
