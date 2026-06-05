/*
* Authoritative Action (Admin and System)
* Cull prosecutions with disallowed states.
* This is meant to run inside of the update action before polls are updated.
*/

// loop through every prosecution
// if the prosecution is in the custody or trial phase:
//   if the prosecutor or defendant has NoPresence:
//     the prosecution is terminated
//
// if the prosecution is in the voting phase:
//   if the defendant is dead:
//     the prosecution is terminated
