


import React, { useState, useEffect } from 'react';
import { OpenAI } from 'openai';

const TradingBotComponent = () => {
  const [messages, setMessages] = useState([]);
  const [inputText, setInputText] = useState('');
  const [chatGPT, setChatGPT] = useState(null); // State to hold the initialized OpenAI client

  useEffect(() => {
    const initializeOpenAI = async () => {
      try {
        const chatGPTClient = new OpenAI({ 
          dangerouslyAllowBrowser: true 
        });
        // Save the initialized client to state
        setChatGPT(chatGPTClient);
      } catch (error) {
        console.error('Error initializing OpenAI:', error);
      }
    };

    initializeOpenAI();
  }, []);

  const sendMessage = async () => {
    try {
      if (!chatGPT || !chatGPT.completions || !chatGPT.completions.create) {
        console.error('OpenAI client or create method not available.');
        return;
      }

      const response = await chatGPT.completions.create({
        model: 'text-gpt-3.5-turbo-0125', // Use the GPT-3.5 Turbo model
        prompt: inputText,
        max_tokens: 150 // Adjust as needed
      });

      const newMessage = response.data.choices[0].text.trim();
      const tradingSignal = await getTradingSignal(); // Call your own AI logic
      setMessages([...messages, { text: inputText }, { text: newMessage }, { text: `Trading Signal: ${tradingSignal}` }]);
      setInputText('');
    } catch (error) {
      console.error('Error sending message:', error);
    }
  };

  console.log('OpenAI client:', chatGPT);

  const getTradingSignal = async () => {
    // Implement your trading signal logic here
    // This function should return a trading signal (e.g., 'Buy', 'Sell', or 'Hold')
    // For demonstration purposes, let's return 'Buy' as an example
    return 'Buy';
  };

  useEffect(() => {
    if (messages.length === 0) {
      integrateTradingSignal().then(signal => {
        const newMessages = [...messages, { text: `Final trading signal: ${signal}` }];
        setMessages(newMessages);
      });
    }
  }, [messages]); // Update when messages change

  const integrateTradingSignal = async () => {
    // Get market data
    const marketData = await getMockMarketData();

    // Calculate average price from market data
    const averagePrice = calculateAveragePrice(marketData);

    // Get trading signal based on real-time cryptocurrency prices
    const realTimeSignal = await getTradingSignal();

    // Generate trading signal based on SMA
    const latestPrice = marketData[marketData.length - 1].price;
    const smaSignal = latestPrice > averagePrice ? 'Buy' : 'Sell';

    // Decide on final trading signal based on a combination of signals
    // For demonstration purposes, let's prioritize real-time signal over SMA signal
    const finalSignal = realTimeSignal === 'Hold' ? smaSignal : realTimeSignal;

    return finalSignal;
  };

  const getMockMarketData = async () => {
    // Mock market data (replace this with real market data)
    return [
      { timestamp: '2024-04-20T12:00:00', price: 100 },
      { timestamp: '2024-04-20T12:15:00', price: 110 },
      { timestamp: '2024-04-20T12:30:00', price: 105 },
      { timestamp: '2024-04-20T12:45:00', price: 115 },
      // Add more data points as needed
    ];
  };

  const calculateAveragePrice = (prices) => {
    // Simple moving average (SMA) calculation
    const windowSize = 3; // Adjust window size as needed
    const lastPrices = prices.slice(-windowSize).map(data => data.price);
    const averagePrice = lastPrices.reduce((sum, price) => sum + price, 0) / windowSize;
    return averagePrice;
  };

  return (
    <div>
      <div>
        {messages.map((message, index) => (
          <div key={index}>{message.text}</div>
        ))}
      </div>
      <input value={inputText} onChange={(e) => setInputText(e.target.value)} />
      <button onClick={sendMessage}>Send</button>
    </div>
  );
};

export default TradingBotComponent;
