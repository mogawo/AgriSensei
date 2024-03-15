const addItemButton = document.getElementById('addItemButton');
// const addSensorButton = document.getElementById('addNewSensor');
const itemsContainer = document.querySelector('.sensors');
const sensorDisplay = document.querySelector('.display');
const usernameDisplay = document.createElement('div');

var loggedInUser = localStorage.getItem('loggedInUser');
var passedLogIn = localStorage.getItem('passedLogIn');

var existingSensors = [];

var currentTab = 0;

var id = loggedInUser;
id = 1;
const apiUrl = '../../user/' + String(id) + '/';

function main()
{
    fetch(apiUrl)
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            console.log(data);
            let devices = data['devices'];
            let userId = data['user_id'];
            displayUser(userId);
            for (let i = 0; i < devices.length; i++)
            {
                for (let k = 0; k < devices[i]['sensors'].length; k++) {
                    let sensorName = String(devices[i]['device_id']) + String(devices[i]['sensors'][k]['sensor_id']);
                    if (!existingSensors.includes(sensorName))
                    {
                        existingSensors.push(sensorName);
                        const newSensor = createSensorElement(devices[i], k);
                        itemsContainer.appendChild(newSensor);
                    }
                }
            }

        })
        .catch(error => {
            console.error('Error:', error);
        });
}

function displayUser(username) {
    usernameDisplay.classList.add('username');
    usernameDisplay.innerHTML = `
        <p>Logged in with user ID: ${username}<p>
    `;
    sensorDisplay.appendChild(usernameDisplay);
}


let itemIdCounter = 1;

// addItemButton.addEventListener('click', () => {
//     const newSensor = createSensorElement();
//     itemsContainer.appendChild(newSensor);
// });
// addSensorButton.addEventListener('click', () => {
//     const newSensor = createSensorElement();
//     itemsContainer.appendChild(newSensor);
//     sensorDisplay
// });

function createSensorElement(devices, k) {
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

    let sensorName = String(devices['device_id']) + String(devices['sensors'][k]['sensor_id']);
    newSensor.dataset.itemId = String(sensorName);
    itemIdCounter++;
    newSensor.dataset.batteryLevel = 100; // Change to take level from API
    newSensor.dataset.description = "Add Description";
    newSensor.dataset.name = "Sensor " + newSensor.dataset.itemId;
    let humidityArray = [];
    let timeArray = [];
    let batteryArray = [100];
    for (let i = k; i < devices['sensors'].length; i++)
    {
        if (devices['sensors'][i]['sensor_id'] != devices['sensors'][k]['sensor_id']) {
            break;
        }
        humidityArray.push(devices['sensors'][i]['value'] * 10);
        timeArray.push(0);
        // humidityArray.push(sensors[i]['amount'] * 10);
        // timeArray.push(timeConversion(sensors[i]['date_time']));
    }
    // let array = [85, 82, 81, 82, 84, 84];
    newSensor.dataset.humidityHistory = JSON.stringify(humidityArray);
    newSensor.dataset.batteryHistory = JSON.stringify(batteryArray);
    newSensor.dataset.timeHistory = JSON.stringify(timeArray);
    // Read from the list by doing
    // let humidityHistoryArray = JSON.parse(newSensor.dataset.humidityHistory); to create a new array
    // Store to the list by doing
    // newSensor.dataset.humidityHistory = JSON.stringify(humidityHistoryArray); to convert the array to a string

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

    let interval = 5000;
    setInterval(updateSensors(newSensor, devices, interval), interval);

    return newSensor;
}

function timeConversion(time) { // TODO
    return time.split('T')[1].split('.')[0];
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
    currentTab = 0;
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
    currentTab = 1;
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

    let n;
    if (humidityHistoryArray.length < 5)
    {
        n = humidityHistoryArray.length;
    }
    else
    {
        n = 5;
    }
    if (graphName == 'humidityGraph')
    {
        combinedArray = humidityHistoryArray.map((humidity, i) => [timeHistoryArray[i], humidity]);
        combinedArray = combinedArray.slice(combinedArray.length-n,combinedArray.length);
        min = find_min(humidityHistoryArray);
        max = find_max(humidityHistoryArray);
    }
    else if (graphName == 'batteryGraph')
    {
        combinedArray = batteryHistoryArray.map((battery, i) => [timeHistoryArray[i], battery]);
        combinedArray = combinedArray.slice(combinedArray.length-n,combinedArray.length);
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
    currentTab = 3;
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

function updateSensors(sensorData, devices, interval) {
    var apiUrl = '../../user/' + String(id) + '/';

    fetch(apiUrl)
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            let sensors = devices['sensors'];
            console.log('---------------');
            console.log(devices);
            console.log(sensors);

            for (let i = 0; i < sensors.length; i++)
            {
                if (sensors[i]['sensor_id'] == sensorData.dataset.itemId) {
                    for (let k = i; k < sensors.length; k++) {
                        if (sensors[k]['sensor_id'] != sensors[i]['sensor_id']) {
                            break;
                        }
                        humidityArray.push(devices['sensors'][k]['value'] * 10);
                        timeArray.push(0);
                    }
                    break;
                }
            }
            if (currentTab == 1) {
                handleDetailsTab(sensorData);
            }
            else if(currentTab == 2){
                handleGraphTab(sensorData);
            }
            else if(currentTab == 3){
                handleDetailsTab(sensorData);
            }

            setInterval(updateSensors(sensorData, devices, interval), interval);

        })
        .catch(error => {
            console.error('Error:', error);
        });
}

if (passedLogIn == 1) {
    // main(loggedInUser);
    main(1);
} 