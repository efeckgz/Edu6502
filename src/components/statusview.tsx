type Registers = {
  pc: number;
  s: number;
  a: number;
  x: number;
  y: number;
  p: number;
};

type Props = {
  registers: Registers | null;
};

const StatusView = ({ registers }: Props) => {
  if (!registers) {
    return <p>Status now available.</p>;
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
          <p>
            PC: 0x{registers.pc.toString(16).padStart(4, "0").toUpperCase()}
          </p>
          <p>S: 0x{registers.s.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>A: 0x{registers.a.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>X: 0x{registers.x.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>Y: 0x{registers.y.toString(16).padStart(2, "0").toUpperCase()}</p>
          <p>P: 0x{registers.p.toString(16).padStart(2, "0").toUpperCase()}</p>
        </div>

        {/* Flags section */}
        <div className="flex flex-col items-start">
          <h1 className="font-bold mb-1">Flags:</h1>
          <p>Carry: {registers.p & 0x01 ? 1 : 0}</p>
          <p>Zero: {registers.p & 0x02 ? 1 : 0}</p>
          <p>Interrupt: {registers.p & 0x04 ? 1 : 0}</p>
          <p>Decimal: {registers.p & 0x08 ? 1 : 0}</p>
          <p>Break: {registers.p & 0x10 ? 1 : 0}</p>
          <p>Overflow: {registers.p & 0x40 ? 1 : 0}</p>
          <p>Negative: {registers.p & 0x80 ? 1 : 0}</p>
        </div>
      </div>
    </>
  );
};

export default StatusView;
