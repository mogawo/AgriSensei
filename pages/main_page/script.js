const addItemButton = document.getElementById('addItemButton');
const addSensorButton = document.getElementById('addNewSensor');
const itemsContainer = document.querySelector('.sensors');
const sensorDisplay = document.querySelector('.display');
const usernameDisplay = document.createElement('div');

var loggedInUser = 'Username';

function setUser(username)
{
    console.log("ACCESSED");
    loggedInUser = username;
}

usernameDisplay.classList.add('username');
usernameDisplay.innerHTML = `
    <p>Logged in with username: ${loggedInUser}<p>
`;
sensorDisplay.appendChild(usernameDisplay);


let itemIdCounter = 0;

function loadUser(user)
{
    userId = user;
}

addItemButton.addEventListener('click', () => {
    const newSensor = createSensorElement();
    itemsContainer.appendChild(newSensor);
});
addSensorButton.addEventListener('click', () => {
    const newSensor = createSensorElement();
    itemsContainer.appendChild(newSensor);
    sensorDisplay
});

function createSensorElement() {
    const newSensor = document.createElement('div');
    newSensor.classList.add('sensor');
    newSensor.onclick = function ()
    { 
        if (usernameDisplay != null) {
            usernameDisplay.remove();
        }
        showSensorInfo(newSensor);
        handleSummaryTab(newSensor);
    };
    newSensor.dataset.itemId = itemIdCounter++;
    newSensor.dataset.batteryLevel = 100; // Change to take level from API
    newSensor.dataset.humidity = 84;
    newSensor.dataset.recentTime = 1640; // Implement a time conversion function
    newSensor.dataset.description = "Add Description";
    newSensor.dataset.name = "Sensor " + newSensor.dataset.itemId;
    let array = [85, 82, 81, 82, 84, 84];
    newSensor.dataset.humidityHistory = JSON.stringify(array);
    let array2 = [100, 100, 100, 100, 100, 100];
    newSensor.dataset.batteryHistory = JSON.stringify(array2);
    // Read from the list by doing
    // let humidityHistoryArray = JSON.parse(newSensor.dataset.humidityHistory); to create a new array
    // Store to the list by doing
    // newSensor.dataset.humidityHistory = JSON.stringify(humidityHistoryArray); to convert the array to a string
    let timeArray = [1600, 1610, 1620, 1630, 1640, 1650];
    newSensor.dataset.timeHistory = JSON.stringify(timeArray);

    newSensor.innerHTML = `
            <div class="sensorName">
                <h2 class='nameTag'>${newSensor.dataset.name}</h2>
                <button class="nameChange"><img src="images/pencil.png"></button> 
            </div>
            <div class="sensorDescription">
                <p class='descriptionTag'>${newSensor.dataset.description}</p>
                <button class="descriptionChange"><img src="images/pencil.png"></button>
            </div>
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

        const sensorNameDiv = newSensor.querySelector('.sensorName');
        sensorNameDiv.innerHTML = `
            <input class='nameTag'></input>
            <button class="nameChange"><img src="images/pencil.png"></button> 
        `;

        const nameInput = sensorNameDiv.querySelector('.nameTag');
        nameInput.focus(); // Set focus to the input field

        nameInput.addEventListener('keypress', (keypressEvent) => {
            if (keypressEvent.key === 'Enter') {
                newSensor.dataset.name = nameInput.value;
                sensorNameDiv.innerHTML = `
                    <h2 class='nameTag'>${newSensor.dataset.name}</h2>
                    <button class="nameChange"><img src="images/pencil.png"></button> 
                `;
            }
        });
    });

    const changeDescriptionButton = newSensor.querySelector('.descriptionChange');
    changeDescriptionButton.addEventListener('click', (event) => {
        event.stopPropagation();

        const sensorDescDiv = newSensor.querySelector('.sensorDescription');
        sensorDescDiv.innerHTML = `
            <input class='descriptionTag'></input>
            <button class="descriptionChange"><img src="images/pencil.png"></button> 
        `;

        const descInput = sensorDescDiv.querySelector('.descriptionTag'); // Fix the class name here
        descInput.focus(); // Set focus to the input field

        descInput.addEventListener('keypress', (keypressEvent) => {
            if (keypressEvent.key === 'Enter') {
                newSensor.dataset.description = descInput.value;
                sensorDescDiv.innerHTML = `
                    <p class='descriptionTag'>${newSensor.dataset.description}</p>
                    <button class="descriptionChange"><img src="images/pencil.png"></button>
                `;
            }
        });
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
        <p>Humidity Level: </p>
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
        <div id="humidityGraph" class="humGraph">
        </div>
        <div id="batteryGraph" class="batGraph">
        </div>
    </div>
    `;

    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(graphDisplay);

    const humidityChart = generateGraph(sensorData, "humidityGraph");
    const batteryChart = generateGraph(sensorData, "batteryGraph");

    // document.getElementById


    return;
}

