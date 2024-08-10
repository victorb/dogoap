import initMiner from './wasm/examples/miner.js'
import initCells from './wasm/examples/cells.js'
import initLemonade from './wasm/examples/resman.js'
import initNeumann from './wasm/examples/von_neumann.js'
import {highlightAll} from 'https://cdn.jsdelivr.net/gh/speed-highlight/core/dist/index.js'

const example = window.location.hash.substr(1) || "cells"

if (example === "miner") {
  initMiner()
} else if (example === "cells") {
  initCells()
} else if (example === "resman") {
  initLemonade()
} else if (example === "von_neumann") {
  initNeumann()
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
