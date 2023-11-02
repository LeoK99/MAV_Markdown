import './style.css'
import { setupCounter } from './buttons.ts'
import { setupfullStackTest } from './buttons.ts'
import { ink } from 'ink-mde'
import { setupNavBar} from './navbar.ts'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <div class="card">
      <button id="fullStackTest" type="button"></button>
    </div>
    <div id="editor"></div>
  </div>
`
ink(document.querySelector<HTMLDivElement>("#editor")!,{
  doc: "# Hello World!",
})
setupNavBar(document.querySelector<HTMLDivElement>("#navBar")!)
setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
setupfullStackTest(document.querySelector<HTMLButtonElement>("#fullStackTest")!)
