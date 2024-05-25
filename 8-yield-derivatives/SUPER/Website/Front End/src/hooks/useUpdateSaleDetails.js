import { useContext, useEffect } from 'react';
import { UpdateTriggerContext } from '../context/contexts.jsx';

// Custom hook for updating sale details
export const useUpdateSaleDetails = () => {
    const {  update } = useContext(UpdateTriggerContext);

    useEffect(() => {
        update();  // Trigger an update on component mount

    }, []);
};