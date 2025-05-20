import "./App.css";

import { useState } from "react";

import CodeEditor from "@uiw/react-textarea-code-editor";

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
              backgroundColor: "white",
              color: "black",
              fontFamily:
                "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
            }}
          />
        </div>
      </div>
    </div>
  );
}

const TopButton = ({ children }: { children: React.ReactNode }) => {
  return <button className="mx-2">{children}</button>;
};

export default App;
