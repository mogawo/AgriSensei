const addItemButton = document.getElementById('addItemButton');
const itemsContainer = document.querySelector('.sensors');
const sensorDisplay = document.querySelector('.display')

let itemIdCounter = 0;

addItemButton.addEventListener('click', () => {
    const newSensor = createSensorElement();
    itemsContainer.appendChild(newSensor);
});

function createSensorElement() {
    const newSensor = document.createElement('div');
    newSensor.classList.add('sensor');
    newSensor.dataset.itemId = itemIdCounter++;

    newSensor.innerHTML = `
        <h2>Sensor ${newSensor.dataset.itemId}</h2>
        <p>Content for sensor ${newSensor.dataset.itemId} goes here.</p>
        <button class="viewSensorInfo">View Data</button>
        <br>
        <button class="removeItemButton">Remove Sensor</button>
    `;

    newSensor.addEventListener('click', () => {
        newSensor.classList.toggle('active');
        itemsContainer.querySelectorAll('.sensor').forEach(otherItem => {
            if (otherItem !== newSensor) {
                otherItem.classList.remove('active');
            }
        });
    });

    const removeItemButton = newSensor.querySelector('.removeItemButton');
    removeItemButton.addEventListener('click', () => {
        itemsContainer.removeChild(newSensor);
    });

    const viewSensorInfo = newSensor.querySelector('.viewSensorInfo');
    viewSensorInfo.addEventListener('click', () =>{
        showSensorInfo(newSensor)
    });

    return newSensor;
}

function showSensorInfo(sensorData) {
    const elements = document.querySelectorAll('.sensorData')
    elements.forEach(element => {
        element.remove();
    });

    const showDisplay = document.createElement('div');
    showDisplay.classList.add('sensorData')

    showDisplay.innerHTML = `
        <h2>Sensor ${sensorData.dataset.itemId}</h2>
        <p>Content for sensor ${sensorData.dataset.itemId} goes here.</p>
        <p>pH level for sensor goes here.</p>
        <p>Temperature level for sensor goes here.</p>
        <p>Humidity level for sensor goes here.</p>
        <p>sensorData ${sensorData}.</p>
    `;

    sensorDisplay.appendChild(showDisplay)
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
