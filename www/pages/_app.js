function MyApp({ Component, pageProps }) {
  return (
    <>
      <Component {...pageProps} />
      <style jsx global>{`
        body {
          background: #e1e1e1;
        }
      `}</style>
    </>
  );
}

export default MyApp;
