"use strict";

exports.__esModule = true;
exports.default = void 0;
var React = _interopRequireWildcard(require("react"));
var _useRTGTransitionProps = _interopRequireDefault(require("./useRTGTransitionProps"));
var _jsxRuntime = require("react/jsx-runtime");
const _excluded = ["component"];
function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
function _getRequireWildcardCache(nodeInterop) { if (typeof WeakMap !== "function") return null; var cacheBabelInterop = new WeakMap(); var cacheNodeInterop = new WeakMap(); return (_getRequireWildcardCache = function (nodeInterop) { return nodeInterop ? cacheNodeInterop : cacheBabelInterop; })(nodeInterop); }
function _interopRequireWildcard(obj, nodeInterop) { if (!nodeInterop && obj && obj.__esModule) { return obj; } if (obj === null || typeof obj !== "object" && typeof obj !== "function") { return { default: obj }; } var cache = _getRequireWildcardCache(nodeInterop); if (cache && cache.has(obj)) { return cache.get(obj); } var newObj = {}; var hasPropertyDescriptor = Object.defineProperty && Object.getOwnPropertyDescriptor; for (var key in obj) { if (key !== "default" && Object.prototype.hasOwnProperty.call(obj, key)) { var desc = hasPropertyDescriptor ? Object.getOwnPropertyDescriptor(obj, key) : null; if (desc && (desc.get || desc.set)) { Object.defineProperty(newObj, key, desc); } else { newObj[key] = obj[key]; } } } newObj.default = obj; if (cache) { cache.set(obj, newObj); } return newObj; }
function _objectWithoutPropertiesLoose(source, excluded) { if (source == null) return {}; var target = {}; var sourceKeys = Object.keys(source); var key, i; for (i = 0; i < sourceKeys.length; i++) { key = sourceKeys[i]; if (excluded.indexOf(key) >= 0) continue; target[key] = source[key]; } return target; }
// Normalizes Transition callbacks when nodeRef is used.
const RTGTransition = /*#__PURE__*/React.forwardRef((_ref, ref) => {
  let {
      component: Component
    } = _ref,
    props = _objectWithoutPropertiesLoose(_ref, _excluded);
  const transitionProps = (0, _useRTGTransitionProps.default)(props);
  return /*#__PURE__*/(0, _jsxRuntime.jsx)(Component, Object.assign({
    ref: ref
  }, transitionProps));
});
var _default = RTGTransition;
exports.default = _default;