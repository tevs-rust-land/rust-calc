import { useEffect } from "react";
import { debounce } from "throttle-debounce";

export default function Home() {
  useEffect(() => {
    import("wasm-calc").then((wasm) => {
      try {
        console.log(typeof wasm.calculate("1 + 12"));

        console.log(wasm.calculate("1 + 12"));
      } catch (error) {
        console.log(error);
      }
    });
  }, []);
  return (
    <div className="container">
      <h3>
        🦀 <span className="title">+</span> 🕸 {"    "}{" "}
        <span className="title">Calculator</span>
      </h3>
      <input placeholder="Input" />
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
      `}</style>

      <style jsx global>{`
        body {
          background: #e1e1e1;
        }
      `}</style>
    </div>
  );
}
