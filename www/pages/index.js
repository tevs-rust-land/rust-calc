import Container from "../components/container";
import Input from "../components/input";
import Results from "../components/results";
import Title from "../components/title";
import useCalculator from "../hooks/useCalculator";

export default function Home() {
  const { result, expression, setExpression } = useCalculator();
  const onInputChange = (e) => {
    setExpression(e.target.value);
  };
  return (
    <Container>
      <Title />
      <Input expression={expression} onInputChange={onInputChange} />
      <Results result={result} />
    </Container>
  );
}
