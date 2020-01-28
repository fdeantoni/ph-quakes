let mymap = createMap();

function updateList(layer, bounded = true) {

    const displayed = layer.getLayers().sort(function(a,b) {
        const first = moment(a.feature.properties.datetime);
        const second = moment(b.feature.properties.datetime);
        return second - first;
    });
    const list = document.getElementById('displayed-list');
    list.innerHTML = "";

    displayed.forEach(function(quake){
        const layerId = layer.getLayerId(quake);
        const props = quake.feature.properties;
        const inBounds = mymap.getBounds().contains({lat: props.latitude, lng: props.longitude});
        if( !bounded || (bounded && inBounds) ) {
            const newItem = document.createElement('li');
            newItem.className = "quake-show";
            newItem.setAttribute("data-layer-id", layerId);
            newItem.innerHTML = '<div class="quake-container">' +
                '<span class="quake-magnitude">' + props.magnitude + '</span>' +
                '<h1 class="quake-location">' + props.province + '</h1>' +
                '<h2 class="quake-utc">' + props.start + '</h2>' +
                '<aside class="quake-aside">' + props.depth + ' km</aside>' +
                '</div>';

            newItem.onclick = function(e) {
                $('.list-view > li').removeClass('quake-selected');
                $(this).addClass('quake-selected');
                mymap.flyTo(quake.getLatLng(), 14);
                mymap.once('moveend', function() {
                    quake.openPopup();
                });
            };

            list.prepend(newItem);
        }
    });
}

function radius(magnitude, depth) {
    let size = Math.ceil(Math.exp(magnitude) / depth);
    if(size < 5) size = 5;
    return size
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
        duration: 80000,
        position: "bottomright"
    });

    let timeline = historyMarkers(json);

    timeline.on('change', function(e){
        updateList(e.target, false);
    });

    timelineControl.addTo(map);
    timelineControl.addTimelines(timeline);

    return timeline;
}

let history_layer = {};

function loadData(json) {
    history_layer = loadHistory(mymap, json);
    history_layer.addTo(mymap);
}




