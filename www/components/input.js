export default function Input({ expression, onInputChange }) {
  return (
    <>
      <input placeholder="Input" value={expression} onChange={onInputChange} />
      <style jsx>{`
        input {
          padding: 15px;
          border-radius: 15px;
          border: 0;
          width: 100%;
          box-shadow: 4px 4px 10px rgba(0, 0, 0, 0.06);
          max-width: 500px;
        }
        input:focus {
          outline: none;
        }
      `}</style>
    </>
  );
}
