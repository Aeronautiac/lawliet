/*
* SYSTEM ACTION
* Change a player's role and grant them abilities associated with that role
* If a player already has the requested role, return an error
* Changing a player's role deletes any of their old "volatile" abilities
* A volatile ability is one which disappears on role change
*/

pub struct SetRoleResponse {}

#[derive(PartialEq, Eq, Clone)]
pub struct SetRole {}
