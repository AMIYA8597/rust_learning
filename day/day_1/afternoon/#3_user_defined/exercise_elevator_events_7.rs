// We will create a data structure to represent an event in an elevator control system. 
    // It is up to you to define the types and functions to construct various events. 
    // Use #[derive(Debug)] to allow the types to be formatted with {:?}.

// This exercise only requires creating and populating data structures so that main runs without errors. 
    // The next part of the course will cover getting data out of these structures.








#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    /// A button was pressed.
    ButtonPressed(Button),

    /// The car has arrived at the given floor.
    CarArrived(Floor),

    /// The car's doors have opened.
    CarDoorOpened,

    /// The car's doors have closed.
    CarDoorClosed,
}

/// A floor is represented as an integer.
type Floor = i32;

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// A user-accessible button.
#[derive(Debug)]
enum Button {
    /// A button in the elevator lobby on the given floor.
    LobbyCall(Direction, Floor),

    /// A floor button within the car.
    CarFloor(Floor),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}




















// The code you provided models an elevator system using the Rust programming language. 
    // Here's a theoretical overview of the approach:

// ### Enums and Types

// 1. **Event Enum**: Represents different events in the elevator system that the controller must react to. The variants include:
//    - `ButtonPressed(Button)`: When a button is pressed.
//    - `CarArrived(Floor)`: When the car arrives at a specific floor.
//    - `CarDoorOpened`: When the car doors have opened.
//    - `CarDoorClosed`: When the car doors have closed.

// 2. **Floor Type**: Represents a floor in the building as an integer (`i32`).

// 3. **Direction Enum**: Represents the direction of travel, either `Up` or `Down`.

// 4. **Button Enum**: Represents different types of buttons:
//    - `LobbyCall(Direction, Floor)`: A button in the elevator lobby on a specific floor, indicating the direction (up or down).
//    - `CarFloor(Floor)`: A button inside the elevator car to select a specific floor.

// ### Functions

// 1. **car_arrived(floor: i32) -> Event**: Creates an `Event` indicating that the car has arrived at a given floor.

// 2. **car_door_opened() -> Event**: Creates an `Event` indicating that the car doors have opened.

// 3. **car_door_closed() -> Event**: Creates an `Event` indicating that the car doors have closed.

// 4. **lobby_call_button_pressed(floor: i32, dir: Direction) -> Event**: Creates an `Event` for a lobby call button press 
    // on a given floor and direction.

// 5. **car_floor_button_pressed(floor: i32) -> Event**: Creates an `Event` for a floor button press inside the elevator car.

// ### Main Function

// The `main` function demonstrates how these events can be used:
// - A passenger presses the up button on the ground floor.
// - The elevator car arrives at the ground floor.
// - The car doors open.
// - A passenger presses the button for the 3rd floor.
// - The car doors close.
// - The elevator car arrives at the 3rd floor.

// ### Theoretical Approach

// 1. **Modeling Events**: Use enums to model various events and states in the elevator system, 
    // ensuring a clear and type-safe representation of all possible scenarios.

// 2. **Handling Buttons**: Differentiate between lobby buttons and car buttons to handle user inputs correctly, 
    // allowing the system to respond appropriately based on where the button was pressed.

// 3. **Function Abstraction**: Use functions to create events, 
    // encapsulating the creation logic and making the main function more readable and manageable.

// 4. **Event-Driven Simulation**: Simulate the elevator operation by creating and printing events in sequence, 
    // demonstrating how the system would react to user inputs and state changes.

// This approach ensures a clean, organized, and type-safe way to model and simulate an elevator system, 
    // allowing for easy expansion and modification in the future.