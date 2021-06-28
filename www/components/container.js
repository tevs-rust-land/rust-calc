import React from "react";

export default function Container({ children }) {
  return (
    <div className="container">
      {children}
      <style jsx>
        {`
          .container {
            display: flex;
            flex-direction: column;
            font: 15px Helvetica, Arial, sans-serif;
            padding: 100px;
            text-align: center;
            align-items: center;
          }
        `}
      </style>
    </div>
  );
}
