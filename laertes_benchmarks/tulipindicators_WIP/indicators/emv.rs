

extern "C" {
    
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
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
pub extern "C" fn ti_emv_start(options: *const f64) -> i32 {
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn ti_emv(mut size: std::os::raw::c_int,
                                mut inputs: *const *const std::os::raw::c_double,
                                mut options: *const std::os::raw::c_double,
                                mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut volume: *const std::os::raw::c_double =
        *inputs.offset(2 as std::os::raw::c_int as isize);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if size <= ti_emv_start(options) { return 0 as std::os::raw::c_int }
    let mut last: std::os::raw::c_double =
        (*high.offset(0 as std::os::raw::c_int as isize) +
             *low.offset(0 as std::os::raw::c_int as isize)) * 0.5f64;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < size {
        let mut hl: std::os::raw::c_double =
            (*high.offset(i as isize) + *low.offset(i as isize)) * 0.5f64;
        let mut br: std::os::raw::c_double =
            *volume.offset(i as isize) / 10000.0f64 /
                (*high.offset(i as isize) - *low.offset(i as isize));
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (hl - last) / br;
        last = hl;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_emv_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_emv\x00")).as_ptr(),
                     b"indicators/emv.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 56 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_emv_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
