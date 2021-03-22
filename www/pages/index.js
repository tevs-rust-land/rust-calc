import { useEffect } from "react";

export default function Home() {
  useEffect(() => {
    import("wasm-calc").then((wasm) => {
      console.log(wasm.calculate("1 + 1"));
    });
  }, []);
  return (
    <div className="hello">
      <p>Hello World</p>

      <style jsx>{`
        .hello {
          font: 15px Helvetica, Arial, sans-serif;
          background: #eee;
          padding: 100px;
          text-align: center;
          transition: 100ms ease-in background;
        }
        .hello:hover {
          background: #ccc;
        }
      `}</style>
    </div>
  );
}
