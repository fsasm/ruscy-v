/*
 * core.rs
 * Author: Fabjan Sukalia <fsukalia@gmail.com>
 * Date: 2016-11-04
 */

#![allow(dead_code)]

pub struct Core {
    pub reg : [u32; 32], // reg[0] is always zero
    pub pc  : u32,
}
