[![](http://meritbadge.herokuapp.com/logistics)](https://crates.io/crates/logistics)

# logistics

Rust Logistics Engineering Client

## About

**CLI**:
`logistics ./file --vrp`

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
