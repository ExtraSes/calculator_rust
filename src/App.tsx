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
    <Theme accentColor="crimson" grayColor="sand" radius="large" scaling="95%">
      <main className="container">
        <h1>Welcome to Tauri + React</h1>

        <div className="row">
          <a href="https://vite.dev" target="_blank">
            <img src="/vite.svg" className="logo vite" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://react.dev" target="_blank">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
          <a href="https://www.radix-ui.com/" target="_blank">
            <img src="/radix.webp" className="logo radix" alt="Radix logo" />
          </a>
        </div>
        <p>Click on the Tauri, Vite, and React logos to learn more.</p>
		<Flex direction="column" gap="2">
			<Text>Hello from Radix Themes :</Text>
      <TextArea placeholder="KAJLKYJLATOP" />

      <div>
			<Grid columns="3" gap="4" rows="repeat(3, 64px)" width="auto">
	<Box><Button color="orange" variant="solid">
		1
	</Button></Box>
	<Box><Button color="orange" variant="solid">
		2
	</Button></Box>
	<Box><Button color="orange" variant="solid">
		3
	</Button></Box>
	<Box><Button color="orange" variant="solid">
		4
	</Button></Box>
	<Box><Button color="orange" variant="solid">
		5
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		6
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		7
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		8
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		9
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		0
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		+
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		-
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		*
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		/
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		=
	</Button></Box>
  <Box><Button color="orange" variant="solid">
		AC
	</Button></Box>
</Grid>
</div>
		</Flex>
      </main>
    </Theme>
  );
}

export default App;
