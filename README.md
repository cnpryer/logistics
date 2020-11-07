[![](http://meritbadge.herokuapp.com/logistics)](https://crates.io/crates/logistics)

# logistics

Rust Logistics Engineering Client

## About

Development project to build CLI Application bundled with filesystem processing and server clients. `logistics` builds and manages projects for logistics engineering models.

- project_name
  - .logistics
  - demand
     - file1
     - file2

`.logistics`
```.logistics
origin [OPTIONAL]:
  - "origin_city"
  - "origin_state"
  - "origin_zip"
origin [OPTIONAL]:
  - origin_city: "city"
  - origin_state: "state"
  - origin_zip: "zip code"
destination [REQUIRED]:
  - "dest_city"
  - "dest_state"
  - "dest_zip"
capacity [OPTIONAL]:
  - "pallets"
  - weight: "lbs"
```

**CLI**:
`logistics demand/file1 --vrp`

**Server**:
`logistics run server --port=9999`

## Shipment Routing

The first target of the client is to provide an efficient method for generating optimized VRP solutions from csv data.

### Scope

Models will be limited to single-origin, one-dimension capacities and demands, and optimized only using the inital distance and capacity constraints.

### Constraints

- _Maximum Vehicle Capacity_: The total number of units allowed in a vehicle for a given route.

- _Maximum Route Distance_: The maximum cummulative travel distance for a route.

### Inputs

- Constraint Setup
- Depot Latitude and Longitude
- Demand Latitudes and Logitudes
- Demand Units
