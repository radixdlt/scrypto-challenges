import React, { createContext, useContext, useState } from 'react';
import {UpdateTriggerContext} from './contexts.jsx';


// Provider component
export const UpdateTriggerProvider = ({ children }) => {
    const [trigger, setTrigger] = useState(0); // initial value 0

    const update = () => {
        setTrigger(prev => prev + 1); // increment to trigger update
    };

    return (
        <UpdateTriggerContext.Provider value={{ trigger, update }}>
            {children}
        </UpdateTriggerContext.Provider>
    );
};