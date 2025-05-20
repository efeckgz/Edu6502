const MemoryGrid = ({ memory }: { memory: Uint8Array }) => {
  const byteHeaders = Array.from({ length: 256 }, (_, i) => i);
  const pageIndices = Array.from({ length: 256 }, (_, i) => i);

  return (
    <div className="relative border border-gray-700 h-[700px] w-full">
      {" "}
      {/* Fixed height container */}
      <div className="overflow-auto absolute inset-0">
        {" "}
        {/* Scrollable area fills the container */}
        <table className="table-fixed border-collapse text-sm font-mono min-w-max">
          <thead>
            <tr>
              <th className="w-10 h-10 bg-gray-800 text-white text-center sticky top-0 left-0 z-20">
                Page <br /> Byte
              </th>
              {byteHeaders.map((byte) => (
                <th
                  key={byte}
                  className="w-10 h-10 bg-gray-800 text-white text-center sticky top-0 z-10"
                >
                  {byte.toString(16).padStart(2, "0").toUpperCase()}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {pageIndices.map((page) => (
              <tr key={page}>
                <th className="w-10 h-10 bg-gray-800 text-white text-center sticky left-0 z-10">
                  {page.toString(16).padStart(2, "0").toUpperCase()}
                </th>
                {byteHeaders.map((byte) => {
                  const addr = page * 256 + byte;
                  return (
                    <td
                      key={byte}
                      className="w-10 h-10 border border-gray-700 text-center text-white bg-black"
                    >
                      {memory[addr]
                        ?.toString(16)
                        .padStart(2, "0")
                        .toUpperCase()}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default MemoryGrid;
