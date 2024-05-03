
const express = require('express');
const cors = require('cors');
const app = express();
const port = process.env.PORT || 5001;


// Apply CORS middleware
app.use(cors({
  origin: 'http://localhost:3000'
}));

app.use(express.json());

//const ticketContract = new web3.eth.Contract(TicketABI, TicketAddress);



  

// Calculate coins based on input amount
// Calculate coins based on input amount with validation
app.post('/calculate-coins', (req, res) => {
  try {
    const { amount } = req.body;

    // Validate amount
    if (isNaN(amount)) {
      throw new Error('Invalid input: amount must be a number');
    }

    // Custom validation logic - Check if the amount is even
    if (!isValidAmount(amount)) {
      throw new Error('Invalid input: amount is not even');
    }

    // Calculate coins based on the input amount only if it's valid
    const coins = calculateCoins(amount);

    res.json({ coins });
  } catch (error) {
    console.error('Error calculating coins:', error.message);
    res.status(400).json({ error: error.message });
  }
});

// Function to check if the amount is even
function isValidAmount(amount) {
  return amount % 2 === 0;
}

// Function to calculate coins
function calculateCoins(amount) {
  return amount / 50; // Example calculation
}



// Mock user database (replace this with your actual user database)
const users = [
  { email: 'alex@gmail.com', password: '123456', name: 'User 1' },
  { email: 'user2@example.com', password: 'password2', name: 'User 2' }
];

// Endpoint for user login
app.post('/api/login', (req, res) => {
  const { email, password } = req.body;

  // Find the user in the mock database
  const user = users.find(u => u.email === email && u.password === password);

  if (user) {
    // Login successful
    res.status(200).json({ message: 'Login successful', user });
  } else {
    // Login failed
    res.status(401).json({ message: 'Invalid email or password' });
  }
});

// Start the server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});


