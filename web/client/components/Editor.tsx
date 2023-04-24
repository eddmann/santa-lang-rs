import * as React from 'react';
import CodeMirror from '@uiw/react-codemirror';
import { rust } from '@codemirror/lang-rust';

export default function Editor({
  source,
  onChange,
}: {
  source: string;
  onChange: (value: string) => void;
}): React.ReactElement {
  return (
    <CodeMirror
      style={{ height: '100%' }}
      value={source}
      autoFocus={false}
      height="100%"
      extensions={[rust()]}
      onChange={onChange}
    />
  );
}
