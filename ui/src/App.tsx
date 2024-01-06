import { processBytes } from "libray";
import { ChangeEvent, useState } from "react";

export const App = () => {
  const [frequency, setFrequency] = useState<number>();

  const onFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files?.length !== 1) return;

    const file = e.target.files[0];
    const reader = new FileReader();

    reader.onload = (event) => {
      if (
        event.target?.result == null ||
        !(event.target.result instanceof ArrayBuffer)
      )
        return;

      const resultArray = new Uint8Array(event.target.result);
      setFrequency(processBytes(resultArray));
    };

    reader.readAsArrayBuffer(file);
  };

  const getFrequency = () => {
    if (frequency === undefined) return "Please select a file.";

    return `${frequency} Hz is the dominant Frequency`;
  };

  return (
    <>
      {getFrequency()}
      <input type="file" onChange={onFileChange} />
    </>
  );
};
