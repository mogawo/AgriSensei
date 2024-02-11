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
    newSensor.dataset.batteryLevel = 100; // Change to take level from API
    newSensor.dataset.humidity = 84;
    newSensor.dataset.recentTime = 1645; // Implement a time conversion function

    newSensor.innerHTML = `
        <div class="sensorName">
            <h2>Sensor ${newSensor.dataset.itemId}</h2>
            <div class="changeNameButton">
                <img src="images/pencil.png">
                <button class="nameChange"></button>
            </div>
        </div>
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
        if (newSensor in showDisplay)
        {
            showDisplay.classList.remove(newSensor);
        }
    });

    const viewSensorInfo = newSensor.querySelector('.viewSensorInfo');
    viewSensorInfo.addEventListener('click', () =>{
        showSensorInfo(newSensor)
        sensorDisplay.removeChild(newSensor);
    });

    return newSensor;
}

function timeConversion(time) { // TODO
    return;
}

function showSensorInfo(sensorData) {
    const elements = document.querySelectorAll('.sensorData')
    elements.forEach(element => {
        element.remove();
    });

    const showDisplay = document.createElement('div');
    showDisplay.classList.add('sensorData')
    // Make these all divs so they can be changed in style.css
    showDisplay.innerHTML = `
        <div class="sensorDisplay">
            <h2>Sensor ${sensorData.dataset.itemId}</h2>
            <p>Content for sensor ${sensorData.dataset.itemId} goes here.</p>
            <div class="sensorReadings">
                <div class="humidityReading">
                    <p>Humidity level: </p>
                    <p>${sensorData.dataset.humidity}%</p>
                </div>
                <div class="sensorBattery">    
                    <p>Sensor Battery Level: </p>
                    <p>${sensorData.dataset.batteryLevel}%</p>
                </div>
                <div class="readingTime">
                    <p>Time of Last Reading: </p>
                    <p>${sensorData.dataset.recentTime}<p>
                </div>
            </div>
            <div class="history">
                <h>History:</h>
            </div>
        </div>
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
