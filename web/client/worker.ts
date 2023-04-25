/* @ts-ignore */
import { run, test } from 'santa-lang';

type RequestType = 'run' | 'test' | 'tokenize' | 'parse';

type Request = {
  type: RequestType;
  source: string;
};

type Response =
  | { type: 'run'; result: {} }
  | { type: 'test'; testCases: [] }
  | { type: 'tokenize'; tokens: [] }
  | { type: 'parse'; ast: {} }
  | { type: RequestType; source: string; error: {} };

let puts = (values: string[]) => console.log(...values);

let read = ([path]: [string]): string => {
  const url = new URL(path);

  if (url.protocol === 'aoc:') {
    let year = url.host;
    let day = url.pathname.substring(1);
    if (!year) [year, day] = url.pathname.substring(2).split('/');

    path = `https://raw.githubusercontent.com/eddmann/advent-of-code/master/${year}/santa-lang/aoc${year}_day${day.padStart(
      2,
      '0'
    )}.input`;
  }

  const request = new XMLHttpRequest();
  request.open('GET', path, false);
  request.send(null);

  return request.responseText.trimEnd();
};

addEventListener('message', event => {
  const request = event.data as Request;

  try {
    switch (request.type) {
      case 'run':
        postMessage({ type: 'run', result: run(event.data.source, { puts, read }) });
        return;
      case 'test':
        postMessage({ type: 'test', testCases: test(event.data.source, { puts, read }) });
        return;
      case 'tokenize':
        postMessage({ type: 'tokenize', tokens: [] });
        return;
      case 'parse':
        postMessage({ type: 'parse', ast: {} });
        return;
    }
  } catch (error) {
    postMessage({ type: request.type, source: request.source, error });
  }
});
