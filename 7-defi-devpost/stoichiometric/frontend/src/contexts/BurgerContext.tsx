import React, { useEffect, useState } from "react";

const BurgerContext = React.createContext(null as any);

interface Props {
    children: any;
}

const BurgerCtx: React.FC<Props> = (props) => {

    const [burgerOpen, setBurgerOpen] = useState(false);

    function toggleBurger() {
        setBurgerOpen(!burgerOpen);
    }

    return (
        <BurgerContext.Provider value={{ burgerOpen, toggleBurger }}>
            {props.children}
        </BurgerContext.Provider>
    )

};

export { BurgerContext, BurgerCtx };