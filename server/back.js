const express = require('express');
const axios = require('axios');
const { OpenAI } = require('openai');

const app = express();
const port = 3000;


// GET endpoint for chat completions
app.get('/api/chat-completion', async (req, res) => {
  try {
    const prompt = req.query.prompt;
    const completion = await generateChatCompletion(prompt);
    res.json({ completion });
  } catch (error) {
    console.error('Error generating chat completion:', error);
    res.status(500).json({ message: 'Internal server error' });
  }
});

// POST endpoint for chat completions
app.post('/api/chat-completion', async (req, res) => {
  try {
    const prompt = req.body.prompt;
    const completion = await generateChatCompletion(prompt);
    res.json({ completion });
  } catch (error) {
    console.error('Error generating chat completion:', error);
    res.status(500).json({ message: 'Internal server error' });
  }
});



const generateChatCompletion = async (prompt) => {
    try {
      const response = await openai.completions.create({
        engine: 'text-gpt-3.5-turbo', // Replace with a non-deprecated model
        prompt: prompt,
        max_tokens: 150 // Adjust as needed
      });
      return response.data.choices[0].text.trim();
    } catch (error) {
      console.error('Error generating chat completion:', error);
      throw error;
    }
  };
  
  
  
  

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
