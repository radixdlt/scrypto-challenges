"use strict";

exports.__esModule = true;
exports.default = useRTGTransitionProps;
var _react = require("react");
var _useMergedRefs = _interopRequireDefault(require("@restart/hooks/useMergedRefs"));
const _excluded = ["onEnter", "onEntering", "onEntered", "onExit", "onExiting", "onExited", "addEndListener", "children"];
function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
function _objectWithoutPropertiesLoose(source, excluded) { if (source == null) return {}; var target = {}; var sourceKeys = Object.keys(source); var key, i; for (i = 0; i < sourceKeys.length; i++) { key = sourceKeys[i]; if (excluded.indexOf(key) >= 0) continue; target[key] = source[key]; } return target; }
/**
 * Normalizes RTG transition callbacks with nodeRef to better support
 * strict mode.
 *
 * @param props Transition props.
 * @returns Normalized transition props.
 */
function useRTGTransitionProps(_ref) {
  let {
      onEnter,
      onEntering,
      onEntered,
      onExit,
      onExiting,
      onExited,
      addEndListener,
      children
    } = _ref,
    props = _objectWithoutPropertiesLoose(_ref, _excluded);
  const nodeRef = (0, _react.useRef)(null);
  const mergedRef = (0, _useMergedRefs.default)(nodeRef, typeof children === 'function' ? null : children.ref);
  const normalize = callback => param => {
    if (callback && nodeRef.current) {
      callback(nodeRef.current, param);
    }
  };

  /* eslint-disable react-hooks/exhaustive-deps */
  const handleEnter = (0, _react.useCallback)(normalize(onEnter), [onEnter]);
  const handleEntering = (0, _react.useCallback)(normalize(onEntering), [onEntering]);
  const handleEntered = (0, _react.useCallback)(normalize(onEntered), [onEntered]);
  const handleExit = (0, _react.useCallback)(normalize(onExit), [onExit]);
  const handleExiting = (0, _react.useCallback)(normalize(onExiting), [onExiting]);
  const handleExited = (0, _react.useCallback)(normalize(onExited), [onExited]);
  const handleAddEndListener = (0, _react.useCallback)(normalize(addEndListener), [addEndListener]);
  /* eslint-enable react-hooks/exhaustive-deps */

  return Object.assign({}, props, {
    nodeRef
  }, onEnter && {
    onEnter: handleEnter
  }, onEntering && {
    onEntering: handleEntering
  }, onEntered && {
    onEntered: handleEntered
  }, onExit && {
    onExit: handleExit
  }, onExiting && {
    onExiting: handleExiting
  }, onExited && {
    onExited: handleExited
  }, addEndListener && {
    addEndListener: handleAddEndListener
  }, {
    children: typeof children === 'function' ? (status, innerProps) =>
    // TODO: Types for RTG missing innerProps, so need to cast.
    children(status, Object.assign({}, innerProps, {
      ref: mergedRef
    })) : /*#__PURE__*/(0, _react.cloneElement)(children, {
      ref: mergedRef
    })
  });
}