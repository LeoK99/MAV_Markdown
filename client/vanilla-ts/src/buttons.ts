export function setupCounter(element: HTMLButtonElement) {
  let counter = 0
  const setCounter = (count: number) => {
    counter = count
    element.innerHTML = `count is ${counter}`
  }
  element.addEventListener('click', () => setCounter(counter + 1))
  setCounter(0)
}

export function setupfullStackTest(element: HTMLButtonElement) {
  const setfullStackTest = async () => {
    const resp = await fetch("http://localhost:8080/usertest", {
      method: 'GET',
    });
    element.innerHTML = `Resp: ${resp.body}`
  }
  element.addEventListener('click', () => setfullStackTest())
  setfullStackTest()
}

