const addItemButton = document.getElementById('addItemButton');
const itemsContainer = document.querySelector('.sensors');

let itemIdCounter = 0;

addItemButton.addEventListener('click', () => {
    const newItem = createItemElement();
    itemsContainer.appendChild(newItem);
});

function createItemElement() {
    const newItem = document.createElement('div');
    newItem.classList.add('sensor');
    newItem.dataset.itemId = itemIdCounter++;

    newItem.innerHTML = `
        <h2>Sensor ${newItem.dataset.itemId}</h2>
        <p>Content for sensor ${newItem.dataset.itemId} goes here.</p>
        <button class="viewSensorInfo">View Data</button>
        <br>
        <button class="removeItemButton">Remove Sensor</button>
    `;

    newItem.addEventListener('click', () => {
        newItem.classList.toggle('active');
        itemsContainer.querySelectorAll('.sensor').forEach(otherItem => {
            if (otherItem !== newItem) {
                otherItem.classList.remove('active');
            }
        });
    });

    const removeItemButton = newItem.querySelector('.removeItemButton');
    removeItemButton.addEventListener('click', () => {
        itemsContainer.removeChild(newItem);
    });

    return newItem;
}

function openSettings() {
    document.getElementById("menu").classList.toggle("show");
}
  
  // Close the dropdown menu if the user clicks outside of it
window.onclick = function(event) {
    if (!event.target.matches('.settingsBtn')) {
        var dropdowns = document.getElementsByClassName("menu");
        var i;
        for (i = 0; i < dropdowns.length; i++) {
            var openDropdown = dropdowns[i];
            if (openDropdown.classList.contains('show')) {
                openDropdown.classList.remove('show');
            }
        }
    }
}
