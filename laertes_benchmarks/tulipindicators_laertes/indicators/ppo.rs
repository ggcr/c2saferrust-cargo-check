
extern "C" {
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
}
/*
 * Tulip Indicators
 * https://tulipindicators.org/
 * Copyright (c) 2010-2017 Tulip Charts LLC
 * Lewis Van Winkle (LV@tulipcharts.org)
 *
 * This file is part of Tulip Indicators.
 *
 * Tulip Indicators is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * Tulip Indicators is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Tulip Indicators.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
#[no_mangle]
pub extern "C" fn ti_ppo_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ppo(mut size: std::os::raw::c_int,
                                mut inputs: * const * const std::os::raw::c_double,
                                mut options: * const std::os::raw::c_double,
                                mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut ppo: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let short_period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let long_period: i32 =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if short_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < 2 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < short_period { return 1 as std::os::raw::c_int }
    if size <= ti_ppo_start(options) { return 0 as std::os::raw::c_int }
    let mut short_per: f64 =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (short_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut long_per: f64 =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (long_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut short_ema: f64 =
        *input.offset(0 as std::os::raw::c_int as isize);
    let mut long_ema: f64 =
        *input.offset(0 as std::os::raw::c_int as isize);
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i < size {
        short_ema =
            (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema =
            (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: f64 =
            100.0f64 * (short_ema - long_ema) / long_ema;
        let mut fresh0 = ppo;
        ppo = ppo.offset(1);
        *fresh0 = out;
        i += 1
    }
    if !(ppo.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
             as std::os::raw::c_long ==
             (size - ti_ppo_start(options)) as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 7], &'_ [i8; 7]>(b"ti_ppo\x00")).as_ptr(),
                     b"indicators/ppo.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 63 as std::os::raw::c_int,
                     b"ppo - outputs[0] == size - ti_ppo_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
