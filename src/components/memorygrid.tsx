const MemoryGrid = ({ memory }: { memory: Uint8Array }) => {
  const byteHeaders = Array.from({ length: 16 }, (_, i) => i);
  const pageIndices = Array.from({ length: 16 }, (_, i) => i);

  return (
    <div className="overflow-auto border border-gray-700">
      <table className="table-fixed border-collapse text-sm font-mono">
        <thead>
          <tr>
            <th className="w-10 h-10 bg-gray-800 text-white text-center"> </th>
            {byteHeaders.map((byte) => (
              <th
                key={byte}
                className="w-10 h-10 bg-gray-800 text-white text-center"
              >
                {byte.toString(16).padStart(2, "0").toUpperCase()}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {pageIndices.map((page) => (
            <tr key={page}>
              <th className="w-10 h-10 bg-gray-800 text-white text-center">
                {page.toString(16).padStart(2, "0").toUpperCase()}
              </th>
              {byteHeaders.map((byte) => {
                const addr = page * 16 + byte;
                return (
                  <td
                    key={byte}
                    className="w-10 h-10 border border-gray-700 text-center text-white bg-black"
                  >
                    {memory[addr]?.toString(16).padStart(2, "0").toUpperCase()}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default MemoryGrid;
