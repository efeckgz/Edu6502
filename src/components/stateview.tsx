export type InternalState = {
  // Registers
  pc: number;
  s: number;
  a: number;
  x: number;
  y: number;
  p: number;

  // Bus
  addr: number;
  data: number;
  rw: boolean;
};

type Props = {
  state: InternalState | null;
};

const StateView = ({ state: state }: Props) => {
  if (!state) {
    return <p>Status not available.</p>;
  }

  return (
    <>
      <h1 className="flex justify-around text-3xl font-bold">
        Processor State
      </h1>
      <div className="flex justify-around p-2">
        {/* Registers section */}
        <div className="flex flex-col">
          <h1 className="font-bold mb-1">Registers:</h1>
          <p>PC: 0x{state.pc.toString(16).padStart(4, "0").toUpperCase()}</p>
          <p>S: 0x{state.s.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>A: 0x{state.a.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>X: 0x{state.x.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>Y: 0x{state.y.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>P: 0x{state.p.toString(16).padStart(2, "0").toUpperCase()}</p>
        </div>

        {/* Flags section */}
        <div className="flex flex-col items-start">
          <h1 className="font-bold mb-1">Flags:</h1>
          <p>Carry: {state.p & 0x01 ? 1 : 0}</p>
          <p>Zero: {state.p & 0x02 ? 1 : 0}</p>
          <p>Interrupt: {state.p & 0x04 ? 1 : 0}</p>
          <p>Decimal: {state.p & 0x08 ? 1 : 0}</p>
          <p>Break: {state.p & 0x10 ? 1 : 0}</p>
          <p>Overflow: {state.p & 0x40 ? 1 : 0}</p>
          <p>Negative: {state.p & 0x80 ? 1 : 0}</p>
        </div>

        {/* Bus Activity section */}
        <div className="flex flex-col items-start">
          <h1 className="font-bold mb-1">Bus Activity:</h1>
          {state ? (
            <>
              <p>
                Address: 0x
                {state.addr.toString(16).padStart(4, "0").toUpperCase()}
              </p>
              <p>
                Data: 0x{state.data.toString(16).padStart(2, "0").toUpperCase()}
              </p>
              <p>Activity: {state.rw ? "Read" : "Write"}</p>
            </>
          ) : (
            <p>No activity</p>
          )}
        </div>
      </div>
    </>
  );
};

export default StateView;
