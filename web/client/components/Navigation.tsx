import * as React from 'react';

export default function Navigation(): React.ReactElement {
  return (
    <nav
      style={{
        height: 32,
        backgroundColor: '#efefef',
        borderBottom: '1px solid #ddd',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '0 16px',
      }}
    >
      <div>santa-lang-rs</div>
      <div></div>
    </nav>
  );
}
