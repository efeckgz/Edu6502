import "./App.css";

import { useState, useEffect } from "react";

import { invoke, Channel } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";

import AceEditor from "react-ace";
import { Button } from "./components/ui/button";

import MemoryGrid from "./components/memorygrid";
import StateView from "./components/stateview";
import HelpModal from "./components/helpmodal.tsx";

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

  const [code, setCode] = useState("");
  const [memory, setMemory] = useState<Uint8Array>(() => {
    let bytes = new Uint8Array(0x10000);
    bytes.fill(0xea);
    return bytes;
  });

  // Text to show on run button.
  // Default is Run. When stopped show Continue.
  const [runBtnText, setRunBtnText] = useState("Run");

  // Use this state to disable some components while running
  const [running, setRunning] = useState(false);

  // Keep track of processor registers
  const [internalState, setInternalState] =
    useState<InternalState>(defaultState);

  useEffect(() => {
    invoke("get_nonzero_bytes").then((r: any) => {
      loadInitialMem(r);
    });
  }, []);

  const loadInitialMem = (bytes: any) => {
    // Create a copy of memory
    const newMem = new Uint8Array(0x10000);
    newMem.fill(0xea);

    // Set the non-zero bytes
    for (let i = 0; i < bytes.length; i++) {
      const [addr, byte] = bytes[i];
      newMem[addr] = byte;
    }

    // Update React state
    setMemory(newMem);
  };

  const updateMem = (state: InternalState) => {
    if (state.rw) {
      return;
    }

    setMemory((mem) => {
      let newmem = new Uint8Array(mem);
      newmem[state.addr] = state.data;
      return newmem;
    });
  };

  // This is the Tauri channel approach
  const chan = new Channel<InternalState>();
  chan.onmessage = (m) => {
    setInternalState(m);
    updateMem(m);
  };

  const assembleAndLoad = async () => {
    setRunBtnText("Run");
    await invoke("assemble_and_load", { program: code, chan })
      .then((stdout: any) => {
        invoke("get_nonzero_bytes").then((bytes: any) => loadInitialMem(bytes));
        message(stdout, {
          title: "Program assembled successfully",
          kind: "info",
        });
      })
      .catch((stderr: any) => {
        message(stderr, { title: "Assembler failed", kind: "error" });
      });
  };

  const runAsm = async () => {
    setRunBtnText("Running...");
    setRunning(true);
    await invoke("run_asm", { chan }).catch((e) => console.log(e));
  };

  const stop = async () => {
    setRunBtnText("Continue");
    setRunning(false);
    invoke("stop");
  };

  const step = () => {
    invoke("step", { chan }).catch((e) => console.log(e));
  };

  const reset = async () => {
    setRunBtnText("Run");
    setRunning(false);

    // Resets the cpu and ram, streams the cpu state
    invoke("reset", { chan }).catch((e) => console.log(e));

    // Gets the ram
    invoke("get_nonzero_bytes").then((bytes: any) => loadInitialMem(bytes));
  };

  return (
    <div className="bg-black text-white min-h-screen overflow-hidden">
      <div className="flex flex-row m-5 space-x-10">
        {/* Left side: Buttons and CodeEditor */}
        <div className="flex flex-col space-y-5">
          <div className="flex flex-row space-x-3">
            <TopButton onClick={() => assembleAndLoad()}>Assemble</TopButton>
            <TopButton onClick={() => runAsm()} disabled={running}>
              {runBtnText}
            </TopButton>
            <TopButton onClick={() => stop()} disabled={!running}>
              Stop
            </TopButton>
            <TopButton onClick={() => step()} disabled={running}>
              Step
            </TopButton>
            <TopButton onClick={() => reset()} disabled={running}>
              Reset
            </TopButton>
            <HelpModal />
          </div>
          <div className="w-[500px]">
            <AceEditor
              height="300px"
              mode="markdown"
              theme="github"
              onChange={(newCode) => setCode(newCode)}
              name="UNIQUE_ID_OF_DIV"
              editorProps={{ $blockScrolling: true }}
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
  disabled,
}: {
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
}) => {
  return (
    <Button
      className="mx-2"
      disabled={disabled}
      variant="secondary"
      onClick={onClick}
    >
      {children}
    </Button>
  );
};

export default App;
