let buttonNames = ["Browse","Graph","New"]



export function setupNavBar(element: HTMLDivElement){
    element.innerHTML = `
    <ul id="navBarList">
    </ul>
    `
    let listElem = document.querySelector<HTMLUListElement>("#navBarList")!
    buttonNames.forEach(buttonName => {

            let temp = document.createElement("button")
            temp.appendChild(document.createTextNode(buttonName))

            temp.setAttribute("id",buttonName.toLowerCase())
            temp.setAttribute("class", "navBarBttn")

            listElem.append(temp);
    });

    
}