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

function display(show, hide) {
    document.getElementById(show + "-map").classList.remove("hide");
    document.getElementById(show + "-list").classList.remove("hide");
    document.getElementById(hide + "-map").classList.add("hide");
    document.getElementById(hide + "-list").classList.add("hide");
    const header = document.getElementById("list-header");
    if(show === "history") {
        header.innerHTML = header.innerHTML.replace("24h", "History");
        history.clear();
        history.leafletMap._onResize();
        history.load(geojson);
    } else {
        header.innerHTML = header.innerHTML.replace("History", "24h");
    }
}

document.getElementById("history-toggle").onclick = function() {
    this.classList.add("hide");
    display("history", "current");
    document.getElementById("current-toggle").classList.remove("hide");
};

document.getElementById("current-toggle").onclick = function() {
    this.classList.add("hide");
    display("current", "history");
    document.getElementById("history-toggle").classList.remove("hide");
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

