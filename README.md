# *PH Quakes* #

The Philippines is in the [Ring of Fire](https://en.wikipedia.org/wiki/Ring_of_Fire) so earthquakes and
volcano eruptions are a frequent occurrence. [PHILVOLCS](https://en.wikipedia.org/wiki/Philippine_Institute_of_Volcanology_and_Seismology) keeps
track of all these and publishes earthquake data on its [website](https://www.phivolcs.dost.gov.ph/index.php/earthquake/earthquake-information3), 
and [twitter account](https://twitter.com/phivolcs_dost).

This application makes use of both sources to gather quake data and display it on a [leaflet](https://leafletjs.com/) 
map. It emulates the functionality the [USGS Earthquake Map](https://earthquake.usgs.gov/earthquakes/map//). 

## Get it and Run it ##

To run the application you will need [Rust](https://www.rust-lang.org/). Install that first. After that, run
the following:

    $ git clone https://github.com/fdeantoni/ph-quakes
    $ cd ph-quakes/server
    $ cargo run 

## License ##

`ph-quakes` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2020 Ferdinand de Antoni

## Disclaimer ##

This is not an officially supported product of PHILVOLCS-DOST and is developed purely for educational purposes.