// Import the variable from your JavaScript file
import { getTokenAddress } from './gateway.ts';

// Get a reference to the select element
const currencySelect = document.getElementById('currencySelect');

currencySelect.addEventListener('change', function() {
    // Call the function to get the updated tokenAddress based on the selected value
    let newTokenAddress = getTokenAddress(currencySelect.value);
    console.log(`current token address ${newTokenAddress} `);
});
