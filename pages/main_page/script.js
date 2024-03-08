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
        handleSummaryTab(newSensor);
    };
    newSensor.dataset.itemId = itemIdCounter++;
    newSensor.dataset.batteryLevel = 100; // Change to take level from API
    newSensor.dataset.humidity = 84;
    newSensor.dataset.recentTime = 1640; // Implement a time conversion function
    newSensor.dataset.description = "Add Description";
    newSensor.dataset.name = "Sensor " + newSensor.dataset.itemId;
    let array = [85, 82, 81, 82, 84];
    newSensor.dataset.humidityHistory = JSON.stringify(array);
    // Read from the list by doing
    // let humidityHistoryArray = JSON.parse(newSensor.dataset.humidityHistory); to create a new array
    // Store to the list by doing
    // newSensor.dataset.humidityHistory = JSON.stringify(humidityHistoryArray); to convert the array to a string
    let timeArray = [1600, 1610, 1620, 1630, 1640];
    newSensor.dataset.timeHistory = JSON.stringify(timeArray);

    newSensor.innerHTML = `
            <div class="sensorName">
                <h2 class='nameTag' style='background-color: darkkhaki'>${newSensor.dataset.name}</h2>
                <button class="nameChange"><img src="images/pencil.png"></button> 
            </div>
            <p>${newSensor.dataset.description}</p>
            <br>
            <button class="removeItemButton">Remove Sensor</button>
    `;

    const removeItemButton = newSensor.querySelector('.removeItemButton');
    removeItemButton.addEventListener('click', (event) => {
        event.stopPropagation();
        itemsContainer.removeChild(newSensor);
    });

    const changeNameButton = newSensor.querySelector('.nameChange');
    changeNameButton.addEventListener('click', (event) => {
        event.stopPropagation();

        newSensor.innerHTML = `
        <div class="sensorName">
            <input class='nameTag'></input>
            <button class="nameChange"><img src="images/pencil.png"></button> 
        </div>
        <p>${newSensor.dataset.description}</p>
        <br>
        <button class="removeItemButton">Remove Sensor</button>
    `;
    });

    
    const nameInput = newSensor.querySelector('.nameTag');
    nameInput.addEventListener('keypress', (event) => {
        if (event.key === 'Enter')
        {
            newSensor.dataset.name = nameInput.value;
        }
    });

    return newSensor;
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
            <button class="detailsTab" id="detailsButton"><h3>Details</h3></button>
        </div>
    `;
    sensorDisplay.appendChild(showDisplay);

    document.getElementById('summaryButton').addEventListener('click', function () {
        // CAN USE CLASSLIST.TOGGLE
        // Change color of all tabs accordingly
        document.getElementById('summaryButton').style.backgroundColor = 'beige';
        document.getElementById('graphButton').style.backgroundColor = 'yellowgreen';
        document.getElementById('detailsButton').style.backgroundColor = 'yellowgreen';

        // Remove any currently displayed tab
        const existingSummary = showDisplay.querySelectorAll('.sensorReadings');
        existingSummary.forEach(element => {
            element.remove();
        });

        handleSummaryTab(sensorData);
    })

    document.getElementById('graphButton').addEventListener('click', function () {
        // // Change color of all tabs accordingly
        document.getElementById('graphButton').style.backgroundColor = 'beige';
        document.getElementById('summaryButton').style.backgroundColor = 'yellowgreen';
        document.getElementById('detailsButton').style.backgroundColor = 'yellowgreen';

        // Remove any currently displayed tab
        const existingSummary = showDisplay.querySelectorAll('.sensorReadings');
        existingSummary.forEach(element => {
            element.remove();
        });

        handleGraphTab(sensorData);
    })

    document.getElementById('detailsButton').addEventListener('click', function () {
        // Change color of all tabs accordingly
        document.getElementById('detailsButton').style.backgroundColor = 'beige';
        document.getElementById('summaryButton').style.backgroundColor = 'yellowgreen';
        document.getElementById('graphButton').style.backgroundColor = 'yellowgreen';

        // Remove any currently displayed tab
        const existingSummary = showDisplay.querySelectorAll('.sensorReadings');
        existingSummary.forEach(element => {
            element.remove();
        });

        handleDetailsTab(sensorData);
    })
}

function handleSummaryTab(sensorData)
{
    const summaryDisplay = document.createElement('div');
    summaryDisplay.classList.add('sensorReadings');

    summaryDisplay.innerHTML = `
    <div class="humidityReading">
        <p>Humidity level: </p>
    </div>
    <div class="sensorBattery">    
        <p>Sensor Battery Level: </p>
        <div class="batteryLevel">    
            <p>${sensorData.dataset.batteryLevel}%</p>
        </div>
    </div>
    `;

    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(summaryDisplay);

    const humidityReading = document.querySelector('.humidityReading');
    let humidityHistoryArray = JSON.parse(sensorData.dataset.humidityHistory);
    let timeHistoryArray = JSON.parse(sensorData.dataset.timeHistory);
    for (let i = humidityHistoryArray.length-1; i >= humidityHistoryArray.length-5; i--)
    {
        if (i < 0) { break; }
        var newHistory = document.createElement('div');
        newHistory.classList.add('humidityList');
        newHistory.innerHTML = `
        <p>${timeHistoryArray[i]}: &nbsp${humidityHistoryArray[i]}%</p>
        `
        humidityReading.appendChild(newHistory);
    }
        
    return;
}

function handleGraphTab(sensorData)
{ 
    const graphDisplay = document.createElement('div');
    graphDisplay.classList.add('sensorReadings');

    graphDisplay.innerHTML = `
    <div class="graphs">
        <div id="humidityGraph>
            <!-- <img src="images/HumidityGraph.png"> -->
        </div>
        <div id="batteryGraph">
            <!-- <img src="images/BatteryGraph.png"> -->
        </div>
    </div>
    `;

    generateGraph(sensorData, "humidityGraph");
    generateGraph(sensorData, "batteryGraph");
    // document.getElementById

    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(graphDisplay);

    return;
}

function handleDetailsTab(sensorData)
{
    const detailsDisplay = document.createElement('div');
    detailsDisplay.classList.add('sensorReadings');
    detailsDisplay.innerHTML = `
    <div class = "details">
        <img src="images/details.png">
    </div>
    `;
    
    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(detailsDisplay);

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

function generateGraph(sensorData, graphID)
{
    let graphName = graphID;
    let humidityHistoryArray = JSON.parse(sensorData.dataset.humidityHistory);
    let batteryHistoryArray = JSON.parse(sensorData.dataset.timeHistory);
    var chart = JSC.chart(graphID, {
        type: 'line',
        series: [
            {
                name: 'Humidity',
                points: [humidityHistoryArray, batteryHistoryArray]
            }
        ]
    });
    return;
}   