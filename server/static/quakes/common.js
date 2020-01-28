function createMap() {

    let map = L.map('map', {
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

    L.tileLayer(mapboxUrl, mapboxConfig).addTo(map);

    L.control.sidebar('sidebar').addTo(map);

    return map;
}

function markerPopup(feature, layer) {
    if (feature.properties) {
        const props = feature.properties;
        const header = '<h3>' + props.province + ' ' + props.longitude + ', ' + props.latitude + '</h3>';
        const details = '<ul style="list-style-type:none;padding-left: 0;">' +
            '<li><b>Magnitude: </b>' + props.magnitude + '</li>' +
            '<li><b>Depth:     </b>' + props.depth + '</li>' +
            '<li><b>Location:  </b>' + props.location + '</li>' +
            '<li><b>Source:    </b><a href="'+ props.url +'" target="_blank">phivolcs</li>' +
            '</ul>';
        layer.bindPopup(header + details);
    }
}