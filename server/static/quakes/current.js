let mymap = createMap();

function updateList(layer) {

    const displayed = layer.getLayers().sort(function(a,b) {
        const first = moment(a.feature.properties.datetime);
        const second = moment(b.feature.properties.datetime);
        return second - first;
    });
    const list = document.getElementById('displayed-list');
    displayed.forEach(function(quake){
        const layerId = layer.getLayerId(quake);
        const props = quake.feature.properties;
        const newItem = document.createElement('li');
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
                let parent = quake.__parent;
                if(parent) {
                    let group = parent._group;
                    if(group && typeof group.zoomToShowLayer == 'function') {
                        console.log("Got group!");
                        group.zoomToShowLayer(quake, function() {
                            quake.openPopup();
                        });
                    } else {
                        quake.openPopup();
                    }
                } else {
                    quake.openPopup();
                }
            });
        };

        list.prepend(newItem);

        setTimeout(function() {
            newItem.className = newItem.className + "quake-show";
        }, 10);

    });
}

function markerIcon(size, text) {
    const style = "width: " + size + "px; height: " + size + "px; line-height: " + size + "px;";
    const html = '<div class="quakes-marker-icon" style="'+ style +'"><b>' + text + '</b></div>';
    return L.divIcon({ html: html, className: 'quakes-cluster', iconSize: L.point(size, size) });
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

    return cluster;
}

function appendCurrent(current, json) {
    let updates = currentMarkers(json);
    current.addLayers(updates);
    current.refreshClusters(updates);
    updateList(updates);
}

let current_layer = {};

function loadData(json) {
    current_layer = loadCurrent(mymap, json);
    current_layer.addTo(mymap);
}

function appendData(json) {
    appendCurrent(current_layer, json);
}

