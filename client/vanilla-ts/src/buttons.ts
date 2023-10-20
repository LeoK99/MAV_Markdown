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
    const resp = await fetch("127.0.0.1:8080/usertest", {method: 'GET'})
    element.innerHTML = `${resp}`
  }
  element.addEventListener('click', () => setfullStackTest())
  setfullStackTest()
}

