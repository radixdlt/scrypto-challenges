import './App.css';
import TradingBotComponent from './openAI';
import TradingBotCompo from './ins'
import Header from './Header';
import Footer from './Footer';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Header/>
        </header>
        <div><TradingBotCompo/></div>
     <Footer>
      <Footer/>
     </Footer>
    </div>
  );
}

export default App;
