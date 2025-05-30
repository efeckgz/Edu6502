import "./App.css";

import { useState } from "react";

import { invoke, Channel } from "@tauri-apps/api/core";

import CodeEditor from "@uiw/react-textarea-code-editor";
import { Button } from "./components/ui/button";

import MemoryGrid from "./components/memorygrid";
import StateView from "./components/stateview";

import { InternalState } from "./components/stateview.tsx";

// type RegisterTuple = [number, number, number, number, number, number];

// type Registers = {
//   pc: number;
//   s: number;
//   a: number;
//   x: number;
//   y: number;
//   p: number;
// };

function App() {
  const defaultState: InternalState = {
    pc: 0,
    s: 255,
    a: 0,
    x: 0,
    y: 0,
    p: 0,

    addr: 0,
    data: 0,
    rw: true,
  };

  const [code, setCode] = useState("lda #$42");
  const [memory] = useState<Uint8Array>(() => new Uint8Array(0x10000));

  // Keep track of processor registers
  const [internalState, setInternalState] =
    useState<InternalState>(defaultState);

  for (let i = 0; i < memory.length; i++) {
    memory[i] = 0xff;
  }

  // This is the Tauri channel approach
  const onEvent = new Channel<InternalState>();
  onEvent.onmessage = (m) => setInternalState(m);

  const runAsm = async () => {
    await invoke("run_asm", { onEvent });
  };

  return (
    <div className="bg-black text-white min-h-screen overflow-hidden">
      <div className="flex flex-row m-5 space-x-10">
        {/* Left side: Buttons and CodeEditor */}
        <div className="flex flex-col space-y-5">
          <div className="flex flex-row space-x-3">
            <TopButton>Assemble</TopButton>
            <TopButton onClick={() => runAsm()}>Run</TopButton>
            <TopButton onClick={() => invoke("stop")}>Stop</TopButton>
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
          <StateView state={internalState} />
        </div>
        {/* Right size: Memory view */}
        <MemoryGrid memory={memory} />
      </div>
    </div>
  );
}

const TopButton = ({
  children,
  onClick,
}: {
  children: React.ReactNode;
  onClick?: () => void;
}) => {
  return (
    <Button className="mx-2" variant="secondary" onClick={onClick}>
      {children}
    </Button>
  );
};

export default App;
