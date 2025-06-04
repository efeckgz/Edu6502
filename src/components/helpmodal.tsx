import * as Dialog from "@radix-ui/react-dialog";
import { X } from "lucide-react";
import { Button } from "./ui/button";

const HelpModal = () => {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button variant="secondary">Help</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-40" />
        <Dialog.Content className="fixed top-1/2 left-1/2 w-[90vw] max-w-md -translate-x-1/2 -translate-y-1/2 rounded-2xl bg-zinc-900 p-6 text-white shadow-xl z-50">
          <div className="flex items-center justify-between mb-4">
            <Dialog.Title className="text-lg font-semibold">
              Using the Emulator
            </Dialog.Title>
            <Dialog.Close asChild>
              <button
                className="text-zinc-400 hover:text-white transition"
                aria-label="Close"
              >
                <X />
              </button>
            </Dialog.Close>
          </div>
          <div className="space-y-3 text-sm">
            <p>
              💡 Write your 6502 assembly code in the editor on the left. When
              writing the code, indent the assembly instructions with 1 tab. Do
              not indent identifiers/symbols.
            </p>
            <p>
              🛠 Click <strong>Assemble</strong> to compile your program. You
              will see a window showing the assembler output. If your program
              assembles successfully, your program will be loaded to the memory
              and the cpu will be reset.
            </p>
            <p>
              ▶ Click <strong>Run</strong> to start execution.
            </p>
            <p>
              ⏸ Use <strong>Stop</strong> to pause the emulator. Click{" "}
              <strong>Continue</strong> to continue execution.
            </p>
            <p>
              🔁 Use <strong>Reset</strong> to restart with the same program.
            </p>
            <p>
              👣 Click <strong>Step</strong> to execute one cycle at a time.
            </p>
            <p>
              🔍 Watch memory and CPU registers update in real time. You can see
              the registers, status flags and bus activity below the code area.
              On the right side you can see the memory. The memory is organized
              in a 256x256 table showing all 65536 addressable memory locations.
              The rows of the table represent the page. The columns represent
              the byte. Page 01 (The second row) is the stack.
            </p>
          </div>
          <div className="mt-6 text-center">
            <Dialog.Close asChild>
              <Button variant="secondary">Got it!</Button>
            </Dialog.Close>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
};

export default HelpModal;
