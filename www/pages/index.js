import { useState, useEffect } from "react";
import { debounce } from "throttle-debounce";

export default function Home() {
  const [expression, setExpression] = useState("");
  const [result, setResult] = useState({
    type: "idle",
  });
  const onInputChange = (e) => {
    setExpression(e.target.value);
  };

  useEffect(() => {
    import("wasm-calc").then((calc) => {
      try {
        const result = calc.calculate(expression);
        setResult({
          type: "success",
          data: result,
        });
      } catch (error) {
        setResult({
          type: "error",
          data: error,
        });
      }
    });
  }, [expression]);
  return (
    <div className="container">
      <h3>
        🦀 <span className="title">+</span> 🕸
        <span className="title">Calculator</span>
      </h3>
      <input placeholder="Input" value={expression} onChange={onInputChange} />
      {result.type === "success" && <h4>{result.data}</h4>}
      {result.type === "error" && <pre>{result.data}</pre>}
      <style jsx>{`
        .title {
          margin-left: 5px;
        }
        .container {
          display: flex;
          flex-direction: column;
          font: 15px Helvetica, Arial, sans-serif;
          padding: 100px;
          text-align: center;
          align-items: center;
        }
        input {
          padding: 15px;
          border-radius: 15px;
          border: 0;
          width: 100%;
          box-shadow: 4px 4px 10px rgba(0, 0, 0, 0.06);
        }
        input:focus {
          outline: none;
        }
        pre {
          font-weight: bold;
          white-space: pre-wrap;
          word-wrap: break-word;
        }
      `}</style>

      <style jsx global>{`
        body {
          background: #e1e1e1;
        }
      `}</style>
    </div>
  );
}
