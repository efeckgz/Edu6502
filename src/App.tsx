import "./App.css";

import { useState } from "react";

import CodeEditor from "@uiw/react-textarea-code-editor";

import { Button } from "./components/button";
import { ScrollArea } from "./components/scrollarea";

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
      <StackView />
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

export default App;
