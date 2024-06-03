pub mod configuration;
pub mod routes;
pub mod startup;




// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
