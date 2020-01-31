import CurrentMap from "./current";
import HistoryMap from "./history"

const current = new CurrentMap("current-map", "sidebar", "current-list");
const history = new HistoryMap("history-map", "sidebar", "history-list");

let reconnected = false;
let geojson = {
    features: [],
    type: "FeatureCollection"
};

function add(json) {
    geojson.features = geojson.features.concat(json.features);
    current.add(json);
}

const historyToggle = document.getElementById('history-toggle');
historyToggle.onclick = function() {
    const listHeader = document.getElementById("list-header");
    const currentContainer = document.getElementById("current-map");
    const currentList = document.getElementById("current-list");
    const historyContainer = document.getElementById("history-map");
    const historyList = document.getElementById("history-list");
    currentContainer.classList.toggle("hide");
    currentList.classList.toggle("hide");
    historyContainer.classList.toggle("hide");
    historyList.classList.toggle("hide");
    if (!historyContainer.classList.contains("hide")) {
        listHeader.innerHTML = listHeader.innerHTML.replace("24h", "History");
        history.clear();
        history.leafletMap._onResize();
        history.load(geojson);
        historyToggle.classList.replace("fa-history", "fa-flash");
    } else {
        listHeader.innerHTML = listHeader.innerHTML.replace("History", "24h");
        historyToggle.classList.replace("fa-flash", "fa-history");
    }
};

function connect() {
    const wsUri = (window.location.protocol==='https:'&&'wss://'||'ws://') + window.location.host + '/ws/';
    let ws = new WebSocket(wsUri);

    ws.onopen = function() {
        // TODO: set config here.
    };

    ws.onmessage = function(event) {
        const json = JSON.parse(event.data);
        console.log('New quakes ', json);

        if(reconnected) current.clear();
        add(json);
        reconnected = false;
    };

    ws.onclose = function(e) {
        console.log('Socket is closed. Reconnect will be attempted in 10 seconds.', e.reason);
        setTimeout(function() {
            reconnected = true;
            connect();
        }, 10000);
    };

    ws.onerror = function(err) {
        console.error('Socket encountered error: ', err.message, 'Closing socket');
        ws.close();
    };
}

connect();

// let counter = 0;
//
// function createDummy() {
//     return { "features":[
//             {
//                 "geometry":{"coordinates":[120.93-counter,13.77],"type":"Point"},
//                 "properties":{"datetime":moment().utc().toISOString(),"depth":134,"end":moment().utc().add(8, 'hours').toISOString(),"latitude":13.77,"location":"Some location","longitude":120.93-counter,"magnitude":2.4,"province":"Some province " + counter,"start":moment().utc().toISOString(),"url":"http://example.com"},"type":"Feature"}
//         ],
//         "type":"FeatureCollection"
//     };
// }
//
// document.getElementById('add-to-list').onclick = function() {
//     add(createDummy());
//     counter = counter + 1;
// };
