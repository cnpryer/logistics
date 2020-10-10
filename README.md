# logistics

Rust Logistics Engineering Library

## About

This is a logistics engineering library developed from the ground up for both software developers and logistics engineers with programming experience.

## Shipment Routing

The first target of the library is to provide an efficient method for generating optimized vehicle routes for small and highly-defined transportation optimization models.

### Scope

Models will be limited to single-origin, one-dimension capacities and demands, and optimized only using the inital distance and capacity constraints. Each iteration of this component's development will aim to expand scope little by little.

### Constraints

- _Maximum Vehicle Capacity_: The total number of units allowed in a vehicle for a given route.

- _Maximum Route Distance_: The maximum cummulative travel distance for a route.

### Inputs

- Constraint Setup
- Depot Latitude and Longitude
- Demand Latitudes and Logitudes
- Demands

## Development Strategy

This library is intended for developers and logistics engineers looking for optimization capabilities for their systematic use-cases. So the project will focus on the problem domain as each initial target and then retroactively refactored into more modular components. 

For example, the routing solution will rely on an implementation of an integer constraint solver. This solver won't be fleshed out first and then a _Shipment Routing_ component is fit to the solver. The design is reversed, forcing the solver to accommodate the business constraints prior to being completed.

This allows for more flexible solver development and, in general, a better development workflow for `logistics`. 


# Inspirations

- [ortools](https://github.com/google/or-tools)