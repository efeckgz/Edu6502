import "./App.css";

import { useState } from "react";

import CodeEditor from "@uiw/react-textarea-code-editor";
import { Button } from "./components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./components/ui/table";
import MemoryGrid from "./components/memorygrid";

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
          <h1 className="flex justify-around text-3xl font-bold">
            Processor State
          </h1>
          <div className="flex justify-around p-2">
            {/* Registers section */}
            <div className="flex flex-col">
              <h1 className="font-bold mb-1">Registers:</h1>
              <p>PC: 0xFFFF</p>
              <p>S: 0xFF</p>
              <p>A: 0xFF</p>
              <p>X: 0xFF</p>
              <p>Y: 0xFF</p>
              <p>P: 0xFF</p>
            </div>

            {/* Flags section */}
            <div className="flex flex-col items-start">
              <h1 className="font-bold mb-1">Flags:</h1>
              <p>Carry: 1</p>
              <p>Zero: 1</p>
              <p>Interrupt: 1</p>
              <p>Decimal: 1</p>
              <p>Break: 1</p>
              <p>Overflow: 1</p>
              <p>Negative: 1</p>
            </div>
          </div>
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

const StackView = () => {
  return (
    <Table className="w-1/3">
      <TableHeader>
        <TableRow>
          <TableHead>Stack Pointer</TableHead>
          <TableHead>Value</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow>
          <TableCell>0xFF</TableCell>
          <TableCell>0x00</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  );
};

const MemoryView = () => {
  return (
    <Table className="w-full">
      {/* <TableCaption>A list of your recent invoices.</TableCaption> */}
      <TableHeader>
        <TableRow>
          <TableHead className="w-[100px]">Invoice</TableHead>
          <TableHead>Status</TableHead>
          <TableHead>Method</TableHead>
          <TableHead className="text-right">Amount</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow>
          <TableCell className="font-medium">INV001</TableCell>
          <TableCell>Paid</TableCell>
          <TableCell>Credit Card</TableCell>
          <TableCell className="text-right">$250.00</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  );
};

export default App;
