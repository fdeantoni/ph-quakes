let mymap = L.map('mapid', {
    center: [12.5, 120.91],
    zoom: 5,
    maxZoom: 18
});

const mapboxUrl = 'https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}';

const mapboxConfig = {
    attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
    id: 'mapbox/outdoors-v11',
    accessToken: 'pk.eyJ1IjoiZmRlYW50b25pIiwiYSI6ImNrNWhhOHlueTAxcHAzZHA3Nnd1MDhveWkifQ.kTW32UkDDmHFl9MGhnNrbw'
};

L.tileLayer(mapboxUrl, mapboxConfig).addTo(mymap);

let lastTimestamp = moment.unix(0).utc();

function updateList(layer, bounded = true) {

    const displayed = layer.getLayers().sort(function(a,b) {
        const first = moment(a.feature.properties.datetime);
        const second = moment(b.feature.properties.datetime);
        return second - first;
    });
    document.getElementById('displayed-list').innerHTML = "";
    displayed.forEach(function(quake){
        const layerId = layer.getLayerId(quake);
        const props = quake.feature.properties;
        let fade = "";
        if(moment(props.datetime).isAfter(lastTimestamp)) {
            fade = " fade-in list-item-new";
        }
        const inBounds = mymap.getBounds().contains({lat: props.latitude, lng: props.longitude});
        if( !bounded || (bounded && inBounds) ) {
            const html = '<li data-layer-id="'+ layerId + '">' +
                '<div class="list-item-container' + fade + '">' +
                '<span class="list-item-magnitude">' + props.magnitude + '</span>' +
                '<h1 class="list-item-location">' + props.province + '</h1>' +
                '<h2 class="list-item-utc">' + props.start + '</h2>' +
                '<aside class="list-item-aside">' + props.depth + ' km</aside>' +
                '</div></li>';
            $('#displayed-list').append(html);
        }
    });

    if(displayed.length > 0) {
        let lastItem = displayed[0];
        lastTimestamp = moment(lastItem.feature.properties.datetime);
    }

    console.log("Last layer timestamp ", lastTimestamp.toISOString());

    $('.list-view > li').click(function(e) {
        $('.list-view > li').removeClass('list-item-selected');
        $(this).addClass('list-item-selected');
        const marker = layer.getLayer($(this).attr('data-layer-id'));
        if(typeof layer.zoomToShowLayer == 'function') {
            layer.zoomToShowLayer(marker, function() {
                marker.openPopup();
            });
        } else {
            mymap.flyTo(marker.getLatLng(), 14);
            marker.openPopup();
        }
    });
}

function markerPopup(feature, layer) {
    if (feature.properties) {
        const props = feature.properties;
        const header = '<h3>' + props.province + ' ' + props.longitude + ', ' + props.latitude + '</h3>';
        const details = '<ul style="list-style-type:none;padding-left: 0;">' +
            '<li><b>Magnitude: </b>' + props.magnitude + '</li>' +
            '<li><b>Depth:     </b>' + props.depth + '</li>' +
            '<li><b>Location:  </b>' + props.location + '</li>' +
            '<li><b>Source:    </b><a href="'+ props.url +'" target="_blank">philvolcs</li>' +
            '</ul>';
        layer.bindPopup(header + details);
    }
}

function markerIcon(size, text) {
    const style = "width: " + size + "px; height: " + size + "px; line-height: " + size + "px; background-image: url('/static/quakes/quake.png'); text-align: center; background-size: 100%; margin-top: 0px;";
    const html = '<div style="'+ style +'"><b style="color:whitesmoke;">' + text + '</b></div>';
    return L.divIcon({ html: html, className: 'mycluster', iconSize: L.point(size, size) });
}

function radius(magnitude, depth) {
    let size = Math.ceil(Math.exp(magnitude) / depth);
    if(size < 5) size = 5;
    return size
}

function currentMarkers(json) {
    return L.geoJSON(json, {
        pointToLayer: function(feature, latlng) {
            if (feature.properties) {
                return new L.marker(latlng, {
                    icon: markerIcon(20,1),
                    zIndexOffset: 1000
                });
            }
        },
        onEachFeature: markerPopup
    });
}

function filterOld(json) {
    const horizon = moment().utc().subtract(24, 'hours');
    const filtered = json.features.filter(function(item) {
        return moment.utc(item.properties.datetime).isAfter(horizon);
    });
    return {
        type: "FeatureCollection",
        features: filtered
    };
}

function loadCurrent(map, json) {

    const currentQuakes = filterOld(json);

    let cluster = L.markerClusterGroup({
        iconCreateFunction: function(cluster) {
            const size = Math.log(cluster.getChildCount())*40;
            return markerIcon(size, cluster.getChildCount());
        }
    });
    let layer = currentMarkers(currentQuakes);
    cluster.addLayer(layer);
    map.addLayer(cluster);
    updateList(cluster);

    cluster.on('animationend', function(e){
        updateList(e.target);
    });

    cluster.on('add', function() {
        updateList(cluster);
    });

    return cluster;
}

function updateCurrent(current, json) {
    const currentQuakes = filterOld(json);
    current.clearLayers();
    let updates = currentMarkers(currentQuakes);
    updates.addTo(current);
    updateList(current);
}

function historyMarkers(json) {
    return L.timeline(json, {
        pointToLayer: function(feature, latlng) {
            if (feature.properties && feature.properties.magnitude && feature.properties.depth) {
                return new L.circleMarker(latlng, {
                    className: "fade-in",
                    radius: radius(feature.properties.magnitude, feature.properties.depth),
                    fillColor: "#ff3b33",
                    color: "#ff3b33",
                    weight: 1,
                    fillOpacity: 0.6
                });
            }
        },
        onEachFeature: markerPopup
    });
}

function loadHistory(map, json) {
    let timelineControl = L.timelineSliderControl({
        formatOutput: function(date){
            return moment(date).format("YYYY-MM-DD HH:MM:SS");
        },
        steps: 4000,
        duration: 80000
    });

    let timeline = historyMarkers(json);

    timeline.on('change', function(e){
        updateList(e.target, false);
    });

    timeline.on('add', function() {
        timelineControl.addTo(map);
        timelineControl.addTimelines(timeline);
        updateList(timeline);
    });

    timeline.on('remove', function() {
        map.removeControl(timelineControl);
    });

    return timeline;
}

let geojson_data = {};
let current_layer = {};
let history_layer = {};
let history_control = {};

function loadData(json) {
    current_layer = loadCurrent(mymap, json);
    history_layer = loadHistory(mymap, json);

    let baseLayers = {
        "Current": current_layer,
        "History": history_layer
    };

    history_control = L.control.layers(baseLayers).addTo(mymap);
    geojson_data = json;
}

function updateData(json) {
    geojson_data.features = geojson_data.features.concat(json.features);
    updateCurrent(current_layer, geojson_data);
    history_control.removeLayer(history_layer);
    history_layer = loadHistory(mymap, geojson_data);
    history_control.addBaseLayer(history_layer, "History");
}


