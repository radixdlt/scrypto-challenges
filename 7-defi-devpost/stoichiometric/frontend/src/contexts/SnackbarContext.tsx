import React, { useState, useRef } from "react";

const SnackbarContext = React.createContext(null as any);

interface Props {
    children: any;
}

const SnackbarCtx: React.FC<Props> = (props) => {

    const [alerts, setAlerts] = useState<any[]>([])

    const alertsRef = useRef(alerts)
    alertsRef.current = alerts

    function popAlert(array: any[]) {
        setAlerts(array.filter((elm, idx) => idx !== array.length - 1));
    }

    const addAlert = (type: string, message: string) => {
        setAlerts([{ type, message }, ...alertsRef.current])
        setTimeout(() => popAlert(alertsRef.current), 5000)
    };

    return (
        <SnackbarContext.Provider value={{ alerts, addAlert }}>
            {props.children}
        </SnackbarContext.Provider>
    )

};

export { SnackbarContext, SnackbarCtx };