import React, { useState, useEffect } from "react";

import useWindowSize from "utils/useWindowSize";

const ResponsiveContext = React.createContext(null as any);

interface Props {
    children: any;
}

const ResponsiveCtx: React.FC<Props> = (props) => {

    const windowSize = useWindowSize();

    const [device, setDevice] = useState("desktop");

    useEffect(() => {
        if (windowSize.width == undefined) return
        if (windowSize.width <= 768) { setDevice("mobile"); return; }
        if (windowSize.width <= 1024) { setDevice("tablet"); return; }
        if (windowSize.width <= 1200) { setDevice("laptop"); return }
        setDevice("desktop");
    }, [windowSize]);

    return (
        <ResponsiveContext.Provider value={{ windowSize, device }}>
            {props.children}
        </ResponsiveContext.Provider>
    )

};

export { ResponsiveContext, ResponsiveCtx };