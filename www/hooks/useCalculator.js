import { useState, useCallback, useEffect } from "react";

export default function useCalculator() {
  const [wasmCalc, setWasmCalc] = useState(null);
  const [expression, setExpression] = useState("");
  const [result, setResult] = useState({
    type: "idle",
  });

  const computeResult = useCallback(
    (expression) => {
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
    },
    [wasmCalc]
  );

  useEffect(() => {
    setResult(computeResult(expression));
  }, [computeResult, expression]);

  useEffect(() => {
    import("wasm-calc").then((calc) => {
      setWasmCalc(() => calc);
    });
  }, []);

  return {
    expression,
    setExpression,
    result,
  };
}
