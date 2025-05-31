import "./App.css";

import { useState, useEffect } from "react";

import { invoke, Channel } from "@tauri-apps/api/core";

import CodeEditor from "@uiw/react-textarea-code-editor";
import { Button } from "./components/ui/button";

import MemoryGrid from "./components/memorygrid";
import StateView from "./components/stateview";

import { InternalState } from "./components/stateview.tsx";

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
  const [memory, setMemory] = useState<Uint8Array>(
    () => new Uint8Array(0x10000),
  );

  // Text to show on run button.
  // Default is Run. When stopped show Continue.
  const [runBtnText, setRunBtnText] = useState("Run");

  // Keep track of processor registers
  const [internalState, setInternalState] =
    useState<InternalState>(defaultState);

  // memory[0x0000] = 100;
  // memory[0x0001] = 0xa9;
  // memory[0x0100] = 0x42;

  // invoke("get_nonzero_bytes").then((r: any) => {
  //   for (let i = 0; i < r.length; i++) {
  //     let [addr, byte] = r[i];
  //     console.log("Addr, Byte: ", addr, byte);
  //     memory[addr] = byte;
  //   }
  // });

  useEffect(() => {
    invoke("get_nonzero_bytes").then((r: any) => {
      // Create a copy of memory
      const newMem = new Uint8Array(0x10000);

      // Set the non-zero bytes
      for (let i = 0; i < r.length; i++) {
        const [addr, byte] = r[i];
        newMem[addr] = byte;
      }

      // Update React state
      setMemory(newMem);
    });
  }, []);

  // for (let i = 0; i < memory.length; i++) {
  //   memory[i] = 0x00;
  // }

  // This is the Tauri channel approach
  const onEvent = new Channel<InternalState>();
  onEvent.onmessage = (m) => setInternalState(m);

  const runAsm = async () => {
    setRunBtnText("Running...");
    await invoke("run_asm", { onEvent });
  };

  const stop = async () => {
    setRunBtnText("Continue");
    invoke("stop");
  };

  const reset = async () => {
    setRunBtnText("Run");
    // Invoke the command to reset the cpu
  };

  return (
    <div className="bg-black text-white min-h-screen overflow-hidden">
      <div className="flex flex-row m-5 space-x-10">
        {/* Left side: Buttons and CodeEditor */}
        <div className="flex flex-col space-y-5">
          <div className="flex flex-row space-x-3">
            <TopButton>Assemble</TopButton>
            <TopButton onClick={() => runAsm()}>{runBtnText}</TopButton>
            <TopButton onClick={() => stop()}>Stop</TopButton>
            <TopButton>Step</TopButton>
            <TopButton onClick={() => reset()}>Reset</TopButton>
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
