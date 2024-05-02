// Import the variable from your JavaScript file
import { getXrdAddress } from './gateway.ts';

// Get a reference to the select element
const currencySelect = document.getElementById('currencySelect');

currencySelect.addEventListener('change', function() {
    // Call the function to get the updated xrdAddress based on the selected value
    let newTokenAddress = getXrdAddress(currencySelect.value);
    console.log(`current token address ${newTokenAddress} `);
});
