import "./App.css";

import { useState } from "react";

import CodeEditor from "@uiw/react-textarea-code-editor";

import { Button } from "./components/button";
import { ScrollArea } from "./components/scrollarea";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./components/table";

function App() {
  const [code, setCode] = useState("lda #$42");

  return (
    <div className="bg-black text-white min-h-screen">
      <div className="flex flex-col mb-3 z-10">
        <div className="flex flex-row m-5">
          <TopButton>Assemble</TopButton>
          <TopButton>Run</TopButton>
          <TopButton>Step</TopButton>
          <TopButton>Reset</TopButton>
        </div>
        <div className="w-1/3 mx-5">
          <CodeEditor
            value={code}
            language="plaintext"
            placeholder="Write your assembly here."
            onChange={(evn) => setCode(evn.target.value)}
            padding={15}
            style={{
              // backgroundColor: "#f5f5f5",
              minHeight: 300,
              backgroundColor: "gray",
              color: "black",
              fontFamily:
                "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
            }}
          />
        </div>
      </div>
      {/* <StackView /> */}
      <StackTable />
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
  const tags = Array.from({ length: 50 }).map(
    (_, i, a) => `v1.2.0-beta.${a.length - i}`,
  );

  return (
    <ScrollArea className="h-72 w-48 rounded-md border">
      <div className="p-4">
        <h4 className="mb-4 text-sm font-medium leading-none">Tags</h4>
        {tags.map((tag) => (
          <>
            <div key={tag} className="text-sm">
              {tag}
            </div>
            {/* <Separator className="my-2" /> */}
          </>
        ))}
      </div>
    </ScrollArea>
  );
};

const StackTable = () => {
  return (
    <Table>
      {/* <TableCaption>A list of your recent invoices.</TableCaption> */}
      <TableHeader>
        <TableRow>
          <TableHead className="w-[100px]">Stack Pointer</TableHead>
          <TableHead>Value</TableHead>
          {/* <TableHead>Method</TableHead> */}
          {/* <TableHead className="text-right">Amount</TableHead> */}
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow>
          <TableCell className="font-medium">0xFF</TableCell>
          <TableCell>0x00</TableCell>
          {/* <TableCell>Paid</TableCell> */}
          {/* <TableCell>Credit Card</TableCell> */}
          {/* <TableCell className="text-right">$250.00</TableCell> */}
        </TableRow>
      </TableBody>
    </Table>
  );
};

export default App;
