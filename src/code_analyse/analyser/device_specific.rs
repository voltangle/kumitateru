#[allow(non_camel_case_types)]
/**
 * This file contains functions for analyzing the code
 * for functions that can cause problems on some devices,
 * like trying to call a buzzer when that buzzer does not
 * exist in some target devices.
 */

struct CodeAnalyse_DeviceSpecific {}

impl CodeAnalyse_DeviceSpecific {
    fn attention_module(code: String) {
        let split_code = code.split(" ");
        if split_code.iter().position(|&keyword| keyword == "Attention.playTone") {

        }
    }
}