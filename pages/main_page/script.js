const addItemButton = document.getElementById('addItemButton');
const itemsContainer = document.querySelector('.sensors');
const sensorDisplay = document.querySelector('.display');

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
    // Erase anything in the current display
    const elements = document.querySelectorAll('.sensorData')
    elements.forEach(element => {
        element.remove();
    });

    const showDisplay = document.createElement('div');
    showDisplay.classList.add('sensorData')
    showDisplay.innerHTML = `
        <div class="displayName">
            <h2>${sensorData.dataset.name}</h2>
            <p>${sensorData.dataset.description}</p>
        </div>

        <div class="displayTabs">
            <button class="summaryTab" id="summaryButton"><h3>Summary</h3></button>
            <button class="graphTab" id="graphButton"><h3>Graph</h3></button>
        </div>
    `;
    sensorDisplay.appendChild(showDisplay);

    document.getElementById('summaryButton').addEventListener('click', function () {
        // Change color of all tabs accordingly
        document.getElementById('summaryButton').style.backgroundColor = 'beige';
        document.getElementById('graphButton').style.backgroundColor = 'yellowgreen';

        // Remove any currently displayed tab
        const existingSummary = showDisplay.querySelectorAll('.sensorReadings');
        existingSummary.forEach(element => {
            element.remove();
        });

        handleSummaryTab(sensorData);
    })

    document.getElementById('graphButton').addEventListener('click', function () {
        // Change color of all tabs accordingly
        document.getElementById('graphButton').style.backgroundColor = 'beige';
        document.getElementById('summaryButton').style.backgroundColor = 'yellowgreen';

        // Remove any currently displayed tab
        const existingSummary = showDisplay.querySelectorAll('.sensorReadings');
        existingSummary.forEach(element => {
            element.remove();
        });

        handleGraphTab(sensorData);
    })
}

function handleSummaryTab(sensorData)
{
    const summaryDisplay = document.createElement('div');
    summaryDisplay.classList.add('sensorReadings');

    summaryDisplay.innerHTML = `
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
    `;

    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(summaryDisplay);
    
    return;
}

function handleGraphTab(sensorData)
{ 
    console.log("Pressed the button");
    const graphDisplay = document.createElement('div');
    graphDisplay.classList.add('sensorReadings');

    graphDisplay.innerHTML = `
    <!-- Graph -->
    <div class="graphs">
        <div class="humidityGraph">
            <img src="images/HumidityGraph.png">
        </div>
        <div class="batteryGraph">
            <img src="images/BatteryGraph.png">
        </div>
    </div>
    `;

    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(graphDisplay);

    return;
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
