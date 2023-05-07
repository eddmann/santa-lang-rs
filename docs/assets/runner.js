/**
 * Replace code snippet copy functionality with runner
 */
const style = document.createElement('style');
style.innerHTML = `
  .md-clipboard:after {
    -webkit-mask-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 96 960 960" width="48"><path d="m383 746 267-170-267-170v340Zm97 230q-82 0-155-31.5t-127.5-86Q143 804 111.5 731T80 576q0-83 31.5-156t86-127Q252 239 325 207.5T480 176q83 0 156 31.5T763 293q54 54 85.5 127T880 576q0 82-31.5 155T763 858.5q-54 54.5-127 86T480 976Zm0-60q142 0 241-99.5T820 576q0-142-99-241t-241-99q-141 0-240.5 99T140 576q0 141 99.5 240.5T480 916Zm0-340Z"/></svg>');
    mask-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" height="48" viewBox="0 96 960 960" width="48"><path d="m383 746 267-170-267-170v340Zm97 230q-82 0-155-31.5t-127.5-86Q143 804 111.5 731T80 576q0-83 31.5-156t86-127Q252 239 325 207.5T480 176q83 0 156 31.5T763 293q54 54 85.5 127T880 576q0 82-31.5 155T763 858.5q-54 54.5-127 86T480 976Zm0-60q142 0 241-99.5T820 576q0-142-99-241t-241-99q-141 0-240.5 99T140 576q0 141 99.5 240.5T480 916Zm0-340Z"/></svg>');
  }
`;
document.head.appendChild(style);

document.addEventListener('DOMContentLoaded', () => {
  wasm_bindgen('/santa-lang-rs/assets/santa_lang_bg.wasm').then(() => {
    [].forEach.call(document.querySelectorAll('.md-clipboard.md-icon'), el => {
      const source = document.querySelector(el.dataset.clipboardTarget);

      if (!source.classList.contains('language-santa')) {
        el.remove();
        return;
      }

      source.addEventListener('dblclick', () => {
        source.contentEditable = true;
      });

      el.addEventListener('click', e => {
        e.preventDefault();
        e.stopPropagation();

        source.parentNode.nextSibling?.remove();
        const result = document.createElement('pre');
        source.parentNode.after(result);

        try {
          result.innerHTML = `<code>${wasm_bindgen.evaluate(source.innerText, {})}</code>`;
        } catch (error) {
          result.innerHTML = `<code>${error.message}</code>`;
        }
      });
    });
  });
});
