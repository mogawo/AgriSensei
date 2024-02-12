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
    newSensor.onclick = function ()
    { 
        showSensorInfo(newSensor);
        // sensorDisplay.removeChild(newSensor);
    };
    newSensor.dataset.itemId = itemIdCounter++;
    newSensor.dataset.batteryLevel = 100; // Change to take level from API
    newSensor.dataset.humidity = 84;
    newSensor.dataset.recentTime = 1640; // Implement a time conversion function
    newSensor.dataset.description = "Add Description";
    newSensor.dataset.name = "Sensor " + newSensor.dataset.itemId;

    newSensor.innerHTML = `
            <div class="sensorName">
                <h2>${newSensor.dataset.name}</h2>
                <button class="nameChange" onclick="handleNameChange(event)"><img src="images/pencil.png"></button> 
            </div>
            <p>${newSensor.dataset.description}</p>
            <br>
            <button class="removeItemButton">Remove Sensor</button>
    `;

    // newSensor.addEventListener('click', () => {
    //     newSensor.classList.toggle('active');
    //     itemsContainer.querySelectorAll('.sensor').forEach(otherItem => {
    //         if (otherItem !== newSensor) {
    //             otherItem.classList.remove('active');
    //         }
    //     });
    // });

    const removeItemButton = newSensor.querySelector('.removeItemButton');
    removeItemButton.addEventListener('click', (event) => {
        event.stopPropagation();
        itemsContainer.removeChild(newSensor);
    });

    return newSensor;
}

function handleNameChange(event)
{
    event.stopPropagation();

    // console.log("Button inside div clicked");
    // sensorData.dataset.name = "New name";

    // const sensorNameElement = sensorData.querySelector('.sensorName h2');
    // sensorNameElement.innerText = sensorData.dataset.name;

    return;
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
        <div class="displayName">
            <h2>${sensorData.dataset.name}</h2>
            <p>${sensorData.dataset.description}</p>
        </div>

        <div class="displayTabs">
            <button class="summaryTab"><h3>Summary</h3></button>
            <button class="graphTab"><h3>Graph</h3></button>
        </div>

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
