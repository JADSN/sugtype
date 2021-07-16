// @ts-nocheck
JG.repeat(1, 50, {
    dtBool: JG.bool(),
    dtU8: JG.integer(0, 255),
    dtuU16: JG.integer(0, 65535),
    dtuU32: JG.integer(0, 4294967295),
    dtuU64: JG.integer(0, 18446744073709551615),
    dtuU128: JG.integer(0, 340282366920938463463374607431768211455),
    dtuUsize: JG.integer(0, 18446744073709551615),
    dtI8: JG.integer(-128, 127),
    dtI16: JG.integer(-32768, 32767),
    dtI32: JG.integer(-2147483648, 2147483647),
    dtI64: JG.integer(-9223372036854775808, 9223372036854775807),
    dtI128: JG.integer(-170141183460469231731687303715884105728, 170141183460469231731687303715884105727),
    dtIsize: JG.integer(-9223372036854775808, 9223372036854775807),
    dtF32: JG.floating(),
    dtF64: JG.floating(),
});