function find_min(array)
{
    let min = 100;
    for (let n = 0; n < array.length; n++)
    {
        if (array[n] < min)
        {
            min = array[n]
        }
    }
    return min;
}

function find_max(array)
{
    let max = 0;
    for (let n = 0; n < array.length; n++)
    {
        if (array[n] > max)
        {
            max = array[n]
        }
    }
    return max;
}

function generateGraph(sensorData, graphID)
{
    let graphName = graphID;
    let humidityHistoryArray = JSON.parse(sensorData.dataset.humidityHistory);
    let batteryHistoryArray = JSON.parse(sensorData.dataset.batteryHistory);
    let timeHistoryArray = JSON.parse(sensorData.dataset.timeHistory);

    let combinedArray;
    let min;
    let max;
    if (graphName == 'humidityGraph')
    {
        combinedArray = humidityHistoryArray.map((humidity, i) => [timeHistoryArray[i], humidity]);
        min = find_min(humidityHistoryArray);
        max = find_max(humidityHistoryArray);
    }
    else if (graphName == 'batteryGraph')
    {
        combinedArray = batteryHistoryArray.map((battery, i) => [timeHistoryArray[i], battery]);
        min = find_min(batteryHistoryArray);
        max = find_max(batteryHistoryArray);
    }
    if (max + 10 <= 100)
    {
        max += 10;
    }
    else {
        max = 100;
    }
    if (min - 10 >= 0)
    {
        min -= 10;
    }
    else {
        min = 0;
    }
    var chart = JSC.chart(graphID, {
        debug: true,
        type: 'line',
        title_label_text: graphID === 'humidityGraph' ? 'Humidity Graph' : 'Battery Graph',
        options: {
            legend: {
                display: false
            }
        },
        series: [
            {
                points: combinedArray
            }
        ],
        yAxis: {
            scale: {
                range: [min, max]
            }
        }
    });

    return chart;
}   

function handleDetailsTab(sensorData)
{
    const detailsDisplay = document.createElement('div');
    detailsDisplay.classList.add('sensorReadings');
    detailsDisplay.innerHTML = `
    <div class='humidityReading'>
        <p>Humidity Level: </p>
    </div>
    <div class='batteryReading'></div>
    `;
    
    const showDisplay = document.querySelector('.sensorData');
    showDisplay.appendChild(detailsDisplay);

    const humidityReading = document.querySelector('.humidityReading');
    let humidityHistoryArray = JSON.parse(sensorData.dataset.humidityHistory);
    let timeHistoryArray = JSON.parse(sensorData.dataset.timeHistory); 
    for (let i = humidityHistoryArray.length-1; i >= 0; i--)
    {
        console.log(i);
        var newHistory = document.createElement('div');
        newHistory.classList.add('humidityList');
        newHistory.innerHTML = `
        <p>${timeHistoryArray[i]}: &nbsp${humidityHistoryArray[i]}%</p>
        `
        humidityReading.appendChild(newHistory);
    }
        
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
