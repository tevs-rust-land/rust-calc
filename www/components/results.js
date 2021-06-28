export default function Results({ result }) {
  return (
    <>
      {result.type === "success" && <h4>{result.data}</h4>}
      {result.type === "error" && <pre>{result.data}</pre>}
      <style jsx>{`
        pre {
          font-weight: bold;
          white-space: pre-wrap;
          word-wrap: break-word;
        }
      `}</style>
    </>
  );
}
