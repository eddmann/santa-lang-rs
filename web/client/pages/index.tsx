import { useState, useCallback, useEffect, useRef } from 'react';
import type { NextPage } from 'next';
import Head from 'next/head';
import dynamic from 'next/dynamic';
import Split from 'react-split';
import Navigation from '../components/Navigation';
const Editor = dynamic(() => import('../components/Editor'), { ssr: false });

function* range(start: number, end: number) {
  for (let i = start; i <= end; i++) {
    yield i;
  }
}

type SourceLocation = { start: number; end: number };

const calculateLineColumn = (
  source: string,
  location: SourceLocation
): { line: number; column: number } => {
  let line = 0;
  let column = 0;

  for (let position = 0; position < source.length; position++) {
    if (position === location.start) {
      return { line, column };
    }

    column++;
    if (source[position] === '\n') {
      line++;
      column = 0;
    }
  }

  throw new Error();
};

const generateErrorMessage = (
  source: string,
  message: string,
  location: SourceLocation,
  trace: SourceLocation[]
): string => {
  let { line, column } = calculateLineColumn(source, location);

  let output = `${message}\n\n`;

  const lines = source.split('\n');
  for (let i = 0; i < lines.length; i++) {
    if (i < line - 2 || i > line + 2) continue;
    const lineNo = `${i + 1}`.padStart(2, '0') + ': ';

    if (i === line) {
      output += `${lineNo}${lines[i]}\n`;
      output += ' '.repeat(column + lineNo.length) + '^~~\n';
    } else {
      output += `${lineNo}${lines[i]}\n`;
    }
  }

  output += `\neditor:${line + 1}:${column + 1}\n`;

  for (const location of trace) {
    let { line, column } = calculateLineColumn(source, location);
    output += `  ${source.substring(location.start, location.end).split('\n')[0]}:${line + 1}:${
      column + 1
    }\n`;
  }

  return output + '\n';
};

const WorkspaceEditor = () => {
  const [source, setSource] = useState('');
  const [result, setResult] = useState('');
  const [isRunning, setRunning] = useState(false);
  const worker = useRef<Worker>();

  useEffect(() => {
    worker.current = new Worker(new URL('../worker.ts', import.meta.url));
    worker.current.onmessage = event => {
      setRunning(false);

      const response = event.data;

      if (response.error) {
        setResult(
          generateErrorMessage(
            response.source,
            response.error.message,
            response.error.source,
            response.error.trace
          )
        );
        return;
      }

      let output = '';

      switch (response.type) {
        case 'run':
          const { result } = response;

          if (result.value) {
            setResult(result.value);
            return;
          }

          if (result.part_one) {
            output += `Part 1: ${result.part_one.value} ${result.part_one.duration}ms\n`;
          }

          if (result.part_two) {
            output += `Part 2: ${result.part_two.value} ${result.part_two.duration}ms\n`;
          }

          setResult(output);
          return;
        case 'test':
          const { testCases } = response;

          for (const [idx, testCase] of Object.entries(testCases) as any) {
            if (+idx > 0) output += `\n`;
            output += `Testcase ${+idx + 1}\n`;

            if (!testCase) {
              output += 'No expectations\n';
              continue;
            }

            if (testCase.part_one) {
              if (testCase.part_one.passed) {
                output += `Part 1: ${testCase.part_one.actual} ✔️\n`;
              } else {
                output += `Part 1: ${testCase.part_one.actual} ✘ (Expected: ${testCase.part_one.expected})\n`;
              }
            }

            if (testCase.part_two) {
              if (testCase.part_two.passed) {
                output += `Part 2: ${testCase.part_two.actual} ✔️\n`;
              } else {
                output += `Part 2: ${testCase.part_two.actual} ✘ (Expected: ${testCase.part_two.expected})\n`;
              }
            }
          }

          setResult(output);
          return;
        case 'tokenize':
          return;
        case 'parse':
          return;
      }
    };

    return () => {
      worker.current && worker.current.terminate();
    };
  }, []);

  const handleRun = useCallback(() => {
    if (isRunning) return;
    setRunning(true);
    setResult('Running...');
    worker.current && worker.current.postMessage({ type: 'run', source });
  }, [source, isRunning]);

  const handleTest = useCallback(() => {
    if (isRunning) return;
    setRunning(true);
    setResult('Testing...');
    worker.current && worker.current.postMessage({ type: 'test', source });
  }, [source, isRunning]);

  const handleExample = (e: React.ChangeEvent<HTMLSelectElement>) => {
    fetch(e.target.value)
      .then(response => response.text())
      .then(setSource);
  };

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === 'Enter') {
        handleRun();
      }
    };

    window.addEventListener('keydown', onKeyDown);

    return () => {
      window.removeEventListener('keydown', onKeyDown);
    };
  }, [handleRun]);

  return (
    <div>
      <div
        style={{
          backgroundColor: '#efefef',
          borderBottom: '1px solid #ddd',
          height: 32,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '0 10px',
        }}
      >
        <div>
          <select onChange={handleExample} defaultValue="title">
            <option value="title" disabled>
              Select an example...
            </option>
            <optgroup label="2018">
              {[...range(1, 14)].map(day => {
                const fileName = `aoc2018_day${('' + day).padStart(2, '0')}.santa`;

                return (
                  <option
                    key={fileName}
                    value={`https://raw.githubusercontent.com/eddmann/advent-of-code/master/2018/santa-lang/${fileName}`}
                  >
                    {fileName}
                  </option>
                );
              })}
            </optgroup>
            <optgroup label="2022">
              {[...range(1, 25)].map(day => {
                const fileName = `aoc2022_day${('' + day).padStart(2, '0')}.santa`;

                return (
                  <option
                    key={fileName}
                    value={`https://raw.githubusercontent.com/eddmann/advent-of-code/master/2022/santa-lang/${fileName}`}
                  >
                    {fileName}
                  </option>
                );
              })}
            </optgroup>
          </select>
        </div>
        <div>
          <button onClick={handleTest} disabled={isRunning}>
            Test
          </button>{' '}
          <button onClick={handleRun} disabled={isRunning}>
            Run
          </button>
        </div>
      </div>
      <Split
        direction="vertical"
        style={{ height: `calc(100vh - 64px)` }}
        sizes={[60, 40]}
        minSize={[200, 200]}
      >
        <div>
          <Editor onChange={setSource} source={source} />
        </div>
        <pre
          style={{
            margin: 0,
            padding: '20px',
            overflowY: 'scroll',
            fontFamily: 'monospace',
            fontSize: 16,
          }}
        >
          {result}
        </pre>
      </Split>
    </div>
  );
};

const Workspace: NextPage = () => {
  return (
    <div>
      <Head>
        <title>santa-lang-rs</title>
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no"
        />
      </Head>
      <div>
        <Navigation />
        <WorkspaceEditor />
      </div>
    </div>
  );
};

export default Workspace;
