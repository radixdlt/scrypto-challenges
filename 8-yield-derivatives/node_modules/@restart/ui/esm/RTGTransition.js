const _excluded = ["component"];
function _objectWithoutPropertiesLoose(source, excluded) { if (source == null) return {}; var target = {}; var sourceKeys = Object.keys(source); var key, i; for (i = 0; i < sourceKeys.length; i++) { key = sourceKeys[i]; if (excluded.indexOf(key) >= 0) continue; target[key] = source[key]; } return target; }
import * as React from 'react';
import useRTGTransitionProps from './useRTGTransitionProps';
import { jsx as _jsx } from "react/jsx-runtime";
// Normalizes Transition callbacks when nodeRef is used.
const RTGTransition = /*#__PURE__*/React.forwardRef((_ref, ref) => {
  let {
      component: Component
    } = _ref,
    props = _objectWithoutPropertiesLoose(_ref, _excluded);
  const transitionProps = useRTGTransitionProps(props);
  return /*#__PURE__*/_jsx(Component, Object.assign({
    ref: ref
  }, transitionProps));
});
export default RTGTransition;