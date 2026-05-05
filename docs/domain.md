# Domain Design

## Philosophy
- PMS is a Fact system
- No business control logic
- State must be observable

## Core Concepts

### Reservation
Represents a booking fact.
It does not validate availability.

### HotelInventory
Represents aggregated inventory state.

## Rules
- Inventory can go negative
- Overselling is allowed
- Oversold amount must be observable

## Non-Responsibilities
- No availability checks
- No booking restrictions