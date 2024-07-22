import initMiner from './wasm/examples/miner.js'
import initCells from './wasm/examples/cells.js'

import { highlightAll } from './3rd-party/speed-highlight.js'

const example = window.location.hash.substr(1) || "cells"

if (example === "miner") {
  initMiner()
} else if (example === "cells") {
  initCells()
} else {
  // impossible?!
}

document.querySelector(`a[href="#${example}"]`).classList.add("active")

fetch(`./sources/${example}.rs`)
  .then(res => res.text())
  .then(text => {
    document.querySelector('#code').textContent = text;
    highlightAll()
  })

window.onhashchange = () => {
  window.location.reload()
}
