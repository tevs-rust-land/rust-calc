import { useState, useRef, useEffect } from "react";

export default function useCalculator() {
  const wasmCalc = useRef(null);
  const [expression, setExpression] = useState("");
  const [result, setResult] = useState({
    type: "idle",
  });

  const computeResult = ({ expression }) => {
    if (!wasmCalc) return { type: "loading" };
    try {
      const result = wasmCalc.calculate(expression);
      return {
        type: "success",
        data: result,
      };
    } catch (error) {
      return {
        type: "error",
        data: error,
      };
    }
  };

  useEffect(() => {
    setResult(computeResult(expression));
  }, [expression]);

  useEffect(() => {
    import("wasm-calc").then((calc) => {
      wasmCalc.current = calc;
    });
  }, []);

  return {
    expression,
    setExpression,
    result,
  };
}
