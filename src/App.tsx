import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Flex, Text, Button, Theme, Grid, Box, TextArea } from "@radix-ui/themes";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  
  return (
    <Theme 
      accentColor="crimson" 
      grayColor="slate" 
      radius="large" 
      scaling="95%"
      appearance="dark"
      hasBackground={false}
    >
      <main className="container">
        <Flex direction="column" gap="6" align="center">
          <Text size="8" weight="bold" style={{ color: 'white', textShadow: '0 2px 4px rgba(0,0,0,0.3)' }}>
            Welcome to Tauri + React
          </Text>

          <Flex gap="4" wrap="wrap" justify="center">
            <Box 
              style={{ 
                background: 'rgba(0, 0, 0, 0.4)',
                backdropFilter: 'blur(20px)',
                borderRadius: '20px',
                padding: '1.5rem',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5)',
                transition: 'all 0.3s ease'
              }}
            >
              <a href="https://vite.dev" target="_blank">
                <img src="/vite.svg" className="logo vite" alt="Vite logo" />
              </a>
            </Box>
            
            <Box 
              style={{ 
                background: 'rgba(0, 0, 0, 0.4)',
                backdropFilter: 'blur(20px)',
                borderRadius: '20px',
                padding: '1.5rem',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5)',
                transition: 'all 0.3s ease'
              }}
            >
              <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
              </a>
            </Box>
            
            <Box 
              style={{ 
                background: 'rgba(0, 0, 0, 0.4)',
                backdropFilter: 'blur(20px)',
                borderRadius: '20px',
                padding: '1.5rem',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5)',
                transition: 'all 0.3s ease'
              }}
            >
              <a href="https://react.dev" target="_blank">
                <img src={reactLogo} className="logo react" alt="React logo" />
              </a>
            </Box>
            
            <Box 
              style={{ 
                background: 'rgba(0, 0, 0, 0.4)',
                backdropFilter: 'blur(20px)',
                borderRadius: '20px',
                padding: '1.5rem',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5)',
                transition: 'all 0.3s ease'
              }}
            >
              <a href="https://www.radix-ui.com/" target="_blank">
                <img src="/radix.png" className="logo radix" alt="Radix logo" />
              </a>
            </Box>
          </Flex>

          <Text size="4" style={{ color: 'rgba(255, 255, 255, 0.9)', textAlign: 'center' }}>
            Калькулятор Демид Эдишен
          </Text>

          <div className="calculator-container">
            <div className="calculator-display">
              0
            </div>
            
            <div className="calculator-buttons">
              <button className="calculator-button">C</button>
              <button className="calculator-button">±</button>
              <button className="calculator-button">%</button>
              <button className="calculator-button operator">÷</button>
              
              <button className="calculator-button">7</button>
              <button className="calculator-button">8</button>
              <button className="calculator-button">9</button>
              <button className="calculator-button operator">×</button>
              
              <button className="calculator-button">4</button>
              <button className="calculator-button">5</button>
              <button className="calculator-button">6</button>
              <button className="calculator-button operator">-</button>
              
              <button className="calculator-button">1</button>
              <button className="calculator-button">2</button>
              <button className="calculator-button">3</button>
              <button className="calculator-button operator">+</button>
              
              <button className="calculator-button" style={{ gridColumn: 'span 2' }}>0</button>
              <button className="calculator-button">.</button>
              <button className="calculator-button equals">=</button>
            </div>
          </div>
        </Flex>
      </main>
    </Theme>
  );
}

export default App;
