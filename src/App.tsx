import "./App.css";

import { useState, useEffect } from "react";

import { invoke } from "@tauri-apps/api/core";

import CodeEditor from "@uiw/react-textarea-code-editor";
import { Button } from "./components/ui/button";

import MemoryGrid from "./components/memorygrid";
import StatusView from "./components/statusview";

type RegisterTuple = [number, number, number, number, number, number];

type Registers = {
  pc: number;
  s: number;
  a: number;
  x: number;
  y: number;
  p: number;
};

function App() {
  const [code, setCode] = useState("lda #$42");
  const [memory] = useState<Uint8Array>(() => new Uint8Array(0x10000));

  // Keep track of processor registers
  const [registers, setRegisters] = useState<Registers | null>(null);
  useEffect(() => {
    invoke<RegisterTuple>("get_registers")
      .then(([pc, s, a, x, y, p]) => setRegisters({ pc, s, a, x, y, p }))
      .catch((e) => console.error("Error retrieving processor registers: ", e));
  }, []);

  for (let i = 0; i < memory.length; i++) {
    memory[i] = 0xff;
  }

  return (
    <div className="bg-black text-white min-h-screen overflow-hidden">
      <div className="flex flex-row m-5 space-x-10">
        {/* Left side: Buttons and CodeEditor */}
        <div className="flex flex-col space-y-5">
          <div className="flex flex-row space-x-3">
            <TopButton>Assemble</TopButton>
            <TopButton>Run</TopButton>
            <TopButton>Step</TopButton>
            <TopButton>Reset</TopButton>
          </div>
          <div className="w-[500px]">
            <CodeEditor
              value={code}
              language="plaintext"
              placeholder="Write your assembly here."
              onChange={(evn) => setCode(evn.target.value)}
              padding={15}
              style={{
                minHeight: 300,
                backgroundColor: "white",
                color: "black",
                fontFamily:
                  "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
              }}
            />
          </div>
          <StatusView registers={registers} />
        </div>
        {/* Right size: Memory view */}
        <MemoryGrid memory={memory} />
      </div>
    </div>
  );
}

const TopButton = ({ children }: { children: React.ReactNode }) => {
  return (
    <Button className="mx-2" variant="secondary">
      {children}
    </Button>
  );
};

export default App;
