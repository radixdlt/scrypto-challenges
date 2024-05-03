import * as React from 'react';
import { TransitionProps } from './useRTGTransitionProps';
export type RTGTransitionProps = TransitionProps & {
    component: React.ElementType;
};
declare const RTGTransition: React.ForwardRefExoticComponent<(Pick<import("react-transition-group/Transition").TimeoutProps<undefined> & {
    children: React.ReactElement<any, string | React.JSXElementConstructor<any>> | ((status: import("react-transition-group").TransitionStatus, props: Record<string, unknown>) => React.ReactNode);
} & {
    component: React.ElementType;
}, keyof import("react-transition-group/Transition").TimeoutProps<undefined>> | Pick<import("react-transition-group/Transition").EndListenerProps<undefined> & {
    children: React.ReactElement<any, string | React.JSXElementConstructor<any>> | ((status: import("react-transition-group").TransitionStatus, props: Record<string, unknown>) => React.ReactNode);
} & {
    component: React.ElementType;
}, keyof import("react-transition-group/Transition").EndListenerProps<undefined>>) & React.RefAttributes<any>>;
export default RTGTransition;
